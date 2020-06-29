use proconio::*;

fn main() {
    input! {
        a: char
    }

    if a >= 'A' && a <= 'Z' { println!("A"); }
    else if a >= 'a' && a <= 'z' { println!("a"); }
}
