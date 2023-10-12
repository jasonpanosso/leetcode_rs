pub struct Solution;

impl Solution {
    fn dfs(adj_matrix: &Vec<Vec<i32>>, visited: &mut Vec<bool>, node: usize) {
        visited[node] = true;

        for (adj_node, &connected) in adj_matrix[node].iter().enumerate() {
            if connected == 1 && !visited[adj_node] {
                Self::dfs(adj_matrix, visited, adj_node);
            }
        }
    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut output = 0;

        let len = is_connected.len();
        let mut visited = vec![false; len];

        for i in 0..len {
            if !visited[i] {
                output += 1;
                Self::dfs(&is_connected, &mut visited, i);
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
        let cities = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];

        let expected = 2;

        assert_eq!(Solution::find_circle_num(cities), expected)
    }

    #[test]
    fn test_example_2() {
        let cities = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];

        let expected = 3;

        assert_eq!(Solution::find_circle_num(cities), expected)
    }

    #[test]
    fn test_example_3() {
        let cities = vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
        ];

        let expected = 1;

        assert_eq!(Solution::find_circle_num(cities), expected)
    }
}
