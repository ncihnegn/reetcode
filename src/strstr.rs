pub fn strstr(haystack: &str, needle: &str) -> Option<usize> {
    //Boyer-Moore-Horspool algorithm
    if needle.len() == 0 {
        return Some(0);
    }
    let mut skip = 0;
    let hbytes = String::from(haystack).into_bytes();
    let nbytes = String::from(needle).into_bytes();
    let nlen = nbytes.len();
    let mut ntruncated = nbytes.clone();
    ntruncated.truncate(nlen-1);
    while skip + nlen <= hbytes.len() {
        for i in 0..nlen {
            if nbytes[i] != hbytes[skip + i] {
                break;
            }
            if i == nlen-1 {
                return Some(skip);
            }
        }
        match ntruncated.iter().rposition(|x| *x == hbytes[skip + nbytes.len()-1]) {
            Some(j) => skip = skip + nlen-1 - j,
            None => skip = skip + nlen,
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::strstr;
    #[test]
    fn example() {
        assert_eq!(strstr("test", ""), Some(0));
        assert_eq!(strstr("test", "testtest"), None);
        assert_eq!(strstr("aaaba", "ba"), Some(3));
        assert_eq!(strstr("mississippi", "issi"), Some(1));
        assert_eq!(strstr("aaaaaaaaaaa", "aaaaaaaaaab"), None);
    }
}
