use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        let mut occ: HashSet<char> = HashSet::new();

        let n = s.len();
        let mut rk: usize = 0;
        let mut ans = 0;

        for i in 0..n {
            if i != 0 {
                occ.remove(&s.chars().nth(i - 1).unwrap());
            }

            while rk < n && !occ.contains(&s.chars().nth(rk).unwrap()) {
                occ.insert(s.chars().nth(rk).unwrap());
                rk += 1;
            }
            ans = ans.max(rk - 1);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::s0003_longest_substring_without_repeating_characters::Solution;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::length_of_longest_substring(String::from("abcabcbb")));
        assert_eq!(1, Solution::length_of_longest_substring(String::from("bbbbb")));
        assert_eq!(3, Solution::length_of_longest_substring(String::from("pwwkew")));
    }
}