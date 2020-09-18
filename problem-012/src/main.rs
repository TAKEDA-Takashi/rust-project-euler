//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2012
//!
//! 三角数の数列は自然数の和で表わされ, 7番目の三角数は 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28 である. 三角数の最初の10項は:
//!
//! 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//! となる.
//!
//! 最初の7項について, その約数を列挙すると, 以下のとおり.
//!
//!  1: 1
//!  3: 1,3
//!  6: 1,2,3,6
//! 10: 1,2,5,10
//! 15: 1,3,5,15
//! 21: 1,3,7,21
//! 28: 1,2,4,7,14,28
//!
//! これから, 7番目の三角数である28は, 5個より多く約数をもつ最初の三角数であることが分かる.
//!
//! では, 500個より多く約数をもつ最初の三角数はいくつか.

use itertools::Itertools;
use problem_012::PrimeFactorization;

fn get_triangular_number(n: usize) -> (usize, usize, usize) {
    if n % 2 == 0 {
        (n / 2, n + 1, (n / 2) * (n + 1))
    } else {
        ((n + 1) / 2, n, (n + 1) / 2 * n)
    }
}

fn divisor_count(ps: &Vec<usize>) -> usize {
    // 約数の個数は素因数の指数に+1をして総乗する
    ps.into_iter()
        .skip_while(|&&n| n == 1)
        .group_by(|&n| n)
        .into_iter()
        .map(|(_, group)| group.count() + 1)
        .product::<usize>()
}

fn main() {
    let find_div_count = 500;
    let mut prime_factorization = PrimeFactorization::new();

    let tri = (1..)
        .map(|n| get_triangular_number(n))
        .find(|(a, b, _)| {
            let mut ps = vec![];
            ps.extend(prime_factorization.exec(*a));
            ps.extend(prime_factorization.exec(*b));

            ps.sort();

            divisor_count(&ps) > find_div_count
        })
        .unwrap();

    // println!("{:?}", tri);
    println!("{}", tri.2);
}
