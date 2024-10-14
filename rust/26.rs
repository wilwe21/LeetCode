impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut l: Vec<_> = vec!();
        for i in nums.into_iter() {
            if !(l.contains(i)) {
                l.push(*i as i32);
            }
        }
        nums.clear();
        for i in l {
            nums.push(i);
        }
        nums.len() as i32
    }
}
