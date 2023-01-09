#![feature(drain_filter)]
#![feature(slice_group_by)]

pub mod partitionings;
pub mod transactions;

use console_log;
use log::{debug, Level};
use partitionings::*;
use transactions::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

fn main() {
    console_log::init_with_level(Level::Debug).expect("error initialising logger");
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let debts = use_state(Vec::<Debt>::new);
    let partitioning_transactions = {
        let debts = debts.clone();
        use_memo(
            |debts| {
                longest_zero_sum_partitionings(debts)
                    .into_iter()
                    .map(|partitioning| {
                        partitioning
                            .into_iter()
                            .flat_map(|partition| pay_credited(&partition))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            },
            debts,
        )
    };

    let debt_name = use_state(String::new);
    let debt_value = use_state(String::new);

    let name_oninput = {
        let debt_name = debt_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            debt_name.set(value);
        })
    };

    let value_oninput = {
        let debt_value = debt_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            let valid = value.is_empty()
                || value
                    .chars()
                    .all(|c| c.is_numeric() || c == '-' || c == '.');
            if valid {
                debt_value.set(value);
            } else {
                debt_value.set(debt_value.to_string());
            }
        })
    };

    let onsubmit = {
        let debts = debts.clone();
        let debt_name = debt_name.clone();
        let debt_value = debt_value.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let value = (*debt_value)
                .parse::<f64>()
                .map(|float| -(float * 100.0).round() as i32);
            if value.is_err() {
                return;
            }
            let mut new_debts = (*debts).clone();
            new_debts.push(Debt {
                name: debt_name.to_string(),
                value: value.unwrap(),
            });
            debts.set(new_debts);
            debt_name.set(String::new());
            debt_value.set(String::new());
        })
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
            <form
                {onsubmit}
            >
                <input
                    placeholder="Name"
                    value={debt_name.to_string()}
                    oninput={name_oninput}
                />
                <input
                    placeholder="Balance"
                    value={debt_value.to_string()}
                    oninput={value_oninput}
                    inputmode="decimal"
                />
                <button>
                    {"Add debt"}
                </button>
            </form>
            <ul>{debts_list_items}</ul>
            <div>{transactions_list_items}</div>
        </div>
    }
}
