impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let mut string_set:Vec<&str> = sentence.split(" ").collect();
        for i in 0..string_set.len() {
            if string_set[i].starts_with(&search_word) {
                return i as i32 + 1;
            }
        }
        -1
    }
}