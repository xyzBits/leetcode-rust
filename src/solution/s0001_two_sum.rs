use std::collections::HashMap;


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        // let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        let mut map = HashMap::with_capacity(nums.len());

        for (index, value) in nums.iter().enumerate() {
            let remain = target - *value;

            match map.get(&remain) {
                Some(remain_index) => {
                    return vec![*remain_index as i32, index as i32];
                },
                None => {
                    map.insert(*value, index);
                },
            }
        }


        vec![]
    }


    pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }


        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                // if *nums.get(i).unwrap() + *nums.get(j).unwrap() == target {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![]
    }

    pub fn two_sum_with_2pass_hash_table(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        let mut map = HashMap::with_capacity(nums.len());

        for i in 0..nums.len() {
            map.insert(nums[i], i);
        }


        for i in 0..nums.len() {
            let complement  = target - nums[i];

            match map.get(&complement) {
                Some(complement_index) => {
                    if i != *complement_index {
                        return vec![*complement_index as i32, i as i32];
                    }
                },
                _ => {}
            }
        }


        vec![]
    }
}


pub struct Solution {}

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














