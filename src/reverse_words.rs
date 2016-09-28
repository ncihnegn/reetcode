pub fn reverse_words(s: &str) -> String {
    let rws = s.split_whitespace();
    let mut r = "".to_string();
    for sub in rws.rev() {
        r = r + sub + " ";
    }
    r.trim().to_string()
}

fn reverse_vec_inplace(v: &mut Vec<char>, begin: usize, end: usize) {
    let mut forward = begin;
    let mut backward = end;
    while forward < backward {
        v.swap(forward, backward);
        forward += 1;
        backward -= 1;
    }
}

pub fn reverse_words_inplace(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut begin = 0;
    let mut end = if chars.is_empty() { 0 } else { chars.len()-1 };
    reverse_vec_inplace(&mut chars, begin, end);
    begin = 0;
    for i in 0..chars.len()+1 {
        if i == chars.len() || chars[i] == ' ' {
            let next_begin = i + 1;
            end = if i > 0 { i-1 } else { 0 };
            reverse_vec_inplace(&mut chars, begin, end);
            begin = next_begin;
        }
    }
    chars.into_iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::reverse_words;
    use super::reverse_words_inplace;
    #[test]
    fn example() {
        assert_eq!(&reverse_words("test"), "test");
        assert_eq!(&reverse_words("test t"), "t test");
        assert_eq!(&reverse_words(""), "");
        assert_eq!(&reverse_words(" test "), "test");
        assert_eq!(&reverse_words(" test  t "), "t test");

        assert_eq!(&reverse_words_inplace("test"), "test");
        assert_eq!(&reverse_words_inplace("test t"), "t test");
        assert_eq!(&reverse_words_inplace(""), "");
        assert_eq!(&reverse_words_inplace(" test "), " test ");
        assert_eq!(&reverse_words_inplace(" test  t "), " t  test ");
    }
}
