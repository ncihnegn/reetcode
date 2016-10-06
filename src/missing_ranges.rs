pub fn missing_ranges(slice: &[i32]) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    let end = slice.to_vec().iter().fold(0i32, |s, i| {
        if *i == s+1 {
            v.push(s.to_string());
        } else if *i != s {
            v.push(s.to_string() + "-" + &(*i-1).to_string());
        }
        i+1
    });

    let max:i32 = 99;

    if end == max {
        v.push(max.to_string());
    }
    if end < max {
        v.push(end.to_string() + "-" + &max.to_string());
    }

    v
}

#[cfg(test)]
mod test {
    use super::missing_ranges;

    #[test]
    fn example() {
        assert_eq!(missing_ranges(&[0, 1,3, 50, 75]), vec!("2", "4-49", "51-74", "76-99"));
    }
}

