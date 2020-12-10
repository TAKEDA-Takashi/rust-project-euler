//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20146
//!
//! n2+1, n2+3, n2+7, n2+9, n2+13, n2+27 が連続する素数になる最小の正整数 n は 10 である. 100万未満の n の総和は 1242490 である.
//!
//! それでは, 1億5000万未満の n の総和を求めよ.

use euler_lib::is_prime;
use rayon::prelude::*;

fn main() {
    let prime_addon = [1, 3, 7, 9, 13, 27];
    let not_prime_addon = [19, 21];

    let sum = (1..15_000_000_u128)
        .into_par_iter()
        .filter(|n| n % 3 != 0 && n % 7 != 0 && n % 13 != 0)
        .map(|n| n * 10)
        .map(|n| {
            let n2 = n * n;
            if prime_addon.par_iter().all(|m| is_prime(&(n2 + m)))
                && not_prime_addon.par_iter().all(|m| !is_prime(&(n2 + m)))
            {
                // println!("{}", n);
                n
            } else {
                0
            }
        })
        .sum::<u128>();

    println!("{}", sum);
}
