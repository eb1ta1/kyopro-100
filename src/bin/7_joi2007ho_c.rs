use proconio::input;
use std::f64;

fn length(p1:Vec<usize>,p2:Vec<usize>) -> f64 {
    let x1 = p1[0] as f64;
    let x2 = p2[0] as f64;
    let y1 = p1[1] as f64;
    let y2 = p2[1] as f64;
    let from = (x1-x2)*(x1-x2) + (y1-y2)*(y1-y2);
    from.sqrt()
}

fn main() {
    input! {
        n: usize,
        trees: [[usize; 2]; n],
    }
    for ni in 0..n-3 {
        let vec_i = &trees[ni];
        for j in ni+1..n-2 {
            let vec_j = &trees[j];
            for k fdc xddc in j+1..n-1 {
                let vec_k = &trees[k];
                for l in k+1..n {
                    let vec_l = &trees[l];

                    let ij = length(vec_i.to_vec(), vec_j.to_vec());
                    let jk = length(vec_j.to_vec(), vec_k.to_vec());
                    let _kl = length(vec_k.to_vec(), vec_l.to_vec());
                    let ki = length(vec_k.to_vec(), vec_i.to_vec());
                    let jl = length(vec_j.to_vec(), vec_l.to_vec());
                    let kl = length(vec_k.to_vec(), vec_i.to_vec());

                    if ij == jk && jk == kl && kl == ki && ki == ij && jl == ki {
                        let area = ij*ij;
                        println!("{}", area)
                    }  
                }
            }
        }
    }
}