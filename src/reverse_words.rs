pub fn reverse_words(s: &str) -> String {
    let sstr = s.to_string();
    let rws = sstr.split_whitespace();
    let mut r = "".to_string();
    for sub in rws {
        r = r + &sub.chars().rev().collect::<String>() + " ";
    }
    let len = r.len()-1;
    r.truncate(len);
    r
}

#[cfg(test)]
mod test {
    use super::reverse_words;
    #[test]
    fn example() {
        assert_eq!(reverse_words("test"), "tset".to_string());
        assert_eq!(reverse_words("test t"), "tset t".to_string());
    }
}
