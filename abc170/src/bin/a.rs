use proconio::*;

fn main() {
    input! {
        x: [i64; 5]
    }

    for i in 0..5 {
        if x[i] == 0 {
            println!("{}", i + 1);
        }
    }
}
