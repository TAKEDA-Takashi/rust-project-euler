//! http://odz.sakura.ne.jp/projecteuler/index.php?Problem%20111
//! Primes with runs

use euler_lib::Prime;
use itertools::{repeat_n, Itertools};
use std::iter::once;

fn main() {
    let digit: usize = 10;
    // 1桁目はこの4種のみ
    let d1 = vec![1, 3, 7, 9];
    let mut prime = Prime::<u128>::new();

    let mut discover = repeat_n(false, 10).collect_vec();
    let mut v = vec![];

    for h in 1..=9 {
        for &d in &d1 {
            // 間が全部0
            let num = h * 10_u128.pow((digit - 1) as u32) + d;
            if prime.is_prime(&num) {
                v.push(num);
            }

            // 先頭から同じ数字が並んでいる
            let s = repeat_n(h.to_string(), digit - 1)
                .chain(once(d.to_string()))
                .collect::<String>();

            let num = s.parse().unwrap();
            if prime.is_prime(&num) {
                v.push(num);
                discover[h as usize] = true;
            }
        }
    }

    for &n in &d1 {
        find_and_set_prime(digit, n, n, &mut prime, &mut v, &mut discover);
    }

    for n in 1..=9 {
        if discover[n] {
            continue;
        }

        for &d in &d1 {
            find_and_set_prime(digit, n as u128, d, &mut prime, &mut v, &mut discover);
        }
    }

    // println!("{:?}", v);
    println!("{}", v.iter().sum::<u128>());
}

fn find_and_set_prime(
    digit: usize,
    n: u128,
    first_n: u128,
    prime: &mut Prime<u128>,
    v: &mut Vec<u128>,
    discover: &mut Vec<bool>,
) {
    let mut nvec = repeat_n(n, digit).collect_vec();
    nvec[digit - 1] = first_n;

    for i in 0..digit - 1 {
        for m in 0..=9 {
            if i == 0 && m == 0 {
                continue;
            }

            nvec[i] = m;

            let num = nvec.iter().join("").parse().unwrap();
            if prime.is_prime(&num) {
                v.push(num);
                discover[n as usize] = true;
            }
        }

        nvec[i] = n;
    }
}
