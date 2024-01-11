pub struct Solution;

// this problem is terrible
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut stack: Vec<char> = vec![];
        let mut queue: Vec<char> = senate.chars().collect();

        while !queue.is_empty() {
            let char = queue.remove(0);

            if stack.is_empty() || stack.last().unwrap() == &char {
                stack.push(char);
            } else {
                queue.push(stack.pop().unwrap());
            }
        }

        match stack.pop().unwrap() {
            'R' => String::from("Radiant"),
            'D' => String::from("Dire"),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let senate = String::from("RD");
        let expected = String::from("Radiant");

        assert_eq!(Solution::predict_party_victory(senate), expected)
    }

    #[test]
    fn test_example_2() {
        let senate = String::from("RDD");
        let expected = String::from("Dire");

        assert_eq!(Solution::predict_party_victory(senate), expected)
    }
    #[test]
    fn test_example_3() {
        let senate = String::from("RRDRDD");
        let expected = String::from("Radiant");

        assert_eq!(Solution::predict_party_victory(senate), expected)
    }

    #[test]
    fn test_example_4() {
        let senate = String::from("RRRRDD");
        let expected = String::from("Radiant");

        assert_eq!(Solution::predict_party_victory(senate), expected)
    }

    #[test]
    fn test_example_5() {
        let senate = String::from("DDRRR");
        let expected = String::from("Dire");

        assert_eq!(Solution::predict_party_victory(senate), expected)
    }
}
