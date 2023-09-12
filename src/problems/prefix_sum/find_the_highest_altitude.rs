pub struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.iter()
            .fold((0, 0), |(mut cur_altitude, mut highest_point), cur_gain| {
                cur_altitude += cur_gain;
                highest_point = highest_point.max(cur_altitude);
                (cur_altitude, highest_point)
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let gain = vec![-5, 1, 5, 0, -7];
        let expected = 1;

        assert_eq!(Solution::largest_altitude(gain), expected)
    }

    #[test]
    fn test_example_2() {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];
        let expected = 0;

        assert_eq!(Solution::largest_altitude(gain), expected)
    }

    #[test]
    fn test_example_3() {
        let gain = vec![-5, 1, 5, 0, 7];
        let expected = 8;

        assert_eq!(Solution::largest_altitude(gain), expected)
    }
}
