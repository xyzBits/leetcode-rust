struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut rev = 0;

        let mut input = x;
        while input != 0 {
            if rev < i32::MIN / 10 || rev > i32::MAX / 10 {
                return 0;
            }

            let digit = input % 10;
            input /= 10;
            rev = rev * 10 + digit;
        }

        rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);

        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(-123000), -321);
        let base: i64 = 2;
        assert_eq!(Solution::reverse((base.pow(31) - 1) as i32), 0);
        assert_eq!(Solution::reverse((-base.pow(31)) as i32), 0);
    }
}