use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        ab_vec: [[i128; 2]; n]
    }
    let mut xyvec = Vec::new();
    for i in &ab_vec {
        for j in &ab_vec {
            xyvec.push([i[0],j[1]]);
        }
    }    
    let mut res: i128 = 0;
    for p in xyvec {
        let mut count: i128 = 0; 
        for n in &ab_vec {
            count = count + (p[0]-n[0]).abs() + (p[1]-n[1]).abs() + (n[1] - n[0]).abs();
        }
        if res != 0 {
            res = min(res, count);
        } else {
            res = count;
        }
    }
    println!("{}", res)
}
