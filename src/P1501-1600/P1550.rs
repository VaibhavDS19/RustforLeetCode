impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for i in arr {
            if i&1 == 1 {
                count += 1;
                if count>2 {
                    return true;
                }
            }
            else {
                count = 0;
            }
        }
        false
    }
}