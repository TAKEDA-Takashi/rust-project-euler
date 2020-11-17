//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20132
//!
//! 1のみからなる数をレピュニットという. R(k) を長さ k のレピュニットとする.
//!
//! 例えば, R(10) = 1111111111 = 11×41×271×9091 となり, 素因数の和は9414となる.
//!
//! R(109) の最初の40個の素因数の和を求めよ.

// R(n) は nの約数をm とすると R(m) で割り切れる

use euler_lib::{get_divisors, Prime};
use itertools::repeat_n;
use num::{BigUint, Zero};
use std::collections::{BTreeSet, HashMap};

fn main() {
    let prime = Prime::<u128>::new();

    let divisors = vec![
        2, 4, 5, 8, 10, 16, 20, 25, 32, 40, 50, 64, 80, 100, 125, 128, 160, 200, 250, 256, 320,
        400, 500, 512, 625, 640, 800, 1000, 1250, 1280, 1600, 2000, 2500, 2560, 3125, 3200, 4000,
        5000, 6250, 6400, 8000, 10000, 12500, 12800, 15625, 16000, 20000, 25000, 31250, 32000,
        40000, 50000, 62500,
    ];

    let mut rp = BTreeSet::new();
    let mut table = HashMap::new();

    for d in divisors {
        let last = *get_divisors(&d).last().unwrap();
        let mut r = repeat_n("1", d)
            .collect::<String>()
            .parse::<BigUint>()
            .unwrap();

        if table.contains_key(&last) {
            r /= repeat_n("1", last)
                .collect::<String>()
                .parse::<BigUint>()
                .unwrap();
        }

        table.insert(d, r);
    }

    for p in prime.iter() {
        for (_, v) in &mut table {
            if (v.clone() % p).is_zero() {
                rp.insert(p);
                *v = v.clone() / p;
            }
        }

        if rp.len() == 40 {
            break;
        }
    }

    println!("{}", rp.iter().sum::<BigUint>());
}
