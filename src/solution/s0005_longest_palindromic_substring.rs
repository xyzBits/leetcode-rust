struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return String::new();//如果不是在最后一行返回，必须要加return
        }

        let (mut start, mut end) = (0, 0);

        for i in 0..s.len() {
            let len1 = Self::expand_around_center(&s, i, i);
            let len2 = Self::expand_around_center(&s, i, i + 1);
            let len = len1.max(len2);
            if len > end - start {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }

        }

        s[start..=end].to_string()
    }

    fn expand_around_center(s: &str, mut left: usize, mut right: usize) -> usize {
        let len = s.len();
        while left >= 0 && right < len && s[left..=left] == s[right..=right] {
            if left == 0 || right == len - 1 {
                break;
            }
            left -= 1;
            right += 1;
        }

        right - left - 1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("bab"), Solution::longest_palindrome(String::from("babad")));
        assert_eq!(String::from("cbbd"), Solution::longest_palindrome(String::from("bb")));
    }
}























