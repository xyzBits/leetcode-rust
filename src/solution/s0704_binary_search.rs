struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let mid = (right - left) / 2 + left;
            let wrap_num = nums.get(mid);
            match wrap_num {
                Some(num) => {
                    if *num == target {
                        return mid as i32;
                    } else if *num > target {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
                None => {
                    return -1;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_704() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(Solution::search(vec![5], -5), -1);
    }
}