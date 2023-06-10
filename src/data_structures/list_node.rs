#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn create_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &num in nums.iter().rev() {
            let mut new_node = ListNode::new(num);
            new_node.next = current;
            current = Some(Box::new(new_node));
        }
        current
    }
}
