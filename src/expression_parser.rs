use evalexpr::{eval_with_context, DefaultNumericTypes, HashMapContext, Value};
use regex::Captures;
use regex::Regex;

pub(crate) fn evaluate_expression(
    template: &str,
    vars: &HashMapContext<DefaultNumericTypes>,
) -> String {
    let re = Regex::new(r"\{([^}]+)\}").unwrap();

    re.replace_all(template, |caps: &Captures<'_>| {
        let expr = &caps[1];
        match eval_with_context(expr, vars) {
            Ok(Value::Float(f)) => format!("{:.2}", f),
            Ok(s) => s.to_string(),
            Err(_) => format!("{{{}}}", expr),
        }
    })
    .into_owned()
}

#[cfg(test)]
mod tests {
    use evalexpr::context_map;
    use hamcrest2::assert_that;
    use hamcrest2::prelude::*;

    use super::*;

    fn get_test_vars() -> HashMapContext<DefaultNumericTypes> {
        let context: HashMapContext<DefaultNumericTypes> = context_map! {
            "A" => Value::from_float(0.2),
            "B" => Value::from_float(0.4),
            "C" => Value::from_int(4),
        }
        .unwrap();

        context
    }

    #[test]
    fn evaluate_empty() {
        let vars = get_test_vars();

        let evaluated = evaluate_expression("", &vars);

        assert_that!(evaluated, is(equal_to("")));
    }

    #[test]
    fn evaluate_replace_simple() {
        let vars = get_test_vars();

        let evaluated = evaluate_expression("{A} {C} {A} test", &vars);

        assert_that!(evaluated, is(equal_to("0.20 4 0.20 test")));
    }

    #[test]
    fn evaluate_division() {
        let vars = get_test_vars();

        let evaluated = evaluate_expression("{B/C}", &vars);

        assert_that!(evaluated, is(equal_to("0.10")));
    }

    #[test]
    fn evaluate_percentage() {
        let vars = get_test_vars();

        let evaluated = evaluate_expression("{B/C*100}% {A}", &vars);

        assert_that!(evaluated, is(equal_to("10.00% 0.20")));
    }
}
