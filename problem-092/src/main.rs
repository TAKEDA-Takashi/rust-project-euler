//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2092
//!
//! 各桁の2乗を足し合わせて新たな数を作ることを, 同じ数が現れるまで繰り返す.
//!
//! 例えば
//!
//! 　　44 → 32 → 13 → 10 → 1 → 1
//! 　　85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89
//!
//! のような列である. どちらも1か89で無限ループに陥っている.
//! 驚くことに, どの数から始めても最終的に1か89に到達する.
//!
//! では, 10,000,000より小さい数で89に到達する数はいくつあるか.

use std::collections::HashSet;

fn main() {
    let ubound = 10_000_000;
    let mut finish1: HashSet<u32> = HashSet::new();
    let mut finish89: HashSet<u32> = HashSet::new();

    finish1.insert(1);
    finish89.insert(89);

    let mut queue = vec![];

    for mut n in 2..ubound {
        loop {
            let sum = n
                .to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|a| a * a)
                .sum::<u32>();

            if finish1.contains(&sum) {
                finish1.insert(n);
                if !queue.is_empty() {
                    finish1.extend(&queue);
                    queue.clear();
                }
                break;
            } else if finish89.contains(&sum) {
                finish89.insert(n);
                if !queue.is_empty() {
                    finish89.extend(&queue);
                    queue.clear();
                }
                break;
            } else {
                queue.push(n);
                n = sum;
            }
        }
    }

    println!("{}", finish89.len());
}
