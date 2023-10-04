#[derive(Debug)]
pub struct RecentCounter {
    current_time: i32,
    pings: Vec<i32>,
    recent_range: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
impl RecentCounter {
    pub fn new() -> Self {
        RecentCounter {
            current_time: 0,
            pings: vec![],
            recent_range: 3000,
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        self.pings.retain(|time| *time >= t - &self.recent_range);

        self.current_time = t;
        self.pings.push(t);

        self.pings.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut test = RecentCounter::new();

        let ping1 = RecentCounter::ping(&mut test, 1);
        println!("{:?}", test);
        assert_eq!(ping1, 1);

        let ping2 = RecentCounter::ping(&mut test, 100);
        println!("{:?}", test);
        assert_eq!(ping2, 2);

        let ping3 = RecentCounter::ping(&mut test, 3001);
        println!("{:?}", test);
        assert_eq!(ping3, 3);

        let ping4 = RecentCounter::ping(&mut test, 3002);
        println!("{:?}", test);
        assert_eq!(ping4, 3);
    }

    // #[test]
    // fn test_example_2() {
    //     assert_eq!(Solution::predict_party_victory(senate), expected)
    // }
}
