fn rev(n: String) -> String {
    let mut b = String::new();
    for i in n.chars().rev() {
        if i == '0' {
            b += "1";
        } else {
            b += "0";
        }
    }
    b
}
impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut s = Vec::new();
        for i in 0..n {
            if i == 0 {
                s.push("0".to_string());
            } else {
                let sc = s.clone();
                let f = format!("{}1{}",sc[(i-1) as usize],rev(sc[(i-1) as usize].to_string())).to_string();
                s.push(f.clone());
            }
        }
        let e = &s[s.clone().len()-1 as usize].chars().collect::<Vec<_>>();
        e[(k-1) as usize]
    }
}
