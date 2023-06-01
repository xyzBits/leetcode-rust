pub struct Solution {}

use crate::util::linked_list::ListNode;
use crate::util::linked_list::to_list;

/***
https://leetcode.cn/problems/add-two-numbers/
*/
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(-1)));
        let mut tail = &mut dummy_head;
        let (mut slide_pointer1, mut slide_pointer2) = (l1, l2);

        let (mut l1_end, mut l2_end, mut overflow) = (false, false, false);

        loop {
            let val1 = match slide_pointer1 {
                Some(node) => {
                    slide_pointer1 = node.next;
                    node.val
                }
                None => {
                    l1_end = true;
                    0
                }
            };

            let val2 = match slide_pointer2 {
                Some(node) => {
                    slide_pointer2 = node.next;
                    node.val
                }
                None => {
                    l2_end = true;
                    0
                }
            };

            if l1_end && l2_end && !overflow {
                break dummy_head.unwrap().next;
            }

            let sum = val1 + val2 + if overflow { 1 } else { 0 };
            let sum = if sum >= 10 {
                overflow = true;
                sum - 10
            } else {
                overflow = false;
                sum
            };

            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(None, Solution::add_two_numbers(None, None));

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}