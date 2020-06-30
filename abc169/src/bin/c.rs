use proconio::*;

fn main() {
    input! {
        a: i64,
        b: f64
    }

    let b = (b * 100.0).round() as i64;
    println!("{}", (a * b) / 100);
}
