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
    let name_input_ref = use_node_ref();
    let value_input_ref = use_node_ref();

    let onsubmit = {
        let name_input_ref = name_input_ref.clone();
        let value_input_ref = value_input_ref.clone();
        let onedit = props.onedit.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            let value_input = value_input_ref.cast::<HtmlInputElement>().unwrap();
            let value = value_input.value();
            if let Ok((owed_cents, formatted)) = parse_and_format_dollar_value(&value) {
                let name = name_input_ref.cast::<HtmlInputElement>().unwrap().value();
                value_input.set_value(&formatted);
                onedit.emit((name, -owed_cents));
            }
        }
    };

    let onblur = {
        let name_input_ref = name_input_ref.clone();
        let value_input_ref = value_input_ref.clone();
        let onedit = props.onedit.clone();
        Callback::from(move |_: FocusEvent| {
            let value_input = value_input_ref.cast::<HtmlInputElement>().unwrap();
            let value = value_input.value();
            if let Ok((owed_cents, formatted)) = parse_and_format_dollar_value(&value) {
                let name = name_input_ref.cast::<HtmlInputElement>().unwrap().value();
                value_input.set_value(&formatted);
                onedit.emit((name, -owed_cents));
            }
        })
    };

    html! {
        <form class="debt-input" {onsubmit}>
            <input
                class="debt-input--name"
                ref={name_input_ref}
                placeholder="Name"
                autocapitalize="on"
                onblur={onblur.clone()}
            />
            <span>{"$"}</span>
            <input
                class="debt-input--value"
                ref={value_input_ref}
                placeholder="0.00"
                inputmode="decimal"
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
