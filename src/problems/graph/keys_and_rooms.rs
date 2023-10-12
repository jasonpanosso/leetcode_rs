pub struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let rooms_len = rooms.len();
        let mut queue = VecDeque::with_capacity(rooms_len);
        let mut set = HashSet::with_capacity(rooms_len);

        queue.extend(&rooms[0]);
        set.insert(0);

        let mut visited_rooms = 1;

        while let Some(next_room) = queue.pop_back() {
            if set.insert(next_room) {
                visited_rooms += 1;
                if rooms_len == visited_rooms {
                    return true;
                }
                queue.extend(&rooms[next_room as usize]);
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
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];

        let expected = true;

        assert_eq!(Solution::can_visit_all_rooms(rooms), expected)
    }

    #[test]
    fn test_example_2() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];

        let expected = false;

        assert_eq!(Solution::can_visit_all_rooms(rooms), expected)
    }
}
