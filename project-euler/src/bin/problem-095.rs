//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2095
//!
//! ある数の真の約数とは, それ自身を除く約数すべてである. 例えば, 28 の真の約数は 1, 2, 4, 7, 14 である. これらの約数の和は 28 に等しいため, これを完全数と呼ぶ.
//!
//! 面白いことに, 220 の真の約数の和は 284 で, 284 の真の約数の和は 220 となっており, 二つの数が鎖をなしている. このため, 220 と 284 は友愛数と呼ばれる.
//!
//! さらに長い鎖はあまり知られていないだろう. 例えば, 12496 から始めると, 5 つの数の鎖をなす.
//!
//! 12496 → 14288 → 15472 → 14536 → 14264 (→ 12496 → ...)
//! この鎖は出発点に戻っているため, 友愛鎖と呼ばれる.
//!
//! いずれの要素も 1,000,000 を超えない最長の友愛鎖の最小のメンバーを求めよ.

use std::collections::HashSet;

const UBOUND: usize = 1_000_000;

fn main() {
    let mut divisors_sum_table = [1; UBOUND + 1];

    for n in 2..=UBOUND / 2 {
        for m in 2..=UBOUND / n {
            divisors_sum_table[n * m] += n;
        }
    }

    let mut chain_stop_numbers = HashSet::new();
    chain_stop_numbers.insert(1);

    let mut maximum_amicable_chain = vec![];

    for mut n in 2..UBOUND {
        if chain_stop_numbers.contains(&divisors_sum_table[n]) {
            continue;
        }

        let mut amicable_chain = vec![n];
        chain_stop_numbers.insert(n);

        loop {
            let m = divisors_sum_table[n];
            if m > UBOUND {
                break;
            }

            if chain_stop_numbers.contains(&m) {
                if let Some(start_index) = amicable_chain.iter().position(|&e| e == m) {
                    if amicable_chain.len() - start_index > maximum_amicable_chain.len() {
                        maximum_amicable_chain =
                            amicable_chain[start_index..].iter().copied().collect();
                    }
                }
                break;
            } else {
                amicable_chain.push(m);
                chain_stop_numbers.insert(m);

                n = m;
            }
        }
    }

    println!("{:?}", maximum_amicable_chain);
    println!("{:?}", maximum_amicable_chain.iter().min());
}
