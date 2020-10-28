//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2088
//!
//! 少なくとも2つの自然数 {a1, a2, ... , ak} の集合の和かつ積として表せる自然数Nを積和数と呼ぶ：N = a1 + a2 + ... + ak = a1 × a2 × ... × ak.
//!
//! 例えば, 6 = 1 + 2 + 3 = 1 × 2 × 3.
//!
//! ある集合の大きさ k に対して,この性質を持つ最小の N を最小積和数と呼ぼう. 集合の大きさ k = 2, 3, 4, 5, 6 に対する最小積和数は次のとおりである.
//!
//! k=2: 4 = 2 × 2 = 2 + 2
//! k=3: 6 = 1 × 2 × 3 = 1 + 2 + 3
//! k=4: 8 = 1 × 1 × 2 × 4 = 1 + 1 + 2 + 4
//! k=5: 8 = 1 × 1 × 2 × 2 × 2 = 1 + 1 + 2 + 2 + 2
//! k=6: 12 = 1 × 1 × 1 × 1 × 2 × 6 = 1 + 1 + 1 + 1 + 2 + 6
//!
//! したがって 2 ≤ k ≤ 6 に対して,全ての最小積和数の和は 4+6+8+12 = 30 である. 8 は和に一度だけカウントされていることに気をつけよう.
//!
//! 実際, 2 ≤ k ≤ 12 に対する最小積和数の完全な集合は {4, 6, 8, 12, 15, 16} なので,その和は 61 である.
//!
//! 2 ≤ k ≤ 12000 に対する全ての最小積和数の和は何か?

use std::collections::{HashMap, HashSet};

fn main() {
    let ubound = 12000;
    let mut map: HashMap<usize, usize> = HashMap::new();

    for n in 2.. {
        for ys in division(n) {
            let k = n - ys.iter().sum::<usize>() + ys.len();

            if k <= ubound && !map.contains_key(&k) {
                map.insert(k, n);
            }
        }

        // exclude 1
        if map.len() == ubound - 1 {
            break;
        }
    }

    println!(
        "{}",
        map.values()
            .collect::<HashSet<_>>()
            .iter()
            .copied()
            .sum::<usize>()
    );
}

fn division(n: usize) -> Vec<Vec<usize>> {
    fn division0(ns: &Vec<usize>) -> Vec<Vec<usize>> {
        let n = ns[ns.len() - 1];
        (ns[ns.len() - 2]..)
            .take_while(|&m| m * m <= n)
            .filter(|m| n % m == 0)
            .map(|m| {
                ns[0..ns.len() - 1]
                    .iter()
                    .chain([m, n / m].iter())
                    .copied()
                    .collect()
            })
            .flat_map(|v| {
                let v2 = division0(&v);
                let mut wrap = vec![v];
                wrap.extend(v2);
                wrap
            })
            .collect()
    }

    (2..)
        .take_while(|&m| m * m <= n)
        .filter(|m| n % m == 0)
        .map(|m| vec![m, n / m])
        .flat_map(|v| {
            let v2 = division0(&v);
            let mut wrap = vec![v];
            wrap.extend(v2);
            wrap
        })
        .collect()
}
