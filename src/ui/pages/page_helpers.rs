use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    let text_len = text.len();
    if text_len <= width {
        // need to pad the truncated string with spaces or UI will get wonky
        let pads = width - text_len;
        let mut padded = text.to_owned();
        for _ in 0..pads {
            padded.push(' ');
        }
        return padded;
    }
    // truncate_ellipse() trims to specified length and always appends ... so result will be
    // too long for width values < 4
    match width {
        0 => "".to_string(),
        1 => ".".to_string(),
        2 => "..".to_string(),
        3 => "...".to_string(),
        _ => text.truncate_ellipse(width - 3).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;
        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;
        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;
        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;
        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;
        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}
