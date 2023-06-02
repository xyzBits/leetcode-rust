/// you are give two integer arrays and nums3
/// sorted in non-decreasing order,
/// and two integers m and n, representing the number of elements in nums1 and nums2 respectively
///
/// merge nums1 and nums2 into a single array sorted in non-decreasing order
///
/// the final soted array should not be returned by the function, but instead be stored inside the array nums1,
///
/// to accommodate this,
/// nums1 has a length of m + n, where the first m elements denote the elements that should be
/// merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n
///
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p1 = m - 1;
        let mut p2 = n - 1;
        let mut cursor = m + n - 1;

        while p2 >= 0 {

            // p1 is empty, we should add this check
            if p1 >= 0 && nums1[p1 as usize] > nums2[p2 as usize] {
                nums1[cursor as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[cursor as usize] = nums2[p2 as usize];
                p2 -= 1;
            }


            cursor -= 1;
        }
    }
}


struct Solution {}

#[cfg(test)]
mod tests {
    use crate::solution::s_0088_merge_sorted_array::Solution;

    #[test]
    fn test_88_works() {
        let mut vec1 = vec![1, 2, 3, 0, 0, 0];
        let mut vec2 = vec![2, 5, 6];
        Solution::merge(&mut vec1, 3, &mut vec2, 3);
        assert_eq!(vec1, vec![1, 2, 2, 3, 5, 6]);


        let mut vec1 = vec![1, 2, 3];
        let mut vec2 = vec![];
        Solution::merge(&mut vec1, 3, &mut vec2, 0);
        assert_eq!(vec1, vec![1, 2, 3]);

        let mut vec1 = vec![0, 0, 0];
        let mut vec2 = vec![1, 2, 3];
        Solution::merge(&mut vec1, 0, &mut vec2, 3);
        assert_eq!(vec1, vec![1, 2, 3]);
    }
}