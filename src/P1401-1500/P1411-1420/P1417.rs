impl Solution {
    pub fn reformat(s: String) -> String {
        let mut t = String::new();
        let mut l:Vec<char> = Vec::new();
        let mut d:Vec<char> = Vec::new();
        for ch in s.chars() {
            if ch>='a' && ch<='z' {
                l.push(ch);
            }
            else {
                d.push(ch);
            }
        }
        if l.len() as i32 -d.len() as i32 >=2 || d.len() as i32 - l.len() as i32 >=2 {
            return t;
        }
        let mut m = d.len();
        let mut n = l.len();
        if m>n {
            while m>0 {
                t.push(d[m-1]);
                m-=1;
                if n>0 {
                    t.push(l[n-1]);
                    n-=1;
                }
            }
        }
        else {
            while n>0 {
                t.push(l[n-1]);
                n-=1;
                if m>0 {
                    t.push(d[m-1]);
                    m-=1;
                }
            }
        }
        t
    }
}