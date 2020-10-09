use std::collections::VecDeque;

impl Solution {
    pub fn final_prices(ref mut ns: Vec<i32>) -> Vec<i32> {
        let n = ns.len();
        let mut s = VecDeque::new();
        for i in 0..n {
            let p = ns[i];
            while !s.is_empty() && ns[*s.back().unwrap()] >= p {
                ns[s.pop_back().unwrap()] -= p;
            }
            s.push_back(i);
        }
        ns.to_vec()
    }
}