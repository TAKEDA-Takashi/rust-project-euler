//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2052
//!
//! 125874を2倍すると251748となる. これは元の数125874と順番は違うが同じ数を含む.
//!
//! 2x, 3x, 4x, 5x, 6x が x と同じ数を含むような最小の正整数 x を求めよ.

use itertools::Itertools;

fn main() {
    println!(
        "{}",
        (1..)
            .map(|n| {
                (
                    n,
                    n.to_string().chars().sorted().collect_vec(),
                    [2 * n, 3 * n, 4 * n, 5 * n, 6 * n],
                )
            })
            .find(|(_, n, ms)| ms
                .iter()
                .all(|m| *n == m.to_string().chars().sorted().collect_vec()))
            .unwrap()
            .0
    );
}
