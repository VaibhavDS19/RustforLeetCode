impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut count=0;
        let mut max=0;
        let mut cur=']';
        for ch in s.chars() {
            if ch!=cur {
                cur=ch;
                if max<count {
                    max = count;
                }
                count=1;
            }
            else {
                count+=1;
            }
        }
        if count>max {
            return count;
        }
        max
    }
}