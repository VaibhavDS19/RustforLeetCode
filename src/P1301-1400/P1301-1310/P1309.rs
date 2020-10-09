impl Solution {
   pub fn freq_alphabets(s: String) -> String {
      let mut ans = String::new();
      if s.is_empty() {
         return ans;
      }
      let mut v = s.chars().collect::<Vec<char>>();
      while !v.is_empty() {
         let ch = v.pop().unwrap();
         if ch == '#' {
            let mut ch2 = v.pop().unwrap();
            let mut u = ch2 as u8 - '0' as u8;
            ch2 = v.pop().unwrap();
            u += (ch2 as u8 - '1' as u8) * 10;
            ans.insert(0, (u + 'j' as u8) as char);
         } else {
            ans.insert(0, (ch as u8 - '1' as u8 + 'a' as u8) as char)
         }
      }
      ans
   }
}