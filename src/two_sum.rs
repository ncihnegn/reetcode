pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len()-1 {
        let s = target - nums[i];
        for j in i+1..nums.len() {
            if nums[j] == s {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len()-1 {
        let s = target - nums[i];
        let remains = &nums[i+1..nums.len()];
        let r = remains.binary_search(&s);
        if r.is_ok() {
            return Some((i, i + 1 + r.unwrap()));
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::two_sum;
    use super::two_sum_sorted;
    #[test]
    fn example() {
        let nums = [2, 7, 11, 15];

        assert_eq!(two_sum(&nums, 9), Some((0, 1)));
        assert_eq!(two_sum(&nums, 13), Some((0, 2)));
        assert_eq!(two_sum(&nums, 17), Some((0, 3)));
        assert_eq!(two_sum(&nums, 18), Some((1, 2)));
        assert_eq!(two_sum(&nums, 22), Some((1, 3)));
        assert_eq!(two_sum(&nums, 26), Some((2, 3)));
        assert_eq!(two_sum(&nums, 4), None);

        assert_eq!(two_sum_sorted(&nums, 9), Some((0, 1)));
        assert_eq!(two_sum_sorted(&nums, 13), Some((0, 2)));
        assert_eq!(two_sum_sorted(&nums, 17), Some((0, 3)));
        assert_eq!(two_sum_sorted(&nums, 18), Some((1, 2)));
        assert_eq!(two_sum_sorted(&nums, 22), Some((1, 3)));
        assert_eq!(two_sum_sorted(&nums, 26), Some((2, 3)));
        assert_eq!(two_sum_sorted(&nums, 4), None);

    }
}
