pub fn longest_substr(s: &str) -> String {//No repeating character
    let v = s.chars().collect::<Vec<char>>();
    let mut work = Vec::new();
    let mut cand = work.clone();
    for c in v {
        let p = work.iter().position(|&x| x == c);
        if let Some(index) = p {
            work = work.split_off(index+1);
        }
        work.push(c);
        if work.len() > cand.len() {
            cand = work.clone();
        }
    }
    cand.into_iter().collect::<String>()
}

pub fn longest_substr_two(s: &str) -> String {//At most two distinct characters
    let limit = 2;
    let v = s.chars().collect::<Vec<char>>();
    let mut work = Vec::new();
    let mut cand = work.clone();
    for c in v {
        if work.len() >= limit && !work.iter().all(|&x| x == work[0]) {
            if !work.iter().any(|&x| x == c) {
                let index = work.iter().rposition(|&x| x == work[0]).unwrap();
                work = work.split_off(index+1);
            }
        }
        work.push(c);
        if work.len() > cand.len() {
            cand = work.clone();
        }
    }
    cand.into_iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::longest_substr;
    use super::longest_substr_two;

    #[test]
    fn example() {
        assert_eq!(&longest_substr(""), "");
        assert_eq!(&longest_substr("a"), "a");
        assert_eq!(&longest_substr("abcabcbb"), "abc");
        assert_eq!(&longest_substr("abcabcdbb"), "abcd");
        assert_eq!(&longest_substr("bbbbb"), "b");
        assert_eq!(&longest_substr("abcabcdbb"), "abcd");

        assert_eq!(&longest_substr_two("eceba"), "ece");
        assert_eq!(&longest_substr_two("ec"), "ec");
        assert_eq!(&longest_substr_two("eec"), "eec");
        assert_eq!(&longest_substr_two("e"), "e");
        assert_eq!(&longest_substr_two(""), "");
        assert_eq!(&longest_substr_two("ecebabb"), "babb");
        assert_eq!(&longest_substr_two("ecebabbec"), "babb");
    }
}
