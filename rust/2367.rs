impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                for k in j..nums.len() {
                    if nums[j] - nums[i] == diff && nums[k] - nums[j] == diff {
                        count += 1;
                    }    
                }
            }
        }
        count
    }
}
