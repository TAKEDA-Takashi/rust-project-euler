//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2049
//!
//! 項差3330の等差数列1487, 4817, 8147は次の2つの変わった性質を持つ.
//!
//! (i)3つの項はそれぞれ素数である.
//! (ii)各項は他の項の置換で表される.
//! 1, 2, 3桁の素数にはこのような性質を持った数列は存在しないが, 4桁の増加列にはもう1つ存在する.
//!
//! それではこの数列の3つの項を連結した12桁の数を求めよ.

use euler_lib::Prime;
use itertools::Itertools;
use std::collections::BTreeSet;

fn main() {
    let set: BTreeSet<usize> = Prime::new()
        .iter()
        .skip_while(|&p| p < 1000)
        .take_while(|&p| p < 10000)
        .collect();

    println!(
        "{}",
        set.iter()
            .tuple_combinations()
            .map(|(&a, &b)| [a, b, b + b - a])
            .filter(|[.., c]| set.contains(c))
            .filter(|[a, b, c]| {
                let (x, y, z) = (
                    a.to_string().chars().sorted().collect::<Vec<_>>(),
                    b.to_string().chars().sorted().collect::<Vec<_>>(),
                    c.to_string().chars().sorted().collect::<Vec<_>>(),
                );

                x == y && y == z
            })
            .find(|&[a, ..]| a != 1487)
            .unwrap()
            .iter()
            .join("")
    );
}
