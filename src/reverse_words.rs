pub fn reverse_words(s: &str) -> String {
    let rws = s.split_whitespace();
    let mut r = "".to_string();
    for sub in rws.rev() {
        r = r + sub + " ";
    }
    r.trim().to_string()
}

pub fn reverse_words_inplace(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut index_forward = 0;
    let mut index_backward = if chars.is_empty() { 0 } else { chars.len()-1 };
    while index_forward < index_backward {
        chars.swap(index_forward, index_backward);
        index_forward = index_forward+1;
        index_backward = index_backward-1;
    }
    index_forward = 0;
    for i in 0..chars.len()+1 {
        if i == chars.len() || chars[i] == ' ' {
            let next_begin = i + 1;
            index_backward = if i > 0 { i-1 } else { 0 };
            while index_forward < index_backward {
                chars.swap(index_forward, index_backward);
                index_forward = index_forward+1;
                index_backward = index_backward-1;
            }
            index_forward = next_begin;
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
