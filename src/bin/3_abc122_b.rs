use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut res = 0;
    let mut resvec = vec![0];
    let agct = ["A", "C", "G", "T"];
    let str: Vec<char> = s.chars().collect();
    for i in str {
        let i_string = i.to_string();
        let i_str: &str = &i_string;
        if agct.contains(&i_str) {
            res += 1;
            resvec.push(res);
        } else {
            res = 0;
        }
    }
    resvec.sort();
    resvec.reverse();
    println!("{}", resvec[0])
}