use std::i32;

pub fn atoi(s: &str) -> Option<i32> {
    enum Sign {
        Positive,
        Negative,
    }
    let mut sign = Sign::Positive;
    let st = s.trim_left();
    let mut chars = st.chars();
    if st.starts_with('-') {
        sign = Sign::Negative;
        chars.next();
    }
    if st.starts_with('+') {
        chars.next();
    }
    let mut val: Option<i32> = None;
    while let Some(c) = chars.next() {
        if c.is_numeric() {
            let (mul, overflow_mul) = val.unwrap_or(0).overflowing_mul(10);
            let (add, overflow_add) = mul.overflowing_add(c.to_digit(10).unwrap() as i32);
            if overflow_mul || overflow_add {
                match sign {
                    Sign::Positive => return Some(i32::MAX),
                    Sign::Negative => return Some(i32::MIN)
                }
            }
            val = Some(add);
        } else {
            break;
        }
    }
    match sign {
        Sign::Positive => val,
        Sign::Negative => val.map(|v| -v)
    }
}

#[cfg(test)]
mod test {
    use std::i32;
    use super::atoi;

    #[test]
    fn example() {
        assert_eq!(atoi("  359xx"), Some(359));
        assert_eq!(atoi(" -359xx33"), Some(-359));
        assert_eq!(atoi(" -xx33"), None);
        assert_eq!(atoi(" -+33"), None);
        assert_eq!(atoi(" -9999999999999999999999999999999"), Some(i32::MIN));
        assert_eq!(atoi(" +9999999999999999999999999999999"), Some(i32::MAX));
    }
}

