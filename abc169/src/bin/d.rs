use proconio::*;

fn main() {
    input! {
        mut n: i64
    }

    if n == 1 {
        println!("0");
        return;
    }

    let mut ans = 0;
    let maxval = (n as f64).sqrt().ceil() as i64;
    for i in 2..=maxval {
        let mut cnt = 0;
        while n % i == 0 {
            n /= i;
            cnt += 1;
        }

        /*
        if cnt != 0 { println!("{}, {}, {}", i, cnt, n); }
        */

        let mut a = 1;
        while cnt >= a {
            cnt -= a;
            a += 1;
            ans += 1;
        }
    }

    if n != 1 { ans += 1; }

    if ans == 0 { println!("1"); }
    else { println!("{}", ans); }
}
