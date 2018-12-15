use std::cmp;

pub(crate) fn icon_by_percentage(icons: &[String], percentage: u16) -> Option<&str> {
    if icons.is_empty() {
        return None;
    }

    let length = icons.len();
    let interval = 100 / length;
    let index = cmp::min(percentage as usize, 100) / interval;

    Some(&icons[cmp::min(index, length - 1)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_by_percentage_with_no_element() {
        let icons = Vec::<String>::new();
        assert_eq!(icon_by_percentage(&icons, 0), None);
        assert_eq!(icon_by_percentage(&icons, 100), None);
    }

    #[test]
    fn test_icon_by_percentage_with_one_element() {
        let icons = vec![String::from("ICON")];
        assert_eq!(icon_by_percentage(&icons, 0), Some("ICON"));
        assert_eq!(icon_by_percentage(&icons, 50), Some("ICON"));
        assert_eq!(icon_by_percentage(&icons, 100), Some("ICON"));
        assert_eq!(icon_by_percentage(&icons, 120), Some("ICON"));
    }

    #[test]
    fn test_icon_by_percentage_with_two_elements() {
        let icons = vec![String::from("LOW"), String::from("HIGH")];
        assert_eq!(icon_by_percentage(&icons, 0), Some("LOW"));
        assert_eq!(icon_by_percentage(&icons, 49), Some("LOW"));
        assert_eq!(icon_by_percentage(&icons, 50), Some("HIGH"));
        assert_eq!(icon_by_percentage(&icons, 100), Some("HIGH"));
        assert_eq!(icon_by_percentage(&icons, 120), Some("HIGH"));
    }

    #[test]
    fn test_icon_by_percentage_with_three_elements() {
        let icons = vec![
            String::from("LOW"),
            String::from("MIDDLE"),
            String::from("HIGH"),
        ];
        assert_eq!(icon_by_percentage(&icons, 0), Some("LOW"));
        assert_eq!(icon_by_percentage(&icons, 32), Some("LOW"));
        assert_eq!(icon_by_percentage(&icons, 33), Some("MIDDLE"));
        assert_eq!(icon_by_percentage(&icons, 65), Some("MIDDLE"));
        assert_eq!(icon_by_percentage(&icons, 66), Some("HIGH"));
        assert_eq!(icon_by_percentage(&icons, 100), Some("HIGH"));
        assert_eq!(icon_by_percentage(&icons, 120), Some("HIGH"));
    }
}
