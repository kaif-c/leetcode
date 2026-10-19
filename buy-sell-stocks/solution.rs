pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // Need to find maximum possible difference
        // Modified Kadane's Algorithm
        // We do not care about buy, we just make sure its smallest possible
        // If difference is greater than previous one then we set it
        // Since we iterate forward the difference between buy and current being
        // the greatest means it is always after buy date
        let mut buy = prices[0];
        let mut profit = 0;
        for i in 1..prices.len() {
            if prices[i] < buy {
                buy = prices[i];
            }
            else if prices[i] - buy > profit {
                profit = prices[i] - buy;
            }
        }
        return profit;
    }
}
