use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    for &i in &a {
        if i == 0 {
            println!("0");
            return;
        }
    }

    let mut ans: i64 = 1;
    for &i in &a {
        if 1000000000000000000 / i < ans {
            println!("-1");
            return;
        }
        ans *= i;
    }
    println!("{}", ans);
}
