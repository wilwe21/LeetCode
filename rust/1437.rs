impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
       let mut st: String = "".to_string();
       for i in nums{
            st += &i.to_string();
       } 
       let mut ar: Vec<_> = st.split("1").collect();
       if ar.len() > 1 {
            ar.remove(0);
            ar.remove(ar.len()-1);
            for mut i in ar {
                if k > i.len().try_into().unwrap() {
                    return false
                }
            }
       } else {
            for i in ar {
                if k > i.len().try_into().unwrap() {
                    return false
                }
            }
       }
       true
    }
}
