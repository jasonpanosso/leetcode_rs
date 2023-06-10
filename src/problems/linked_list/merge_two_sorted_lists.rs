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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prehead = ListNode::new(-1);
        let mut cur_node = &mut prehead;

        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val < node2.val {
                cur_node.next = list1.take();
                cur_node = cur_node.next.as_mut().unwrap();
                list1 = cur_node.next.take();
            } else {
                cur_node.next = list2.take();
                cur_node = cur_node.next.as_mut().unwrap();
                list2 = cur_node.next.take();
            }
        }
        cur_node.next = list1.or(list2);
        prehead.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let list1 = ListNode::create_linked_list(vec![0, 3, 4]);
        let list2 = ListNode::create_linked_list(vec![1, 2, 5]);

        let merged = ListNode::create_linked_list(vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(Solution::merge_two_lists(list1, list2), merged);
    }

    #[test]
    fn test_merge_different_sized_lists() {
        let list1 = ListNode::create_linked_list(vec![0, 3, 4]);
        let list2 = ListNode::create_linked_list(vec![1]);

        let merged = ListNode::create_linked_list(vec![0, 1, 3, 4]);
        assert_eq!(Solution::merge_two_lists(list1, list2), merged);
    }

    #[test]
    fn test_merge_one_list_empty() {
        let list1 = ListNode::create_linked_list(vec![0, 3, 4]);
        let list2 = ListNode::create_linked_list(vec![]);

        let merged = ListNode::create_linked_list(vec![0, 3, 4]);
        assert_eq!(Solution::merge_two_lists(list1, list2), merged);
    }

    #[test]
    fn test_merge_both_list_empty() {
        let list1 = ListNode::create_linked_list(vec![]);
        let list2 = ListNode::create_linked_list(vec![]);

        let merged = ListNode::create_linked_list(vec![]);
        assert_eq!(Solution::merge_two_lists(list1, list2), merged);
    }
}
