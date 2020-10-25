impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let mut s = String::new();
        let t = text.len();
        let mut v:Vec<&str> = text.split_whitespace().collect();
        let mut l=0;
        for i in &v {
            l += i.len();
        }
        if v.len()==1 {
            s = v[0].to_string();
            if l<t {
                s = s+ &" ".repeat(t-l)
            }
            return s;
        }
        let words = (t-l)/( v.len()-1 );
        let spaces=" ".repeat(words);
        let words = v.len() -1;
        let mut j=0;
        for j in 0..words {
            s = s + v[j] + &spaces;
        }
        s =s + v[v.len()-1];
        l = s.len();
        if l<t {
            s = s + &" ".repeat(t-l);
        }
        s
    }
}