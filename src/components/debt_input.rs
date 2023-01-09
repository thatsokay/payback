use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DebtInputProps {
    pub onedit: Callback<(String, i32)>,
}

#[function_component(DebtInput)]
pub fn debt_input(props: &DebtInputProps) -> Html {
    let name = use_state(String::new);
    let value = use_state(String::new);

    let name_onchange = {
        let name = name.clone();
        move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        }
    };

    let value_onchange = {
        let value = value.clone();
        move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            value.set(input.value());
        }
    };

    let onsubmit = {
        let name = name.clone();
        let value = value.clone();
        let onedit = props.onedit.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            if let Ok(parsed) = value.parse::<f64>() {
                let owed_value = (parsed * 100.0).round() as i32;
                value.set(format!(
                    "{}{}.{:02}",
                    if owed_value < 0 { "-" } else { "" },
                    owed_value.abs() / 100,
                    owed_value.abs() % 100
                ));
                onedit.emit((name.to_string(), -owed_value));
            }
        }
    };

    let onblur = {
        let name = name.clone();
        let value = value.clone();
        let onedit = props.onedit.clone();
        Callback::from(move |_: FocusEvent| {
            if let Ok(parsed) = value.parse::<f64>() {
                let owed_value = (parsed * 100.0).round() as i32;
                value.set(format!(
                    "{}{}.{:02}",
                    if owed_value < 0 { "-" } else { "" },
                    owed_value.abs() / 100,
                    owed_value.abs() % 100
                ));
                onedit.emit((name.to_string(), -owed_value));
            }
        })
    };

    html! {
        <form {onsubmit}>
            <input
                placeholder="Name"
                type="text"
                value={name.to_string()}
                onchange={name_onchange}
                onblur={onblur.clone()}
            />
            {"$"}
            <input
                placeholder="Value"
                inputmode="decimal"
                value={value.to_string()}
                onchange={value_onchange}
                onblur={onblur.clone()}
            />
            <button hidden={true} /> // Hidden button to allow form submit.
        </form>
    }
}
