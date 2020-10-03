impl Solution {
    pub fn thousand_separator(mut n: i32) -> String {
        let mut ans = String::new();
        let mut count = 0;
        while n > 0 {
            ans.insert(0, (b'0' + (n % 10) as u8) as char);
            n /= 10;
            count += 1;
            if count == 3 && n > 0 {
                ans.insert(0, '.');
                count = 0;
            }
        }
        if ans.is_empty() {
            ans.push('0')
        }
        ans
    }
}