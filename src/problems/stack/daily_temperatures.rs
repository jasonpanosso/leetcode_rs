pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = vec![];
        let mut res: Vec<i32> = vec![0; temperatures.len()];

        for (i, &temp) in temperatures.iter().enumerate() {
            while let Some(&last) = stack.last() {
                if temperatures[last] >= temp {
                    break;
                }
                res[last] = (i - last) as i32;
                stack.pop();
            }
            stack.push(i);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temps_always_increasing() {
        let temps = vec![30, 40, 50, 60];
        let expected = vec![1, 1, 1, 0];
        assert_eq!(Solution::daily_temperatures(temps), expected);
    }

    #[test]
    fn test_temps_varying_increasing_decreasing_temps() {
        let temps = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(temps), expected);
    }

    #[test]
    fn test_temps_always_decreasing() {
        let temps = vec![70, 60, 50, 40];
        let expected = vec![0, 0, 0, 0];
        assert_eq!(Solution::daily_temperatures(temps), expected);
    }
}
