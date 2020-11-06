//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20109
//! Darts

use std::collections::{HashMap, HashSet};
use std::iter::once;

fn main() {
    let single: HashSet<_> = (1..=20).chain(once(25)).collect();
    let double: HashSet<_> = single.iter().map(|n| n * 2).collect();
    let triple: HashSet<_> = single.iter().filter(|&&n| n != 25).map(|n| n * 3).collect();

    let count_table: HashMap<_, _> = (1..=170)
        .map(|n| {
            (
                n,
                [
                    single.contains(&n),
                    double.contains(&n),
                    triple.contains(&n),
                ]
                .iter()
                .filter(|c| **c)
                .count(),
            )
        })
        .chain(once((0, 1)))
        .collect();

    let count = (2..100)
        .map(|n| {
            let v: HashSet<_> = double.iter().map(|m| n - m).filter(|&x| x >= 0).collect();

            v.iter()
                .map(|x| {
                    (0..=x / 2)
                        .map(|y| {
                            let a = *count_table.get(&y).unwrap();
                            let b = *count_table.get(&(x - y)).unwrap();

                            if y != x - y {
                                a * b
                            } else {
                                a * (b + 1) / 2
                            }
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{}", count);
}
