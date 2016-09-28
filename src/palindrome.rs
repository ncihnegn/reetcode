pub fn is_palindrome(s: &str) -> bool {
    let mut chars = s.to_lowercase().chars().collect::<Vec<_>>();
    chars.retain(|&x| x.is_alphanumeric());
    let orig = chars.clone().into_iter().collect::<String>();
    let rev = chars.into_iter().rev().collect::<String>();
    orig == rev
}

#[cfg(test)]
mod test {
    use super::is_palindrome;

    #[test]
    fn example() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama"), true);
        assert_eq!(is_palindrome("race a car"), false);
    }
}
