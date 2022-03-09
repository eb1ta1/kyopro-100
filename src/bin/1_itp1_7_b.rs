use proconio::input;

fn main() {
    let mut flag = true;
    while flag {
        input! {
            l: [i32; 2],
        };
        let mut cnt = 0;
        let n = l[0];
        let x = l[1];
        if n == 0 && x == 0 {
            flag = false;
        } else {
            for i in 1..(n-1) {
                for j in (i+1)..n {
                    for k in (j+1)..(n+1) {
                        if (i+j+k) == x {
                            cnt += 1
                        }
                    }
                }
            }
        println!("{}", cnt);
        }
    }
}