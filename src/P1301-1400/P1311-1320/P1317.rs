impl Solution {
   pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
       fn contains_zero(i: i32) -> bool {
           let s = i.to_string();
           s.contains('0')
       }
       for i in 1..=n / 2 {
           if contains_zero(i) || contains_zero(n - i) {
               continue;
           } else {
               return vec![i, n - i];
           }
       }
       vec![0, 0] //unreachable!()
   }
}