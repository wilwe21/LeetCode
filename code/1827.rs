impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ops: i32 = 0;
        let mut clon = nums.clone();
        println!("{:?}", clon);
        for i in 0..nums.len()-1 {
            while &clon[i+1] <= &clon[i] {
                clon[i+1] += 1;
                ops += 1;
            }
        }
        ops
    }
}
