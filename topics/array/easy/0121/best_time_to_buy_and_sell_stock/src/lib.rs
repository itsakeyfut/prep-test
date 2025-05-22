struct Solution;
impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for curr_price in prices {
            min_price = min_price.min(curr_price);
            max_profit = max_profit.max(curr_price - min_price);
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let prices = vec![7,1,5,3,6,4];
        let expected = 5;
        assert_eq!(Solution::max_profit(prices), expected);
    }

    #[test]
    fn example2() {
        let prices = vec![7,6,4,3,1];
        let expected = 0;
        assert_eq!(Solution::max_profit(prices), expected);
    }
}