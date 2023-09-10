pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut wrapper: Vec<String> = vec![String::from(""); num_rows as usize];

        let mut ascending = true;
        let mut i: i32 = 0;

        s.chars().for_each(|c| {
            wrapper[i as usize].push(c);
            match ascending {
                true => {
                    if i == num_rows - 1 {
                        ascending = false;
                        i -= 1;
                    } else {
                        i += 1;
                    }
                }
                false => {
                    if i == 0 {
                        ascending = true;
                        i += 1;
                    } else {
                        i -= 1;
                    }
                }
            }
        });
        wrapper.into_iter().reduce(|acc, cur| acc + &cur).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_rows() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 3;

        let expected = String::from("PAHNAPLSIIGYIR");

        assert_eq!(Solution::convert(s, num_rows), expected);
    }

    #[test]
    fn test_4_rows() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 4;

        let expected = String::from("PINALSIGYAHRPI");

        assert_eq!(Solution::convert(s, num_rows), expected);
    }

    #[test]
    fn test_1_row_single_char_input() {
        let s = String::from("A");
        let num_rows = 1;

        let expected = String::from("A");

        assert_eq!(Solution::convert(s, num_rows), expected);
    }
}
