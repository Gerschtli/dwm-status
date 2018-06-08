use feature;

#[derive(Debug)]
pub struct CpuLoadData {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}

impl feature::Renderable for CpuLoadData {
    fn render(&self) -> String {
        format!("{:.2} {:.2} {:.2}", self.one, self.five, self.fifteen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feature::Renderable;

    #[test]
    fn test_display() {
        let data = CpuLoadData {
            one: 0.5,
            five: 1.52,
            fifteen: 2.1234,
        };

        assert_eq!(data.render(), "0.50 1.52 2.12");
    }
}
