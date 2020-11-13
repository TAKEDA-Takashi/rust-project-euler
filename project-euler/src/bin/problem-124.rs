//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20124
//! Ordered radicals

use euler_lib::Prime;
use itertools::Itertools;

fn main() {
    let ubound = 100_000;

    let prime = Prime::<usize>::new();

    println!(
        "{:?}",
        (1..=ubound)
            .map(|n| (prime.factorization(&n).iter().dedup().product::<usize>(), n))
            .sorted()
            .nth(10000 - 1) // 0-origin
    );
}
