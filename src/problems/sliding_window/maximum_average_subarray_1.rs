pub struct Solution;

// ehh better solution is to have some current_sum variable, and then
// adding/subtracting the windows left/right index to this sum. having to make
// an iter for each window is super super whack. CBA to reimplement X)
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        nums.windows(k as usize)
            .map(|window| window.iter().sum::<i32>())
            .max()
            .unwrap() as f64
            / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        let expected: f64 = 12.75000;

        assert_eq!(Solution::find_max_average(nums, k), expected)
    }

    #[test]
    fn test_example_2() {
        let nums = vec![5];
        let k = 1;
        let expected: f64 = 5.00000;

        assert_eq!(Solution::find_max_average(nums, k), expected)
    }
}
