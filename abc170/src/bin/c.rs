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

    let mut counter: [i64; 101] = [0; 101];
    for i in 0..n {
        counter[p[i] as usize] += 1;
    }

    let mut ans = None;
    for i in 0..=100 {
        for sign in [-1, 1].iter() {
            let val = x + sign * i as i64;
            if 1 <= val && val <= 100 {
                if counter[val as usize] > 0 { continue; }
            }

            ans = Some(val);
            break;
        }
        if ans != None { break; }
    }
    println!("{}", ans.unwrap());
}
