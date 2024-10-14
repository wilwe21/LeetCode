impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let a: Vec<_> = s.split_whitespace().collect();
        a[a.len()-1].len() as i32
    }
}
