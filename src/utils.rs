use std::cmp;

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
#[cfg_attr(all(test, feature = "mocking"), mocktopus::macros::mockable)]
pub(crate) fn icon_by_percentage<I: Into<f64>>(icons: &[String], percentage: I) -> Option<&str> {
    if icons.is_empty() {
        return None;
    }

    let length = icons.len();
    let interval = 100 / length;
    let index = cmp::min(percentage.into() as usize, 100) / interval;

    Some(&icons[cmp::min(index, length - 1)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_by_percentage_with_no_element() {
        let icons = Vec::<String>::new();
        assert_eq!(icon_by_percentage(&icons, 0), None);
        assert_eq!(icon_by_percentage(&icons, 100), None);
    }

    #[test]
    fn icon_by_percentage_with_one_element() {
        let icons = vec!["ICON".to_owned()];
        assert_eq!(icon_by_percentage(&icons, 0), Some("ICON"));
        assert_eq!(icon_by_percentage(&icons, 50), Some("ICON"));
        assert_eq!(icon_by_percentage(&icons, 100), Some("ICON"));
        assert_eq!(icon_by_percentage(&icons, 120), Some("ICON"));
    }

    #[test]
    fn icon_by_percentage_with_two_elements() {
        let icons = vec!["LOW".to_owned(), "HIGH".to_owned()];
        assert_eq!(icon_by_percentage(&icons, 0), Some("LOW"));
        assert_eq!(icon_by_percentage(&icons, 49), Some("LOW"));
        assert_eq!(icon_by_percentage(&icons, 50), Some("HIGH"));
        assert_eq!(icon_by_percentage(&icons, 100), Some("HIGH"));
        assert_eq!(icon_by_percentage(&icons, 120), Some("HIGH"));
    }

    #[test]
    fn icon_by_percentage_with_three_elements() {
        let icons = vec!["LOW".to_owned(), "MIDDLE".to_owned(), "HIGH".to_owned()];
        assert_eq!(icon_by_percentage(&icons, 0), Some("LOW"));
        assert_eq!(icon_by_percentage(&icons, 32), Some("LOW"));
        assert_eq!(icon_by_percentage(&icons, 33), Some("MIDDLE"));
        assert_eq!(icon_by_percentage(&icons, 65), Some("MIDDLE"));
        assert_eq!(icon_by_percentage(&icons, 66), Some("HIGH"));
        assert_eq!(icon_by_percentage(&icons, 100), Some("HIGH"));
        assert_eq!(icon_by_percentage(&icons, 120), Some("HIGH"));
    }
}
