impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut ch:Vec<char> = s.chars().collect();
        let mut res:Vec<char> = s.chars().collect();
        let mut j:usize = 0;
        for i in indices {
            res[i as usize] = ch[j];
            j+=1;
        }
        res.into_iter().collect()
    }
}