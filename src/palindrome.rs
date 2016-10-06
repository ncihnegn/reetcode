pub fn is_palindrome(s: &str) -> bool {
    let mut chars = s.to_lowercase().chars().collect::<Vec<_>>();
    chars.retain(|&x| x.is_alphanumeric());
    let mut iter = chars.iter();
    let mut rev_iter = chars.iter().rev();
    while let Some(c) = iter.next() {
        if c != rev_iter.next().unwrap() {
            return false;
        }
    }
    true
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
