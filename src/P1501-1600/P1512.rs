impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count:i32 = 0;
        let mut n=nums.len();
        for i in 0..&n - 1 as usize{
            for j in &i + 1..n {
                if nums[i]==nums[j]  {
                    count += 1;
                }
            }
        }
        count
    }
}