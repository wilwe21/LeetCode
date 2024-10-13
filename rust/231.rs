impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false
        }
        if n == 2_i32.pow(n.ilog(2)) {
            return true
        }
        return false
    }
}
