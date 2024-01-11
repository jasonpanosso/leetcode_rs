pub struct Solution;

impl Solution {
    fn has_repeated_chars(chars: &[char]) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for &ch in chars {
            if ch != '.' && !set.insert(ch) {
                return true;
            }
        }
        false
    }
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // check rows valid
        for row in board.iter() {
            if Self::has_repeated_chars(row) {
                return false;
            }
        }
        // check columns valid
        for i in 0..9 {
            let column: Vec<char> = board.iter().map(|row| row[i]).collect();
            if Self::has_repeated_chars(&column) {
                return false;
            }
        }
        // check 9x9 grids valid
        for i in (0..=6).step_by(3) {
            let rows: Vec<&Vec<char>> = (i..i + 3).map(|i| &board[i]).collect();
            let mut subgrids: Vec<Vec<char>> = vec![vec![]; 3];
            for (j, k) in (0..=6).step_by(3).enumerate() {
                rows.iter()
                    .for_each(|row| subgrids[j].extend_from_slice(&row[k..k + 3]))
            }
            for grid in subgrids {
                if Self::has_repeated_chars(&grid) {
                    println!("{:?}", grid);
                    return false;
                }
            }
        }
        true
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
}
