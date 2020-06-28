use proconio::*;

fn lower_bound<F>(l: usize, r: usize, criteria: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut l = l;
    let mut r = r;
    while r - l > 1 {
        let mid = l + (r - l) / 2;
        if criteria(mid) {
            r = mid;
        } 
        else {
            l = mid;
        }
    }
    return l;
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
        a: [u64; n],
        b: [u64; m]
    }
    
    let mut sum_a = vec![0; n + 1];
    let mut sum_b = vec![0; m + 1];

    for i in 0 .. n {
        sum_a[i + 1] = sum_a[i] + a[i];
    }

    for i in 0 .. m {
        sum_b[i + 1] = sum_b[i] + b[i];
    }

    let mut ans = 0;

    for i in 0..=n {
        if sum_a[i] > k { break; }

        let criteria = |j: usize| -> bool {
            sum_a[i] + sum_b[j] > k
        };
        let j = lower_bound(0, m + 1, criteria);

        ans = std::cmp::max(ans, i + j);
    }

    println!("{}", ans);
}
