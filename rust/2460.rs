impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nu = nums.clone();
        for i in 0..nu.len()-1 {
            if nu[i] == nu[i+1] {
                nu[i] = nu[i]*2;
                nu[i+1] = 0;
            }
        }
        let z = nu.iter().filter(|&v| *v == 0 ).count();
        for i in 0..z {
            nu.remove(nu.iter().position(|&x| x==0).expect("number"));
            nu.push(0);
        }
        nu
    }
}
