use proconio::*;

fn main() {
    input! {
        a: u32
    }

    println!("{}", a + a * a + a * a * a);
}
