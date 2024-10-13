impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string().chars().rev().collect::<String>() == x.to_string()
    }
}
