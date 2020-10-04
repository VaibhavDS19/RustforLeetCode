use std::collections::HashSet;
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visited:HashSet<i32> = HashSet::new();
        let mut cur = 0;
        visited.insert(0);
        for i in path.chars() {
            if i=='N' {
                cur += 10_000;
            }
            else if i=='E' {
                cur += 1;
            }
            else if i=='S' {
                cur -= 10_000;
            }
            else if i=='W' {
                cur-=1;
            }
            if visited.contains(&cur) {
                return true;
            }
            visited.insert(cur);
        }
        false
    }
}