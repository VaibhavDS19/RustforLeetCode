impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut m1 = 0;
        let mut m2 = 0;
        for i in nums.iter() {
            if m1<*i {
                m2 = m1;
                m1 = *i;
            }
            else if m2<*i {
                m2 = *i;
            }
        }
        (m1-1)*(m2-1)
    }
}