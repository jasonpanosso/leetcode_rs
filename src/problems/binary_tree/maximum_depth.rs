pub struct Solution;

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

use crate::data_structures::tree_node::TreeNode;
impl Solution {
    fn helper(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        match root {
            Some(node) => {
                return max(
                    Self::helper(node.borrow().left.clone(), depth + 1),
                    Self::helper(node.borrow().right.clone(), depth + 1),
                );
            }
            None => depth,
        }
    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(root, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut node1 = TreeNode::new(3);
        let node2 = TreeNode::new(9);
        let mut node3 = TreeNode::new(20);
        let node4 = TreeNode::new(15);
        let node5 = TreeNode::new(7);

        node3.left = Some(Rc::new(RefCell::new(node4)));
        node3.right = Some(Rc::new(RefCell::new(node5)));

        node1.left = Some(Rc::new(RefCell::new(node2)));
        node1.right = Some(Rc::new(RefCell::new(node3)));

        println!("{:?}", node1);

        let expected = 3;
        assert_eq!(
            Solution::max_depth(Some(Rc::new(RefCell::new(node1)))),
            expected
        );
    }
}
