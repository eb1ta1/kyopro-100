// rust
use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let mut total_divs = 0;
    for i in (1..=n).step_by(2) {
        let mut divs = 0; 
        for div in 1..i {
            if i % div == 0 && (div*div) < i{
                divs += 2;
            }
        }
        if divs == 8 {
            total_divs += 1
        }
    }
    println!("{}", total_divs)
}