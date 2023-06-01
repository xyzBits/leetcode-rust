struct Solution {}


impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let len = strs.len();

        let mut prefix = strs[0].clone();
        for i in 0..len {
            prefix = Self::longest_common_prefix_helper(&prefix, &strs[i]);
            if prefix.is_empty() {
                break;
            }
        }

        prefix
    }

    pub fn longest_common_prefix_helper(str1: &str, str2: &str) -> String {
        let length = std::cmp::min(str1.len(), str2.len());
        let mut index = 0;
        while index < length && str1.chars().nth(index) == str2.chars().nth(index) {
            index += 1;
        }
        str1[0..index].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::s0014_longest_common_prefix::Solution;

    #[test]
    fn test_1() {
        assert_eq!("fl", Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
        assert_eq!("", Solution::longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]));

    }
}