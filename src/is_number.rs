pub fn is_number(s: &str) -> bool {
    let st = s.trim();
    let mut chars = st.chars();
    let mut valid = false;
    let mut decimal = false;
    let mut exp = false;
    let mut start = true;
    while let Some(c) = chars.next() {
        if c.is_numeric() {
            valid = true;
            start = false;
        } else if c == '.' && !decimal && !exp {
            decimal = true;
        } else if c == 'e' && valid && !exp {
            exp = true;
            start = true;
            valid = false;
        } else if start && (c == '+' || c == '-') {
            start = false;
        } else {
            return false;
        }

    }
    valid
}

#[cfg(test)]
mod test {
    use super::is_number;

    #[test]
    fn example() {
        assert_eq!(is_number("  -359  "), true);
        assert_eq!(is_number("  - "), false);
        assert_eq!(is_number(" -359xx"), false);
        assert_eq!(is_number("  1 1  "), false);
        assert_eq!(is_number(" 0xFF  "), false);
        assert_eq!(is_number(" 1e+10e  "), false);
        assert_eq!(is_number(" 1e  "), false);
        assert_eq!(is_number(" e10  "), false);
        assert_eq!(is_number(" 1e+10  "), true);
        assert_eq!(is_number(" 1.e+10  "), true);
        assert_eq!(is_number(" .1e+10  "), true);
        assert_eq!(is_number(" .1e+1.0  "), false);
        assert_eq!(is_number(" .e+1.0  "), false);
    }
}

