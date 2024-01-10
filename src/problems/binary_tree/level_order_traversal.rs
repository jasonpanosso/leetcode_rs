use crate::data_structures::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type TreeNodeRef = Rc<RefCell<TreeNode>>;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<TreeNodeRef>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut output: Vec<Vec<i32>> = vec![];
        let mut queue: VecDeque<(TreeNodeRef, usize)> = VecDeque::new();
        queue.push_front((root.unwrap(), 0));

        while let Some((node, depth)) = queue.pop_front() {
            let borrowed_node = node.borrow();

            while output.len() != depth + 1 {
                output.push(vec![]);
            }

            output[depth].push(borrowed_node.val);

            if let Some(left) = &borrowed_node.left {
                queue.push_back((Rc::clone(left), depth + 1));
            }

            if let Some(right) = &borrowed_node.right {
                queue.push_back((Rc::clone(right), depth + 1));
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];

        let root = TreeNode::from_vec(tree);

        let expected: Vec<Vec<i32>> = vec![vec![3], vec![9, 20], vec![15, 7]];

        assert_eq!(Solution::level_order(root), expected)
    }
}
