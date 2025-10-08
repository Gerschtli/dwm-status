use crate::feature::Renderable;
use crate::expression_parser::*;
use evalexpr::*;

#[derive(Debug)]
pub(super) struct Data {
    cache: String,
    template: String,
}

impl Data {
    pub(super) const fn new(template: String) -> Self {
        Self {
            cache: String::new(),
            template,
        }
    }

    pub(super) fn update(&mut self, one: f32, five: f32, fifteen: f32, nproc: u32) {
        let context : HashMapContext<DefaultNumericTypes> = context_map! {
            "CL1" => Value::from_float(one.into()),
            "CL5" => Value::from_float(five.into()),
            "CL15" => Value::from_float(fifteen.into()),
            "NPROC" => Value::from_int(nproc.into()),
        }.unwrap();

        self.cache = evaluate_expression(&self.template, &context);
    }
}

impl Renderable for Data {
    fn render(&self) -> &str {
        &self.cache
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::assert_that;
    use hamcrest2::prelude::*;

    use super::*;

    #[test]
    fn render_with_default() {
        let object = Data::new("{CL1} {CL5} {CL15}".to_owned());

        assert_that!(object.render(), is(equal_to("")));
    }

    #[test]
    fn render_with_update() {
        let mut object = Data::new("{CL1} {CL5} {CL15}".to_owned());

        object.update(20.1234, 0.005, 5.3, 2);

        assert_that!(object.render(), is(equal_to("20.12 0.00 5.30")));
    }

    #[test]
    fn render_with_update_and_missing_placeholder() {
        let mut object = Data::new("{CL1} - {CL15}".to_owned());

        object.update(20.1234, 0.005, 5.3, 2);

        assert_that!(object.render(), is(equal_to("20.12 - 5.30")));
    }

    #[test]
    fn render_with_update_and_math() {
        let mut object = Data::new("{CL1/NPROC*100}% {CL5} {CL15}".to_owned());

        object.update(20.1234, 0.005, 5.3, 2);

        assert_that!(object.render(), is(equal_to("1006.17% 0.00 5.30")));
    }
}
