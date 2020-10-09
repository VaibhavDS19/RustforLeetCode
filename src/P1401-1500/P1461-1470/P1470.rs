impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut shuffled= vec![0; 2*n];
        for i in 0..n {
            shuffled[i*2] = nums[i];
            shuffled[i*2 + 1] = nums[n+i];
        }
        shuffled
    }
}