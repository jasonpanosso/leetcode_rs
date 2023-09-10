pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut prev_had_flower = false;
        let mut planted_count = 0;

        for i in 0..flowerbed.len() {
            match flowerbed[i] {
                0 => {
                    if !prev_had_flower && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0) {
                        planted_count += 1;
                        prev_had_flower = true;
                    } else {
                        prev_had_flower = false;
                    }
                }
                1 => {
                    prev_had_flower = true;
                }
                _ => unreachable!(),
            }
        }
        planted_count >= n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;

        let expected = true;

        assert_eq!(Solution::can_place_flowers(flowerbed, n), expected);
    }

    #[test]
    fn test_example_2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;

        let expected = false;

        assert_eq!(Solution::can_place_flowers(flowerbed, n), expected);
    }
}
