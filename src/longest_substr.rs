pub fn longest_substr(s: &str) -> String {//No repeating character
    let mut work = Vec::new();
    let mut cand = work.clone();
    for c in s.chars() {
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
    let mut work = Vec::new();
    let mut cand = work.clone();
    for c in s.chars() {
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

pub fn longest_substr_k(s: &str, k: usize) -> String {
    //At most K distinct characters
    use std::collections::HashMap;
    use std::cmp;

    let mut work = Vec::new();
    let mut cand = work.clone();
    let mut m = HashMap::<char, usize>::new();
    if k > 0 {
        for c in s.chars() {
            if !work.iter().any(|&x| x == c) && m.len() == k {
                let min_index = m.iter().fold(work.len(), |min_i, (&_, &ind)|
                                              cmp::min(min_i, ind));
                m.remove(&work[min_index]);
                assert!(m.len() == k-1);
                for (_, ind) in m.iter_mut() {
                    assert!(*ind > min_index);
                    *ind -= min_index+1;
                }
                work = work.split_off(min_index+1);
            }
            m.insert(c, work.len());
            work.push(c);
            //println!("\n{}", work.clone().into_iter().collect::<String>());
            if work.len() > cand.len() {
                cand = work.clone();
            }
        }
    }
    cand.into_iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::longest_substr;
    use super::longest_substr_two;
    use super::longest_substr_k;

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

        assert_eq!(&longest_substr_k("ecebabbec", 2), "babb");
        assert_eq!(&longest_substr_k("ecebabbec", 3), "ebabbe");
    }
}
