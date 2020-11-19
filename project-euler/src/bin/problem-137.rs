//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20137
//! Fibonacci golden nuggets

// 与えられた級数を整理すると S = -x / (x^2 + x - 1)
// n > m, n ⊥ m, x = m / n としてさらに式を整理すると S = n, n^2 -mn -m^2 -m = 0 なので
// D = 5m^2 + 4m より、D = c^2 が満たすべき条件となる。

use num::integer::Roots;

fn main() {
    println!(
        "{}",
        (1_u128..)
            .map(|m| m * m)
            .filter_map(|m| {
                let d = 5 * m * m + 4 * m;
                let rd = d.sqrt();
                if rd * rd == d {
                    Some((m + rd) / 2)
                } else {
                    None
                }
            })
            .nth(14) // 0-origin
            .unwrap()
    );
}
