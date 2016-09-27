pub fn longest_substr(s: &str) -> String {
    let v = s.chars().collect::<Vec<char>>();
    let mut cand = Vec::new();
    let mut work = Vec::new();
    for c in v {
        let p = work.iter().position(|&x| x == c);
        match p {
            None => work.push(c),
            Some(index) => {
                if work.len() > cand.len() {
                    cand = work.clone();
                }
                work.push(c);
                for i in 0..index+1 {
                    work.remove(i);
                }
            }
        }
    }
    cand.into_iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::longest_substr;

    #[test]
    fn example() {
        assert_eq!(&longest_substr("abcabcbb"), "abc");
        assert_eq!(&longest_substr("bbbbb"), "b");
        assert_eq!(&longest_substr("abcabcdbb"), "abcd");
    }
}
