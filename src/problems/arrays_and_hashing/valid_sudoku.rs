use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set = HashSet::with_capacity(9);
        let board = &board;

        // check rows & columns
        for i in 0..9 {
            if !Self::slice_is_valid(&board[i], &mut set) {
                return false;
            }

            if !Self::slice_is_valid(
                &(0..9).map(|j| board[j][i]).collect::<Vec<char>>(),
                &mut set,
            ) {
                return false;
            }
        }

        // check 3x3 subgrids
        (0..9).step_by(3).all(|row| {
            (0..9).step_by(3).all(|col| {
                Self::slice_is_valid(
                    &(0..3)
                        .flat_map(|i| (0..3).map(move |j| board[row + i][col + j]))
                        .collect::<Vec<char>>(),
                    &mut set,
                )
            })
        })
    }

    fn slice_is_valid(slice: &[char], set: &mut HashSet<char>) -> bool {
        set.clear();

        slice.iter().all(|ch| *ch == '.' || set.insert(*ch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_board() {
        let board: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(Solution::is_valid_sudoku(board))
    }

    #[test]
    fn test_invalid_board() {
        let board: Vec<Vec<char>> = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert!(!Solution::is_valid_sudoku(board))
    }

    #[test]
    fn test_board_2() {
        let board: Vec<Vec<char>> = vec![
            vec!['.', '9', '.', '.', '4', '.', '.', '.', '.'],
            vec!['1', '.', '.', '.', '.', '.', '6', '.', '.'],
            vec!['.', '.', '3', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['3', '.', '.', '.', '5', '.', '.', '.', '.'],
            vec!['.', '.', '7', '.', '.', '4', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '7', '.', '.', '.', '.'],
        ];

        assert!(Solution::is_valid_sudoku(board))
    }

    #[test]
    fn test_invalid_subgrid() {
        let board: Vec<Vec<char>> = vec![
            vec!['.', '9', '.', '.', '4', '.', '.', '.', '.'],
            vec!['1', '.', '.', '.', '.', '.', '6', '.', '.'],
            vec!['.', '.', '9', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['3', '.', '.', '.', '5', '.', '.', '.', '.'],
            vec!['.', '.', '7', '.', '.', '4', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '7', '.', '.', '.', '.'],
        ];

        assert!(!Solution::is_valid_sudoku(board))
    }
}
