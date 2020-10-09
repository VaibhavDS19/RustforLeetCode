impl Solution {
   pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
       let mut v:Vec<i32> = Vec::new();
       let mut n;
       let mut m;
       let mut i=0;
       while i<nums.len() {
           n=nums[i];
           i+=1;
           m=nums[i];
           for j in 0..n{
               v.push(m);
           }
           i+=1
       }
       v
   }
}