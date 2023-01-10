#![feature(drain_filter)]
#![feature(slice_group_by)]

pub mod balancing;
mod components;
pub mod debt;
pub mod partitionings;
mod state;

use console_log;
use log::{debug, Level};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use balancing::transact_debted_amounts_asc;
use components::debt_input::DebtInput;
use debt::Debt;
use partitionings::longest_zero_sum_partitionings;
use state::{Action, State};

fn main() {
    console_log::init_with_level(Level::Debug).expect("error initialising logger");
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let state = use_reducer(State::new);

    let onedit = {
        let state = state.clone();
        move |i: usize| {
            let state = state.clone();
            Callback::from(move |(name, value): (String, i32)| {
                state.dispatch(Action::Edit((i, name, value)))
            })
        }
    };

    let onremove = {
        let state = state.clone();
        move |i: usize| {
            let state = state.clone();
            Callback::from(move |_| state.dispatch(Action::Remove(i)))
        }
    };

    let onadd = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::Add))
    };

    let debts = use_state(Vec::<Debt>::new);
    let partitioning_transactions = {
        use_memo(
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
                            .flat_map(|partition| transact_debted_amounts_asc(&partition).unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            },
            state.entries.clone(),
        )
    };

    let debts_list_items = {
        let debts = debts.clone();
        debts
            .iter()
            .map(|debt| {
                html! {
                    <li>
                        {
                            format!(
                                "{}: {}${}.{:02}",
                                debt.name,
                                if debt.value <= 0 {""} else {"-"}, // Invert debt to display amount owed
                                (debt.value / 100).abs(),
                                (debt.value % 100).abs(),
                            )
                        }
                    </li>
                }
            })
            .collect::<Html>()
    };

    let transactions_list_items = {
        partitioning_transactions
            .iter()
            .enumerate()
            .map(|(i, transactions)| {
                html! {
                    <div>
                        <h2>{format!("Option {}", i + 1)}</h2>
                        {transactions
                            .iter()
                            .map(|transaction| {
                                html! {
                                    <p>
                                        {
                                            format!(
                                                "{} pays {} ${}.{:02}",
                                                transaction.payer,
                                                transaction.payee,
                                                transaction.value / 100,
                                                transaction.value % 100,
                                            )
                                        }
                                    </p>
                                }
                            })
                            .collect::<Html>()
                        }
                    </div>
                }
            })
            .collect::<Html>()
    };

    html! {
        <div>
            <div>
                {state
                    .entries
                    .iter()
                    .enumerate()
                    .map(|(i, entry)| {
                        html! {
                            <div key={entry.id}>
                                <DebtInput onedit={onedit(i)} />
                                <button onclick={onremove(i)}>{"X"}</button>
                            </div>
                        }
                    })
                    .collect::<Html>()
                }
            </div>
            <button onclick={onadd}>{"Add person"}</button>
            <ul>
                {state
                    .entries
                    .iter()
                    .map(|entry| {
                        html! {
                            <li>
                                {
                                    format!(
                                        "{}: {}${}.{:02}",
                                        entry.debt.name,
                                        if entry.debt.value <= 0 {""} else {"-"}, // Invert debt to display amount owed
                                        entry.debt.value.abs() / 100,
                                        entry.debt.value.abs() % 100,
                                    )
                                }
                            </li>
                        }
                    })
                    .collect::<Html>()
                }
            </ul>
            <ul>{debts_list_items}</ul>
            <div>{transactions_list_items}</div>
        </div>
    }
}
