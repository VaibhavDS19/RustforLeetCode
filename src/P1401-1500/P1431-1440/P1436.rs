use std::collections::HashSet;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let source: HashSet<String> = paths.iter().map(|v| v[0].clone()).collect();
        let destination: HashSet<String> = paths.iter().map(|v| v[1].clone()).collect();
        destination.difference(&source).collect::<Vec<&String>>()[0].to_string()
    }
}