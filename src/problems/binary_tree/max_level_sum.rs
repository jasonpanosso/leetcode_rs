pub struct Solution;

use crate::data_structures::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut output = 0;

        if let Some(root) = root {
            let mut cur_max = i32::MIN;
            let mut cur_depth = 0;

            let mut queue = VecDeque::new();
            queue.push_back(root);

            while !queue.is_empty() {
                cur_depth += 1;
                let queue_len = queue.len();

                let level_sum: i32 = (0..queue_len).fold(0, |sum: i32, _| {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();

                    if let Some(left) = &node.left {
                        queue.push_back(left.clone());
                    }

                    if let Some(right) = &node.right {
                        queue.push_back(right.clone());
                    }

                    sum + node.val
                });

                if level_sum > cur_max {
                    cur_max = level_sum;
                    output = cur_depth;
                }
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
        let tree = vec![Some(1), Some(7), Some(0), Some(7), Some(-8), None, None];

        let root = TreeNode::from_vec(tree);
        let expected = 2;

        assert_eq!(Solution::max_level_sum(root), expected);
    }

    #[test]
    fn test_example_2() {
        let tree = vec![
            Some(989),
            None,
            Some(10250),
            Some(98693),
            Some(-89388),
            None,
            None,
            None,
            Some(-32127),
        ];

        let root = TreeNode::from_vec(tree);
        let expected = 2;

        assert_eq!(Solution::max_level_sum(root), expected);
    }
}
