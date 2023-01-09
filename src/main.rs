#![feature(drain_filter)]
#![feature(slice_group_by)]

mod components;
pub mod partitionings;
mod state;
pub mod transactions;

use console_log;
use log::{debug, Level};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use components::debt_input::DebtInput;
use partitionings::{longest_zero_sum_partitionings, Debt};
use state::{Action, State};
use transactions::pay_credited;

fn main() {
    console_log::init_with_level(Level::Debug).expect("error initialising logger");
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let state = use_reducer(|| State {
        debts: vec![Debt::new()],
    });

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
            |debts| {
                longest_zero_sum_partitionings(debts)
                    .into_iter()
                    .map(|partitioning| {
                        partitioning
                            .into_iter()
                            .flat_map(|partition| pay_credited(&partition).unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            },
            state.debts.clone(),
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
                {
                    (0..(state.debts.len()))
                        .map(|i| {
                            html! {
                                <div key={i}>
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
                {
                    state.debts
                        .iter()
                        .map(|debt| {
                            html! {
                                <li>
                                    {
                                        format!(
                                            "{}: {}${}.{:02}",
                                            debt.name,
                                            if debt.value <= 0 {""} else {"-"}, // Invert debt to display amount owed
                                            debt.value.abs() / 100,
                                            debt.value.abs() % 100,
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
