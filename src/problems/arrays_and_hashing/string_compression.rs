// this problem is so bad LMAO..
pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() == 1 {
            return 1;
        }
        chars.push('\0');
        let mut char_repeat_count = 0;
        let mut prev_char: char = chars[0];

        let s: String = chars.iter().fold(String::from(""), |mut str, char| {
            if prev_char == *char {
                char_repeat_count += 1;
            } else {
                str += &prev_char.to_string();
                if char_repeat_count != 1 {
                    str += &char_repeat_count.to_string();
                }
                char_repeat_count = 1;
            }
            prev_char = *char;
            str
        });

        let mut i = 0;
        s.chars().for_each(|c| {
            chars[i] = c;
            i += 1;
        });
        println!("out: {:?}", s);
        s.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let expected = 6;
        println!("{:?}", chars);

        assert_eq!(Solution::compress(&mut chars), expected);
        assert_eq!(
            &chars[0..expected as usize],
            vec!['a', '2', 'b', '2', 'c', '3']
        );
    }

    #[test]
    fn test_standard_2() {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        let expected = 4;

        println!("{:?}", chars);
        assert_eq!(Solution::compress(&mut chars), expected);
        assert_eq!(chars[0..expected as usize], vec!['a', 'b', '1', '2']);
    }

    #[test]
    fn test_single_char() {
        let mut chars = vec!['a'];
        let expected = 1;

        println!("{:?}", chars);
        assert_eq!(Solution::compress(&mut chars), expected);
        assert_eq!(chars[0..expected as usize], vec!['a']);
    }

    #[test]
    fn test_three_single_chars() {
        let mut chars = vec!['a', 'b', 'c'];
        let expected = 3;

        println!("{:?}", chars);
        assert_eq!(Solution::compress(&mut chars), expected);
        assert_eq!(chars[0..expected as usize], vec!['a', 'b', 'c']);
    }
}
