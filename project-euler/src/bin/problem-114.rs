//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20114
//!
//! 長さ 7 ユニットからなる 1 列上に, 最低 3 ユニットの長さを持つ赤ブロックが置かれている. ただしどの赤ブロック同士も, 少なくとも 1 ユニットの黒い正方形が間にある(赤ブロックは長さが異なってもよい). これを敷き詰める方法は, ちょうど 17 通りある.
//!
//! 114_1.png
//! 50 ユニットの長さの 1 列を敷き詰める方法は何通りあるか.
//!
//! 注意: 上の例では起こりえないが, 通常はブロックの大きさが複数混ざっていてもよい. 例えば, 8 ユニットの長さの 1 列では, 赤(3), 黒(1), 赤(4) を使うことができる.

use std::collections::HashMap;

fn main() {
    let m = 3;
    let n = 50;

    let mut memo = HashMap::new();

    println!("{}", block_combinations(m, n, &mut memo));
}

fn block_combinations(m: usize, n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    fn block_combinations0(m: usize, n: usize, memo: &mut HashMap<usize, usize>) -> usize {
        if m > n {
            return 0;
        } else if m == n {
            return 1;
        } else if memo.contains_key(&n) {
            return *memo.get(&n).unwrap();
        }

        let sum = (m..=(n - (m + 1)))
            .map(|k| {
                (block_combinations0(m, k, memo) - block_combinations0(m, k - 1, memo))
                    * ((n - m - k) * (n - m - k + 1))
                    / 2
            })
            .sum::<usize>()
            + (n - m + 1) * (n - m + 2) / 2;

        memo.insert(n, sum);

        sum
    }

    block_combinations0(m, n, memo) + 1
}
