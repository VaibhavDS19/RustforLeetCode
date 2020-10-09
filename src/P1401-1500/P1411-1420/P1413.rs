impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut sum=0;
        let mut min=0;
        for i in nums.iter() {
            sum+=i;
            if sum<min {
                min=sum;
            }
        }
        if min>=0 {
            return 1;
        }
        min*(-1)+1
    }
}