use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    a.sort();
    let maxval = a[a.len() - 1];

    let mut ans = 0;
    let mut arr: [bool; 1000001] = [false; 1000001];
    for i in 0..n {
        let mut val = a[i];

        if i + 1 < n && a[i] == a[i + 1] {
            arr[val] = true;
        }
        else if arr[val] == false {
            ans += 1;
        }

        val += a[i];
        while val <= maxval {
            arr[val] = true;
            val += a[i];
        }
    }

    println!("{}", ans);
}
