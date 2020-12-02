//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20143
//! Investigating the Torricelli point of a triangle

// フェルマー点は120°のため余弦定理から(a, b, c)を求める。

use num::integer::Roots;
use std::collections::HashSet;

fn main() {
    let ubound: u128 = 120_000;
    let mut set = HashSet::new();

    for r in 1..ubound / 3 {
        for q in r..(ubound - r) / 2 {
            let a2 = r * r + q * q + r * q;
            let a = a2.sqrt();

            if a * a == a2 {
                for p in q..=ubound - (r + q) {
                    let b2 = q * q + p * p + q * p;
                    let b = b2.sqrt();

                    if b * b == b2 {
                        let c2 = p * p + r * r + p * r;
                        let c = c2.sqrt();

                        if c * c == c2 {
                            // println!("{} {} {}: {} {} {}", r, q, p, a, b, c);
                            set.insert(r + q + p);
                        }
                    }
                }
            }
        }
    }

    println!("{}", set.iter().sum::<u128>());
}
