pub struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        let mut result = right;

        while left < right {
            let mid = left + (right - left) / 2;
            if h >= piles
                .iter()
                .fold(0, |acc, pile| acc + Self::ceiling_div(*pile, mid))
            {
                result = result.min(mid);
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        result
    }

    fn ceiling_div(a: i32, b: i32) -> i32 {
        if a % b != 0 {
            a / b + 1
        } else {
            a / b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // wtf do I even name these tests
    #[test]
    fn test_h_twice_len_piles() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        assert_eq!(Solution::min_eating_speed(piles, h), 4);
    }

    #[test]
    fn test_h_equals_len_piles() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        assert_eq!(Solution::min_eating_speed(piles, h), 30);
    }

    #[test]
    fn test_h_equals_len_piles_plus_one() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        assert_eq!(Solution::min_eating_speed(piles, h), 23);
    }


    #[test]
    fn test_len_piles_equals_one() {
        let piles = vec![4];
        let h = 1;
        assert_eq!(Solution::min_eating_speed(piles, h), 4);
    }

    #[test]
    fn test_len_piles_equals_one_h_equals_twice_len_piles() {
        let piles = vec![4];
        let h = 2;
        assert_eq!(Solution::min_eating_speed(piles, h), 2);
    }
}
