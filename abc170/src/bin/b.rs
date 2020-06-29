use proconio::*;

fn main() {
    input! {
        x: i64,
        y: i64
    }

    let m = 4 * x - y;
    if m % 2 == 0 && 0 <= m && m <= 2 * x {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
