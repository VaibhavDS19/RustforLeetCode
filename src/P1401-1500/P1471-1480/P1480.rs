impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        let mut run_sum:Vec<i32> = Vec::new();
        for i in nums.iter() {
            sum += i;
            run_sum.push(sum);
        }
        run_sum
    }
}