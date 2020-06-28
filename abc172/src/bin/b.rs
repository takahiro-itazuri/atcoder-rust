use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut count = 0;

    for i in 0 .. s.len() {
        if s[i] != t[i] {
            count += 1;
        }
    }

    println!("{}", count);
}
