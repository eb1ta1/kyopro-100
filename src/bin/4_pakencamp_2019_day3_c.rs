use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; m]; n],
    }
    let mut max_res = 0;
    for t1 in 0..m-1 {
        for t2 in t1+1..m {
            let mut total_score = 0;
            for s in 0..n {
                total_score += max(a[s][t1], a[s][t2]);
            }
        max_res = max(max_res, total_score);
        }
    }
    println!("{}", max_res);
}