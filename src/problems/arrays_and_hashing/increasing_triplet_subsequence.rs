pub struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut i = i32::MAX;
        let mut j = i32::MAX;

        for num in nums {
            if num <= i {
                i = num;
            } else if num <= j {
                j = num;
            } else {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let expected = true;

        println!("==START==");
        println!("NUMS: {:?}", nums);
        println!("EXPECTED: {:?}", expected);

        assert_eq!(Solution::increasing_triplet(nums), expected);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![5, 4, 3, 2, 1];
        let expected = false;

        println!("==START==");
        println!("NUMS: {:?}", nums);
        println!("EXPECTED: {:?}", expected);

        assert_eq!(Solution::increasing_triplet(nums), expected);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![2, 1, 5, 0, 4, 6];
        let expected = true;

        println!("==START==");
        println!("NUMS: {:?}", nums);
        println!("EXPECTED: {:?}", expected);

        assert_eq!(Solution::increasing_triplet(nums), expected);
    }

    #[test]
    fn test_example_4() {
        let nums = vec![1, 5, 0, 4, 1, 3];
        let expected = true;

        println!("==START==");
        println!("NUMS: {:?}", nums);
        println!("EXPECTED: {:?}", expected);

        assert_eq!(Solution::increasing_triplet(nums), expected);
    }

    #[test]
    fn test_example_5() {
        let nums = vec![5, 1, 6];
        let expected = false;

        println!("==START==");
        println!("NUMS: {:?}", nums);
        println!("EXPECTED: {:?}", expected);

        assert_eq!(Solution::increasing_triplet(nums), expected);
    }

    #[test]
    fn test_example_6() {
        let nums = vec![4, 6, 3, 4, 5];
        let expected = true;

        println!("==START==");
        println!("NUMS: {:?}", nums);
        println!("EXPECTED: {:?}", expected);

        assert_eq!(Solution::increasing_triplet(nums), expected);
    }

    #[test]
    fn test_example_7() {
        let nums = vec![9, 10, 5, 11, 10, 9, 8];
        let expected = true;

        println!("==START==");
        println!("NUMS: {:?}", nums);
        println!("EXPECTED: {:?}", expected);

        assert_eq!(Solution::increasing_triplet(nums), expected);
    }

    #[test]
    fn test_example_8() {
        let nums = vec![5, 1, 5, 5, 2, 5, 4];
        let expected = true;

        println!("==START==");
        println!("NUMS: {:?}", nums);
        println!("EXPECTED: {:?}", expected);

        assert_eq!(Solution::increasing_triplet(nums), expected);
    }
}
