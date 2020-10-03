impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        high/2 + (high&1) - low/2
    }
}