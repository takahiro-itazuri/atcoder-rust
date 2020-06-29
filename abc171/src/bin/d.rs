use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        bc: [(i64, i64); q]
    }

    const MAX_VAL: usize = 100001;
    let mut counter: [i64; MAX_VAL] = [0; MAX_VAL];
    let mut sum: i64 = 0;
    for i in 0..n {
        counter[a[i] as usize] += 1;
        sum += a[i];
    }

    for (b, c) in bc {
        sum += (c - b) * counter[b as usize];
        println!("{}", sum);

        counter[c as usize] += counter[b as usize];
        counter[b as usize] = 0;
    }
}
