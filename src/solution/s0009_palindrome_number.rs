
struct Solution {}

impl Solution {
    /// given an integer x, return true if x is a palindrome, and false otherwise
    /// cloud you solve it without converting the integer to a string
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return  false; }
        let mut x = x;
        let mut vec = vec![];

        while x > 0 {
            vec.push(x % 10);
            x = x / 10;
        }


        let len = vec.len();

        for i in 0..len / 2 {
            if vec[i] != vec[len - i - 1] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_palindrome(121));
        assert_eq!(true, Solution::is_palindrome(123321));
        assert_eq!(false, Solution::is_palindrome(12345));
    }
}