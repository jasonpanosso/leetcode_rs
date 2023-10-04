pub struct Solution;

use crate::data_structures::list_node::ListNode;
impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        } else if head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut odd_head = head;
        let mut even_head = odd_head.as_ref().unwrap().next.to_owned();

        let mut odd_tail = odd_head.as_mut();
        let mut even_tail = even_head.as_mut();

        while even_tail.is_some() {
            let new_odd = even_tail.as_ref().unwrap().next.to_owned();
            let new_even = match new_odd.as_ref() {
                Some(node) => node.next.to_owned(),
                None => None,
            };

            odd_tail.as_mut().unwrap().next = new_odd;
            even_tail.as_mut().unwrap().next = new_even;

            if odd_tail.as_ref().unwrap().next.is_some() {
                odd_tail = odd_tail.unwrap().next.as_mut();
                even_tail = even_tail.unwrap().next.as_mut();
            } else {
                break;
            }
        }

        odd_tail.unwrap().next = even_head;

        odd_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let list = ListNode::create_linked_list(vec![1, 2, 3, 4, 5]);
        let expected = ListNode::create_linked_list(vec![1, 3, 5, 2, 4]);

        assert_eq!(Solution::odd_even_list(list), expected);
    }
}
