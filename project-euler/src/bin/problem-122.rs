//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20120
//!
//! n15を求めるのに最も単純な方法では 14 回の掛け算が必要である.
//!
//! n × n × ... × n = n15
//!
//! しかし2進法を用いれば, 6 回の掛け算で計算できる.
//!
//! n × n = n2
//! n2 × n2 = n4
//! n4 × n4 = n8
//! n8 × n4 = n12
//! n12 × n2 = n14
//! n14 × n = n15
//!
//! ところがたった 5 回の掛け算のみでも計算できる.
//!
//! n × n = n2
//! n2 × n = n3
//! n3 × n3 = n6
//! n6 × n6 = n12
//! n12 × n3 = n15
//!
//! m(k)を nk を求めるのに必要最低限な掛け算の回数と定義する. たとえば m(15)=5 である.
//!
//! 1 ≤ k ≤ 200 に対し, Σ m(k) を求めよ.

use std::collections::HashMap;
use std::iter::once;

const UBOUND: usize = 200;

fn main() {
    let mut opt_pow_table = HashMap::new();
    opt_pow_table.insert(2, vec![vec![2]]);

    for n in 3..=UBOUND {
        let mut min_len = usize::MAX;
        let mut min_vecs = vec![];

        for i in (n + 1) / 2..n {
            for v in &opt_pow_table[&i] {
                if v.len() + 1 > min_len {
                    break;
                }

                let last = v.last().unwrap();

                if once(1).chain(v.iter().copied()).any(|m| n == last + m) {
                    let w = v.iter().copied().chain(once(n)).collect::<Vec<_>>();

                    if w.len() == min_len {
                        min_vecs.push(w);
                    } else if w.len() < min_len {
                        min_len = w.len();
                        min_vecs = vec![w];
                    }
                }
            }
        }

        opt_pow_table.insert(n, min_vecs);
    }

    // println!("{:?}", opt_pow_table);
    println!(
        "{}",
        opt_pow_table.values().map(|vs| vs[0].len()).sum::<usize>()
    );
}
