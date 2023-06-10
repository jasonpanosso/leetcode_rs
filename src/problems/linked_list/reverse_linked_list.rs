// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use crate::data_structures::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node: Option<Box<ListNode>> = None;
        while let Some(next) = head {
            node = Some(Box::new(ListNode {
                val: next.val,
                next: node,
            }));
            head = next.next
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        let list = ListNode::create_linked_list(vec![1, 2, 3]);

        let reversed = ListNode::create_linked_list(vec![3, 2, 1]);
        assert_eq!(Solution::reverse_list(list), reversed);
    }

    #[test]
    fn test_empty_list() {
        let list = ListNode::create_linked_list(vec![]);

        let reversed = ListNode::create_linked_list(vec![]);
        assert_eq!(Solution::reverse_list(list), reversed);
    }

    #[test]
    fn test_reverse_list_single_node() {
        let list = ListNode::create_linked_list(vec![1]);

        let reversed = ListNode::create_linked_list(vec![1]);
        assert_eq!(Solution::reverse_list(list), reversed);
    }
}
