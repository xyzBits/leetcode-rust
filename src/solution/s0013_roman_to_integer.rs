use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut symbol_value = HashMap::new();
        symbol_value.insert('I', 1);
        symbol_value.insert('V', 5);
        symbol_value.insert('X', 10);
        symbol_value.insert('L', 50);
        symbol_value.insert('C', 100);
        symbol_value.insert('D', 500);
        symbol_value.insert('M', 1000);

        let mut ans = 0;
        let n = s.len();

        for i in 0..n {
            let ch = s.chars().nth(i).unwrap();
            let value = symbol_value.get(&ch).copied().unwrap();
            if i < n - 1 && value < symbol_value.get(&s.chars().nth(i + 1).unwrap()).copied().unwrap() {
                ans -= value;
            } else {
                ans += value;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}