use crate::data_structures::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut length = 0;
        let mut node = head.as_ref();
        while node.is_some() {
            node = node.unwrap().next.as_ref();
            length += 1;
        }

        if length <= 1 {
            return None;
        }

        let target = length / 2 - 1;
        length = 0;

        let mut node = head.as_mut();

        while length < target {
            node = node.unwrap().next.as_mut();
            length += 1;
        }
        node.unwrap().next = node.as_mut().unwrap().next.as_mut().unwrap().next.take();

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let list = ListNode::create_linked_list(vec![1, 3, 4, 7, 1, 2, 6]);
        let expected = ListNode::create_linked_list(vec![1, 3, 4, 1, 2, 6]);

        assert_eq!(Solution::delete_middle(list), expected);
    }

    #[test]
    fn test_example_2() {
        let list = ListNode::create_linked_list(vec![1, 2, 3, 4]);
        let expected = ListNode::create_linked_list(vec![1, 2, 4]);

        assert_eq!(Solution::delete_middle(list), expected);
    }
}
