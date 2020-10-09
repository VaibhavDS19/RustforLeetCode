impl Solution {
   pub fn maximum69_number (num: i32) -> i32 {
       let s = num.to_string();
       let mut c = String::new();
       let mut d = false;
       for i in s.chars() {
           if i=='6' && !d {
               c.push('9');
               d=true;
           }
           else {
               c.push(i);
           }
       }
       c.parse::<i32>().unwrap()
   }
}