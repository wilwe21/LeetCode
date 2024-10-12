// failed
impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut char = 0;
        for i in 0..times.len() {
            if times[i][0] < times[target_friend as usize][0] {
                char += 1
            }
            if times[i][1] < times[target_friend as usize][0]{
                char -= 1
            }
        }
        char
    }
}
