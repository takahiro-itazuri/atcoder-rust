use proconio::*;

fn solve(n: u64) {
    if n > 26 {
        solve((n - 1) / 26);
    }

    print!("{}", ((('a' as u64) + (n - 1) % 26) as u8) as char);
}

fn main() {
    input! {
        n: u64
    }

    solve(n);
}
