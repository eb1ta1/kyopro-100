use std::usize;

fn main() {
    let n: usize = {
        let mut line: String = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };

    let a: Vec<i64> = {
        let mut line: String = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    let _q: usize = {
        let mut line: String = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };

    let m: Vec<i64> = {
        let mut line: String = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let mut nums = Vec::new();
    for i in 0..(2_i32.pow(n as u32)) {
        let mut num = 0;
        for j in 0..n {
            if (i>>j) & 1 == 1 {
                num += a[j];
                nums.push(num)
            }
        }
    }
    for mi in &m {
        if nums.contains(&mi) {
            println!("yes")
        } else {
            println!("no")
        }
    }
}