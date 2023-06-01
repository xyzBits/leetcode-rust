
struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (length1, length2) = (nums1.len(), nums2.len());
        let total_length = length1 + length2;

        return if total_length % 2 == 1 {
            let mid_index = total_length / 2;
            let median = Self::get_kth_element(&nums1, &nums2, mid_index + 1);
            median
        } else {
            let (mid_index_1, mid_index_2) = (total_length / 2 - 1, total_length / 2);
            let median = (Self::get_kth_element(&nums1, &nums2, mid_index_1 + 1) + Self::get_kth_element(&nums1, &nums2, mid_index_2 + 1)) / 2.;
            median
        };
    }


    fn get_kth_element(nums1: &Vec<i32>, nums2: &Vec<i32>, mut k: usize) -> f64 {
        let (length1, length2) = (nums1.len(), nums2.len());
        let (mut index_1, mut index_2) = (0, 0);

        let kth_element = 0;
        loop {
            if index_1 == length1 {
                return nums2[index_2 + k - 1] as f64;
            }

            if index_2 == length2 {
                return nums1[index_1 + k - 1] as f64;
            }

            if k == 1 {
                return std::cmp::min(nums1[index_1], nums2[index_2]) as f64;
            }

            let half = k / 2;
            let new_index_1 = std::cmp::min(index_1 + half, length1) - 1;
            let new_index_2 = std::cmp::min(index_2 + half, length2) - 1;
            let (pivot_1, pivot_2) = (nums1[new_index_1], nums2[new_index_2]);

            if pivot_1 <= pivot_2 {
                k -= (new_index_1 - index_1 + 1);
                index_1 = new_index_1 + 1;
            } else {
                k -= (new_index_2 - index_2 + 1);
                index_2 = new_index_2 + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(2., Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
        assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
    }
}