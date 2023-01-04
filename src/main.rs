#![feature(drain_filter)]
#![feature(slice_group_by)]

use console_log;
use dioxus::prelude::*;
use log::{debug, Level};

pub mod partitionings;
pub mod transactions;

fn main() {
    console_log::init_with_level(Level::Debug).expect("error initialising logger");
    dioxus::web::launch(app);
}

// https://github.com/DioxusLabs/dioxus/blob/master/examples/eval.rs
fn app(cx: Scope) -> Element {
    let name = use_state(&cx, String::new);
    let dollars = use_state(&cx, String::new);
    let cents = use_state(&cx, String::new);
    // let eval = dioxus_desktop::use_eval(cx);
    // let future: &UseRef<Option<EvalResult>> = use_ref(cx, || None);
    // if future.read().is_some() {
    //     let future_clone = future.clone();
    //     cx.spawn(async move {
    //         if let Some(fut) = future_clone.with_mut(|o| o.take()) {
    //             println!("{:?}", fut.await)
    //         }
    //     });
    // }

    cx.render(rsx! {
        div {
            input {
                placeholder: "Name",
                value: "{name}",
                oninput: move |e| name.set(e.value.clone()),
            }
            input {
                placeholder: "$",
                // r#type: "number",
                // inputmode: "numeric",
                value: "{dollars}",
                oninput: move |e| {
                    let new_value = e.value.clone();
                    dollars.modify(|current| {
                        debug!("\"{}\" -> \"{}\"", current, new_value);
                        if new_value.is_empty() || new_value.parse::<i32>().is_ok() {
                            new_value
                        } else {
                            current.clone()
                        }
                    })
                },
            }
            input {
                placeholder: "00",
                // r#type: "number",
                // inputmode: "numeric",
                value: "{cents}",
                oninput: move |e| {
                    let new_value = e.value.clone();
                    cents.modify(|current| {
                        debug!("\"{}\" -> \"{}\"", current, new_value);
                        if new_value.is_empty() || (new_value.len() <= 2 && new_value.parse::<i32>().is_ok()) {
                            new_value
                        } else {
                            current.clone()
                        }
                    })
                },
            }
            // button {
            //     onclick: move |_| {
            //         let fut = eval(script);
            //         future.set(Some(fut));
            //     },
            //     "Execute"
            // }
        }
    })
}
