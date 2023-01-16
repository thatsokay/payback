use std::num::ParseFloatError;
use web_sys::HtmlInputElement;
use yew::prelude::*;

fn parse_and_format_dollar_value(dollars: &str) -> Result<(i32, String), ParseFloatError> {
    let parsed = dollars.parse::<f64>()?;
    let cents = (parsed * 100.0).round() as i32;
    let formatted = format!(
        "{}{}.{:02}",
        if cents < 0 { "-" } else { "" },
        cents.abs() / 100,
        cents.abs() % 100
    );
    Ok((cents, formatted))
}

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
            if let Ok((owed_cents, formatted)) = parse_and_format_dollar_value(&value) {
                value.set(formatted);
                onedit.emit((name.to_string(), -owed_cents));
            }
        }
    };

    let onblur = {
        let name = name.clone();
        let value = value.clone();
        let onedit = props.onedit.clone();
        Callback::from(move |_: FocusEvent| {
            if let Ok((owed_cents, formatted)) = parse_and_format_dollar_value(&value) {
                value.set(formatted);
                onedit.emit((name.to_string(), -owed_cents));
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

mod tests {
    use super::*;

    #[test]
    fn test_parse_and_format_dollar_value() {
        assert_eq!(
            parse_and_format_dollar_value("9.99").unwrap(),
            (999_i32, String::from("9.99"))
        );
    }

    #[test]
    fn test_parse_and_format_dollar_value_signed() {
        assert_eq!(
            parse_and_format_dollar_value("+9.99").unwrap(),
            (999_i32, String::from("9.99"))
        );
        assert_eq!(
            parse_and_format_dollar_value("-9.99").unwrap(),
            (-999_i32, String::from("-9.99"))
        );
        assert_eq!(
            parse_and_format_dollar_value("-0.01").unwrap(),
            (-1_i32, String::from("-0.01"))
        );
    }

    #[test]
    fn test_parse_and_format_dollar_value_rounding() {
        assert_eq!(
            parse_and_format_dollar_value("9.9901").unwrap(),
            (999_i32, String::from("9.99"))
        );
        assert_eq!(
            parse_and_format_dollar_value("9.9999").unwrap(),
            (1000_i32, String::from("10.00"))
        );
    }

    #[test]
    fn test_parse_and_format_dollar_value_truncated() {
        assert_eq!(
            parse_and_format_dollar_value("9").unwrap(),
            (900_i32, String::from("9.00"))
        );
        assert_eq!(
            parse_and_format_dollar_value("9.").unwrap(),
            (900_i32, String::from("9.00"))
        );
        assert_eq!(
            parse_and_format_dollar_value("9.0").unwrap(),
            (900_i32, String::from("9.00"))
        );
        assert_eq!(
            parse_and_format_dollar_value(".9").unwrap(),
            (90_i32, String::from("0.90"))
        );
    }
}
