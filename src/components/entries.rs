use yew::prelude::*;

use crate::components::debt_form::DebtForm;
use crate::state::{Action, State};

#[derive(Clone, PartialEq, Properties)]
pub struct EntriesProps {
    pub state: UseReducerHandle<State>,
}

/// List of form items for manipulating the state entries
#[function_component(Entries)]
pub fn entries(props: &EntriesProps) -> Html {
    let on_edit_entry = {
        let state = props.state.clone();
        move |i: usize| {
            let state = state.clone();
            Callback::from(move |(name, value): (String, i32)| {
                state.dispatch(Action::Edit((i, name, value)))
            })
        }
    };

    let on_remove_entry = {
        let state = props.state.clone();
        move |i: usize| {
            let state = state.clone();
            move |_| state.dispatch(Action::Remove(i))
        }
    };

    let on_add_entry = {
        let state = props.state.clone();
        move |_| state.dispatch(Action::Add)
    };

    html! {
        <div class="entries">
            <label class="name-label">{"Name"}</label>
            <div></div>
            <label class="value-label">{"Amount owed"}</label>
            <div></div>
            {props
                .state
                .entries
                .iter()
                .enumerate()
                .map(|(i, entry)| {
                    html! {
                        <div class="entry" key={entry.id}>
                            <DebtForm id={entry.id} onedit={on_edit_entry(i)} />
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
    }
}
