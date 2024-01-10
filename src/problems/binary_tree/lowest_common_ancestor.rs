use crate::data_structures::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type TreeNodeRef = Rc<RefCell<TreeNode>>;

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<TreeNodeRef>,
        p: Option<TreeNodeRef>,
        q: Option<TreeNodeRef>,
    ) -> Option<TreeNodeRef> {
        if let (Some(p), Some(q)) = (p, q) {
            Self::helper(&root, &p.borrow(), &q.borrow())
        } else {
            None
        }
    }

    // avoid having to constantly borrow + clone p & q
    fn helper(maybe_node: &Option<TreeNodeRef>, p: &TreeNode, q: &TreeNode) -> Option<TreeNodeRef> {
        if let Some(node) = maybe_node {
            let node_borrow = node.borrow();

            if node_borrow.val == p.val || node_borrow.val == q.val {
                return Some(Rc::clone(node));
            }

            let left = Self::helper(&node_borrow.left, p, q);

            let right = Self::helper(&node_borrow.right, p, q);

            match (left, right) {
                (Some(_), Some(_)) => Some(Rc::clone(node)),
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
