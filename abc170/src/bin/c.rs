use proconio::*;

fn main() {
    input! {
        x: i64,
        n: usize,
        p: [i64; n]
    }
    
    if n == 0 {
        println!("{}", x);
        return;
    }

    let mut flag: [bool; 102] = [false; 102];
    for i in 0..n {
        flag[p[i] as usize] = true;
    }

    let mut min_diff = 101;
    let mut ans = x;
    for i in 0..=101 {
        if flag[i] { continue; }
        let diff = (i as i64 - x).abs();
        if diff < min_diff {
            min_diff = diff;
            ans = i as i64;
        }
    }

    println!("{}", ans);
}
