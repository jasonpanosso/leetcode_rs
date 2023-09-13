pub struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::with_capacity(asteroids.len());

        for &space_rock in &asteroids {
            loop {
                if let Some(&last_space_rock) = output.last() {
                    match (space_rock > 0, last_space_rock > 0) {
                        (true, true) | (false, false) | (true, false) => {
                            output.push(space_rock);
                            break;
                        }
                        _ => match space_rock.abs().cmp(&last_space_rock.abs()) {
                            Ordering::Equal => {
                                output.pop();
                                break;
                            }
                            Ordering::Greater => {
                                output.pop();
                            }
                            Ordering::Less => {
                                break;
                            }
                        },
                    }
                } else {
                    output.push(space_rock);
                    break;
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let asteroids = vec![5, 10, -5];
        let expected = vec![5, 10];

        assert_eq!(Solution::asteroid_collision(asteroids), expected)
    }

    #[test]
    fn test_example_2() {
        let asteroids = vec![8, -8];
        let expected = vec![];

        assert_eq!(Solution::asteroid_collision(asteroids), expected)
    }

    #[test]
    fn test_example_3() {
        let asteroids = vec![10, 2, -5];
        let expected = vec![10];

        assert_eq!(Solution::asteroid_collision(asteroids), expected)
    }
}
