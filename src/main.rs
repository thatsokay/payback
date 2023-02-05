#![feature(drain_filter)]
#![feature(slice_group_by)]

pub mod balancing;
mod components;
pub mod debt;
pub mod partitionings;
mod state;

use console_log;
use log::Level;
use std::rc::Rc;
use web_sys::window;
use yew::prelude::*;

use balancing::balance_by_debted_amounts_asc;
use components::entries::Entries;
use partitionings::longest_zero_sum_partitionings;
use state::State;

fn main() {
    console_log::init_with_level(Level::Debug).expect("error initialising logger");
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(State::new);
    let transaction_partitioning_index = use_state(|| 0);
    let show_help_text = use_state(|| false);

    let on_toggle_help_text = {
        let show_help_text = show_help_text.clone();
        move |_| show_help_text.set(!*show_help_text)
    };

    let transaction_partitionings = use_memo(
        |entries| {
            let debts: Vec<_> = entries
                .clone()
                .into_iter()
                .map(|entry| entry.debt)
                .collect();
            longest_zero_sum_partitionings(&debts)
                .into_iter()
                .map(|partitioning| {
                    partitioning
                        .into_iter()
                        .flat_map(|partition| balance_by_debted_amounts_asc(&partition))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        },
        state.entries.clone(),
    );

    let transactions = transaction_partitionings
        .get(*transaction_partitioning_index)
        .unwrap_or(&vec![])
        .clone();

    {
        let transaction_partitioning_index = transaction_partitioning_index.clone();
        let transaction_partitionings = Rc::clone(&transaction_partitionings);
        use_effect_with_deps(
            move |_| transaction_partitioning_index.set(0),
            transaction_partitionings,
        );
    }

    let on_copy_transactions = {
        let transactions = transactions.clone();
        window()
            .and_then(|window| window.navigator().clipboard())
            .map(|clipboard| {
                move |_| {
                    if !transactions.is_empty() {
                        clipboard.write_text(
                            &(transactions
                                .iter()
                                .map(|transaction| transaction.to_string())
                                .collect::<Vec<_>>()
                                .join("\n")),
                        );
                    }
                }
            })
    };

    let on_decrement_transaction_partitioning_index = {
        let transaction_partitioning_index = transaction_partitioning_index.clone();
        move |_| {
            if *transaction_partitioning_index > 0 {
                transaction_partitioning_index.set(*transaction_partitioning_index - 1);
            }
        }
    };

    let on_increment_transaction_partitioning_index = {
        let transaction_partitioning_index = transaction_partitioning_index.clone();
        let partitionings_len = transaction_partitionings.len();
        move |_| {
            if *transaction_partitioning_index < partitionings_len - 1 {
                transaction_partitioning_index.set(*transaction_partitioning_index + 1);
            }
        }
    };

    html! {
        <div class="margin">
            <div class="content">
                <div class="header">
                    <h1 class="title">{"Payback"}</h1>
                    <a
                        class="help"
                        href="javascript:void(0)"
                        onclick={on_toggle_help_text}
                    >
                        {"Help "}{if *show_help_text { "▴" } else { "▾" }}
                    </a>
                </div>
                {html! {
                    if *show_help_text {
                        <div class="help-text">
                            <p>
                                {
                                    "Enter everyone's names and the amount \
                                    they are owed (positive if they are owed \
                                    money, negative if they owe money)."
                                }
                            </p>
                            <p>
                                {
                                    "Once all the debts sum to zero, the \
                                    transactions required to settle everyone's \
                                    debts will be displayed below."
                                }
                            </p>
                        </div>
                    }
                }}
                <Entries {state} />
                {html! {
                    if !transactions.is_empty() {
                        <div class="output-actions">
                            {match on_copy_transactions {
                                Some(onclick) => html! {
                                    <button
                                        class="output-actions--copy"
                                        {onclick}
                                    >
                                        {"Copy"}
                                    </button>
                                },
                                None => html! { <div></div> }
                            }}
                            if transaction_partitionings.len() > 1 {
                                <div class="output-actions--pagination">
                                    <button
                                        onclick={on_decrement_transaction_partitioning_index}
                                    >
                                        {"<"}
                                    </button>
                                    <div>
                                        {format!(
                                            "{}/{}",
                                            *transaction_partitioning_index + 1,
                                            transaction_partitionings.len()
                                        )}
                                    </div>
                                    <button
                                        onclick={on_increment_transaction_partitioning_index}
                                    >
                                        {">"}
                                    </button>
                                </div>
                            }
                        </div>
                        <div class="transactions">
                            {transactions
                                .iter()
                                .map(|transaction| {
                                    html! {
                                        <div class="transaction">
                                            {transaction}
                                        </div>
                                    }
                                })
                                .collect::<Html>()
                            }
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
