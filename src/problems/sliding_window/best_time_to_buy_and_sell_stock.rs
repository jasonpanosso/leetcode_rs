pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut cheapest = prices[0];

        for price in prices.iter() {
            cheapest = cheapest.min(*price);
            max_profit = max_profit.max(price - cheapest);
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(prices), 5)
    }

    #[test]
    fn test_group_2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0)
    }

    #[test]
    fn test_group_3() {
        let prices = vec![7];
        assert_eq!(Solution::max_profit(prices), 0)
    }
}
