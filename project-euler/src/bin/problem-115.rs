//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20115
//!
//! 注意: これは Problem 114 をより難しくした問題である.
//!
//! 長さ n ユニットからなる 1 列上に, 最低 m ユニットの長さを持つ赤ブロックが置かれている. ただしどの赤ブロック同士も, 少なくとも 1 ユニットの黒い正方形が間にある(赤ブロックは長さが異なってもよい).
//!
//! 敷き詰め計数関数 F(m, n) は 1 列に敷き詰める方法が何通りかを表すとする.
//!
//! 例えば, F(3, 29) = 673135 であり, F(3, 30) = 1089155 である.
//!
//! m = 3 の時, n = 30 がこの敷き詰め計数関数が初めて 1,000,000 を超える最小の値であることがわかる.
//!
//! 同様に, m = 10 では F(10, 56) = 880711, F(10, 57) = 1148904 であることがわかり, つまり n = 57 がこの敷き詰め計数関数が初めて 1,000,000 を超える最小の値であることがわかる.
//!
//! m = 50 のとき, この敷き詰め計数関数が初めて 1,000,000 を超える最小の n の値を求めよ.

use std::collections::HashMap;

fn main() {
    let m = 50;
    let mut memo = HashMap::new();

    for n in 50.. {
        if block_combinations(m, n, &mut memo) > 1_000_000 {
            println!("{}", n);
            break;
        }
    }
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
