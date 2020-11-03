//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20106
//!
//! 大きさ n の集合 A の要素の和を S(A) で表す. 空でなく共通要素を持たないいかなる 2 つの部分集合 B と C に対しても以下の性質が真であれば, A を特殊和集合と呼ぼう.
//!
//! i. S(B) ≠ S(C); つまり, 部分集合の和が等しくてはならない.
//! ii. B が C より多くの要素を含んでいればこのとき S(B) > S(C) となる.
//!
//! 本問題に対しては, 与えられた集合は n 個の単調増加する要素を含み, かつ第二のルールをすでに満たしているものと仮定しよう.
//!
//! 驚くべきことに, n = 4 の集合から得ることができる 25 個の可能な部分集合の対のうち, 1 個の対のみが 同一性（第一のルール）をテストされる必要がある. 同様に, n = 7 のときは, 966 個の部分集合の対のうち 70 個のみがテストされる必要がある.
//!
//! n = 12 に対して, 得られる 261625 個の部分集合の対のうち, 同一性をテストされる必要があるものは何個か.
//!
//! 注意: この問題は Problem 103 と 105 に関連している.

use euler_lib::combination;
use itertools::Itertools;
use std::collections::BTreeSet;

fn main() {
    let n = 12;
    let need_test = (2..=n / 2)
        .map(|c| (combination(&n, &c) * combination(&(n - c), &c) - check_count(n, c) * 2) / 2)
        .sum::<usize>();

    println!("{}", need_test);
}

fn check_count(n: usize, c: usize) -> usize {
    let set: BTreeSet<_> = (0..n).collect();

    set.iter()
        .copied()
        .combinations(c)
        .map(|p| {
            (&set - &p.iter().copied().collect::<BTreeSet<_>>())
                .into_iter()
                .combinations(c)
                .filter(|q| p.iter().zip(q).all(|(a, b)| a < b))
                .count()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        // 4C2 = 6通り。
        // (4-2)C2 = 1なので、排他的なペアの選び方は 6 * 1 = 6通り（半分は重複）。
        // check_count(4, 2) => 2。そのうち4通り（2 * 2）が順序が一定になっているので 6 - 4 = 2通り。
        // 2 / 2 = 1通りがチェックすべき組み合わせ。

        assert_eq!(
            1,
            (combination(&4, &2) * combination(&(4 - 2), &2) - check_count(4, 2) * 2) / 2
        );
    }

    #[test]
    fn test_7() {
        // 7C2 = 21通り。
        // (7-2)C2 = 10なので、排他的なペアの選び方は 21 * 10 = 210通り（半分は重複）。
        // check_count(7, 2) => 70。そのうち140通り（70 * 2）が順序が一定になっているので 210 - 140 = 70通り。
        // 70 / 2 = 35通りがチェックすべき組み合わせ。

        // 7C3 = 35通り。
        // (7-3)C3 = 4なので、排他的なペアの選び方は 35 * 4 = 140通り（半分は重複）。
        // check_count(7, 3) => 35。そのうち70通り（35 * 2）が順序が一定になっているので 140 - 70 = 70通り。
        // 70 / 2 = 35通りがチェックすべき組み合わせ。

        let n = 7;
        let need_test = (2..=n / 2)
            .map(|c| (combination(&n, &c) * combination(&(n - c), &c) - check_count(n, c) * 2) / 2)
            .sum::<usize>();

        assert_eq!(70, need_test);
    }
}
