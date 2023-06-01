use std::collections::HashMap;

pub struct Solution {}


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![];
        }

        let mut map = HashMap::with_capacity(nums.len());

        for (index, value) in nums.iter().enumerate() {
            let remain = target - *value;

            match map.get(&remain) {
                Some(remain_index) => {
                    return vec![*remain_index, index as i32];
                }
                None => {
                    map.insert(*value, index as i32);
                }
            }
        }

        return vec![];
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
        assert_eq!(Vec::<i32>::new(), Solution::two_sum(vec![], -1));
    }
}














