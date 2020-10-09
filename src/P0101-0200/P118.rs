impl Solution {
   pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
       let mut ret: Vec<Vec<i32>> = Vec::new();

       for i in 0..num_rows as usize {
           ret.push((1..i).fold(vec![1; i + 1], |mut acc, j| {
               acc[j] = ret[i-1][j-1] + ret[i-1][j];
               acc
           }));
       }

       ret
   }
}