pub struct Solution;

use crate::data_structures::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Node {
        if let (Some(node), Some(p), Some(q)) = (&root, &p, &q) {
            if node.borrow().val == p.borrow().val || node.borrow().val == q.borrow().val {
                return Some(node.clone());
            }

            let left = Self::lowest_common_ancestor(
                node.borrow().left.clone(),
                Some(p.clone()),
                Some(q.clone()),
            );

            let right = Self::lowest_common_ancestor(
                node.borrow().right.clone(),
                Some(p.clone()),
                Some(q.clone()),
            );

            match (left, right) {
                (Some(_), Some(_)) => Some(node.clone()),
                (Some(left_or_right), None) | (None, Some(left_or_right)) => Some(left_or_right),
                (None, None) => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ];

        let root = TreeNode::from_vec(tree);

        assert_eq!(
            Solution::lowest_common_ancestor(
                root.clone(),
                root.clone().unwrap().borrow().left.clone(),
                root.clone().unwrap().borrow().right.clone()
            ),
            root.clone()
        )
    }
}
