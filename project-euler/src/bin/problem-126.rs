//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20126
//!
//! Cuboid layers

use itertools::Itertools;
use std::collections::HashMap;

type Cuboid = (usize, usize, usize);

fn main() {
    let ubound = 5000;
    let cube_bound = 20000;
    let mut table: HashMap<usize, usize> = HashMap::new();

    for a in 1..ubound {
        for b in a..ubound {
            for c in b..ubound {
                for n in 1.. {
                    let cuboid = (a, b, c);
                    let add_cube_count = layer(cuboid, n);

                    if add_cube_count > cube_bound {
                        break;
                    }

                    *table.entry(add_cube_count).or_default() += 1;
                }
            }
        }
    }

    println!(
        "{:?}",
        table
            .iter()
            .filter(|(_, v)| **v == 1000)
            .map(|(k, _)| *k)
            .collect_vec()
            .iter()
            .min()
    );
}

fn layer((a, b, c): Cuboid, n: usize) -> usize {
    2 * (a * b + b * c + c * a)
        + 4 * (n - 1) * (a + b + c)
        + if n > 1 { 4 * (n - 1) * (n - 2) } else { 0 }
}
