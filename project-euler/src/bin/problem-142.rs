//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20142
//!
//! x + y, x - y, x + z, x - z, y + z, y - zが全て平方数となる整数の組 x > y > z > 0で, 最小の x + y + z を求めよ.

use itertools::Itertools;
use num::integer::Roots;

fn main() {
    let int_tuple = (2..1000)
        .tuple_combinations()
        .filter_map(|(b, a)| {
            if (a + b) % 2 != 0 {
                return None;
            }

            let a = a * a;
            let b = b * b;
            Some(((a - b) / 2, (a + b) / 2))
        })
        .filter_map(|(y, x)| {
            (1..)
                .map(|n| n * n)
                .take_while(|&f| f < y)
                .find(|&f| {
                    let z = y - f;
                    is_squre(x + z) && is_squre(x - z) && is_squre(y + z)
                })
                .map(|f| (y - f, y, x))
        })
        .next()
        .unwrap();

    println!("{:?}", int_tuple.0 + int_tuple.1 + int_tuple.2);
}

fn is_squre(n: usize) -> bool {
    let rt = n.sqrt();
    rt * rt == n
}
