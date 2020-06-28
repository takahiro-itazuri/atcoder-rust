use proconio::*;

fn main() {
    input! {
        n: u64
    }

    let mut ans = 0;

    for i in 1..=n {
        let m = n / i;
        ans += i * (1 + m) * m / 2;
    }

    println!("{}", ans);
}
