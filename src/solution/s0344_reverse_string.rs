struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let length = s.len();
        let (mut left, mut right) = (0, length - 1);

        while left < right {
            let tmp = s[left];
            s[left] = s[right];
            s[right] = tmp;

            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        println!("{:?}", s); // Output: ['o', 'l', 'l', 'e', 'h']
        // assert_eq!(Solution::reverse_string(vec!["h".to_string(),"e".to_string(),"l".to_string(),"l".to_string(),"o".to_string()]), vec!["o","l","l","e","h"])
    }
}