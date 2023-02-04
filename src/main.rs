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
use components::debt_input::DebtInput;
use partitionings::longest_zero_sum_partitionings;
use state::{Action, State};

fn main() {
    console_log::init_with_level(Level::Debug).expect("error initialising logger");
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(State::new);
    let transaction_partitioning_index = use_state(|| 0);

    let on_edit_entry = {
        let state = state.clone();
        move |i: usize| {
            let state = state.clone();
            Callback::from(move |(name, value): (String, i32)| {
                state.dispatch(Action::Edit((i, name, value)))
            })
        }
    };

    let on_remove_entry = {
        let state = state.clone();
        move |i: usize| {
            let state = state.clone();
            move |_| state.dispatch(Action::Remove(i))
        }
    };

    let on_add_entry = {
        let state = state.clone();
        move |_| state.dispatch(Action::Add)
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
        move |_| {
            if !transactions.is_empty() {
                let clipboard = window()
                    .and_then(|window| window.navigator().clipboard())
                    .expect("Cannot access clipboard API");
                clipboard.write_text(
                    &(transactions
                        .iter()
                        .map(|transaction| transaction.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")),
                );
            }
        }
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
                <div class="entries">
                    <label class="name-label">{"Name"}</label>
                    <div></div>
                    <label class="value-label">{"Amount owed"}</label>
                    <div></div>
                    {state
                        .entries
                        .iter()
                        .enumerate()
                        .map(|(i, entry)| {
                            html! {
                                <div class="entry" key={entry.id}>
                                    <DebtInput id={entry.id} onedit={on_edit_entry(i)} />
                                    <button
                                        onclick={on_remove_entry(i)}
                                        tabindex="0"
                                    >
                                        {"☓"}
                                    </button>
                                </div>
                            }
                        })
                        .collect::<Html>()
                    }
                    <button
                        class="add-entry"
                        onclick={on_add_entry}
                        tabindex="2"
                    >
                        {"Add person"}
                    </button>
                </div>
                {html! {
                    if !transactions.is_empty() {
                        <div class="output-actions">
                            <button
                                class="output-actions--copy"
                                onclick={on_copy_transactions}
                            >
                                {"Copy"}
                            </button>
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
