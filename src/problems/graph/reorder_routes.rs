pub struct Solution;

impl Solution {
    fn dfs(connections: &Vec<Vec<(usize, bool)>>, node: usize, parent: usize) -> i32 {
        let mut output = 0;

        for &(neighbor, direction) in connections[node].iter() {
            if neighbor != parent {
                if direction {
                    output += 1
                }
                output += Self::dfs(connections, neighbor, node)
            }
        }
        output
    }

    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let connections = connections.iter().fold(
            vec![vec![]; connections.len() + 1],
            |mut acc, connection| {
                acc[connection[0] as usize].push((connection[1] as usize, true));
                acc[connection[1] as usize].push((connection[0] as usize, false));
                acc
            },
        );
        Self::dfs(&connections, 0, n as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 6;
        let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];

        let expected = 3;

        assert_eq!(Solution::min_reorder(n, connections), expected)
    }

    #[test]
    fn test_example_2() {
        let n = 5;
        let connections = vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]];

        let expected = 2;

        assert_eq!(Solution::min_reorder(n, connections), expected)
    }
    #[test]
    fn test_example_3() {
        let n = 3;
        let connections = vec![vec![1, 0], vec![2, 0]];

        let expected = 0;

        assert_eq!(Solution::min_reorder(n, connections), expected)
    }
}
