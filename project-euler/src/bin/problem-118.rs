//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20118
//!
//! 1 から 9 の全ての数字を使い, 自由につなげることで 10 進数の数字を作り, 複数の集合を作ることができる. 集合 {2,5,47,89,631} は面白いことに全ての要素が素数である.
//!
//! 1 から 9 の数字をちょうど 1 個ずつ含み, 素数の要素しか含まない集合はいくつあるか?

use euler_lib::Prime;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let numbers: HashSet<_> = (1..=9).collect();

    let mut prime = Prime::<u32>::new();
    let mut prime_iter = prime.iter().filter(|&p| !p.to_string().contains(&"0"));

    let p1: HashSet<_> = prime_iter.by_ref().take_while(|&p| p < 10).collect();
    let p2: HashSet<_> = prime_iter
        .by_ref()
        .take_while(|&p| p < 10_u32.pow(2))
        .collect();
    let p3: HashSet<_> = prime_iter
        .by_ref()
        .take_while(|&p| p < 10_u32.pow(3))
        .collect();
    let p4: HashSet<_> = prime_iter
        .by_ref()
        .take_while(|&p| p < 10_u32.pow(4))
        .collect();
    let p5: HashSet<_> = prime_iter
        .by_ref()
        .take_while(|&p| p < 10_u32.pow(5))
        .collect();
    let p6: HashSet<_> = prime_iter
        .by_ref()
        .take_while(|&p| p < 10_u32.pow(6))
        .collect();
    let p7: HashSet<_> = prime_iter
        .by_ref()
        .take_while(|&p| p < 10_u32.pow(7))
        .collect();
    // 8桁は素数生成より都度判定の方が計算量が少ない

    let mut subsum = HashMap::new();
    subsum.insert(1, 0);
    subsum.insert(2, 0);
    subsum.insert(4, 0);
    subsum.insert(6, 0);
    subsum.insert(12, 0);
    subsum.insert(24, 0);
    subsum.insert(36, 0);

    for v in numbers.iter().permutations(9) {
        // 1 1 1 1 2 3
        // 1 1 1 1 5
        if p1.contains(v[0]) && p1.contains(v[1]) && p1.contains(v[2]) && p1.contains(v[3]) {
            let x = v[4] * 10 + v[5];
            let y = v[6] * 100 + v[7] * 10 + v[8];
            if p2.contains(&x) && p3.contains(&y) {
                *subsum.get_mut(&24).unwrap() += 1;
            }

            let x = x * 1000 + y;
            if p5.contains(&x) {
                *subsum.get_mut(&24).unwrap() += 1;
            }
        }

        // 1 1 1 2 2 2
        // 1 1 1 2 4
        // 1 1 1 3 3
        // 1 1 1 6
        if p1.contains(v[0]) && p1.contains(v[1]) && p1.contains(v[2]) {
            let x = v[3] * 10 + v[4];
            if p2.contains(&x) {
                let y = v[5] * 10 + v[6];
                let z = v[7] * 10 + v[8];
                if p2.contains(&y) && p2.contains(&z) {
                    *subsum.get_mut(&36).unwrap() += 1;
                }

                let y = y * 100 + z;
                if p4.contains(&y) {
                    *subsum.get_mut(&6).unwrap() += 1;
                }
            }

            let x = x * 10 + v[5];
            let y = v[6] * 100 + v[7] * 10 + v[8];
            if p3.contains(&x) && p3.contains(&y) {
                *subsum.get_mut(&12).unwrap() += 1;
            }

            let x = x * 1000 + y;
            if p6.contains(&x) {
                *subsum.get_mut(&6).unwrap() += 1;
            }
        }

        // 1 1 2 2 3
        // 1 1 2 5
        // 1 1 3 4
        // 1 1 7
        if p1.contains(v[0]) && p1.contains(v[1]) {
            let x = v[2] * 10 + v[3];
            if p2.contains(&x) {
                let y = v[4] * 10 + v[5];
                let z = v[6] * 100 + v[7] * 10 + v[8];
                if p2.contains(&y) && p3.contains(&z) {
                    *subsum.get_mut(&4).unwrap() += 1;
                }

                let y = y * 1000 + z;
                if p5.contains(&y) {
                    *subsum.get_mut(&2).unwrap() += 1;
                }
            }

            let x = x * 10 + v[4];
            let y = v[5] * 1000 + v[6] * 100 + v[7] * 10 + v[8];
            if p3.contains(&x) && p4.contains(&y) {
                *subsum.get_mut(&2).unwrap() += 1;
            }

            let x = x * 10000 + y;
            if p7.contains(&x) {
                *subsum.get_mut(&2).unwrap() += 1;
            }
        }

        // 1 2 2 2 2
        // 1 2 2 4
        // 1 2 3 3
        // 1 2 6
        // 1 3 5
        // 1 4 4
        // 1 8
        if p1.contains(v[0]) {
            let x = v[1] * 10 + v[2];
            if p2.contains(&x) {
                let y = v[3] * 10 + v[4];
                if p2.contains(&y) {
                    let z = v[5] * 10 + v[6];
                    let w = v[7] * 10 + v[8];
                    if p2.contains(&z) && p2.contains(&w) {
                        *subsum.get_mut(&24).unwrap() += 1;
                    }

                    let z = z * 100 + w;
                    if p4.contains(&z) {
                        *subsum.get_mut(&2).unwrap() += 1;
                    }
                }

                let y = y * 10 + v[5];
                let z = v[6] * 100 + v[7] * 10 + v[8];
                if p3.contains(&y) && p3.contains(&z) {
                    *subsum.get_mut(&2).unwrap() += 1;
                }

                let y = y * 1000 + z;
                if p6.contains(&y) {
                    *subsum.get_mut(&1).unwrap() += 1;
                }
            }

            let x = x * 10 + v[3];
            let y = v[4] * 10000 + v[5] * 1000 + v[6] * 100 + v[7] * 10 + v[8];
            if p3.contains(&x) && p5.contains(&y) {
                *subsum.get_mut(&1).unwrap() += 1;
            }

            let x = x * 10 + v[4];
            let y = v[5] * 1000 + v[6] * 100 + v[7] * 10 + v[8];
            if p4.contains(&x) && p4.contains(&y) {
                *subsum.get_mut(&2).unwrap() += 1;
            }

            let x = x * 10000 + y;
            if prime.is_prime(&x) {
                *subsum.get_mut(&1).unwrap() += 1;
            }
        }

        // 2 2 2 3
        // 2 2 5
        // 2 3 4
        // 2 7
        let x = v[0] * 10 + v[1];
        if p2.contains(&x) {
            let y = v[2] * 10 + v[3];
            if p2.contains(&y) {
                let z = v[4] * 10 + v[5];
                let w = v[6] * 100 + v[7] * 10 + v[8];
                if p2.contains(&z) && p3.contains(&w) {
                    *subsum.get_mut(&6).unwrap() += 1;
                }

                let z = z * 1000 + w;
                if p5.contains(&z) {
                    *subsum.get_mut(&2).unwrap() += 1;
                }
            }

            let y = y * 10 + v[4];
            let z = v[5] * 1000 + v[6] * 100 + v[7] * 10 + v[8];
            if p3.contains(&y) && p4.contains(&z) {
                *subsum.get_mut(&1).unwrap() += 1;
            }

            let y = y * 10000 + z;
            if p7.contains(&y) {
                *subsum.get_mut(&1).unwrap() += 1;
            }
        }

        // 3 3 3
        // 3 6
        let x = x * 10 + v[2];
        if p3.contains(&x) {
            let y = v[3] * 100 + v[4] * 10 + v[5];
            let z = v[6] * 100 + v[7] * 10 + v[8];
            if p3.contains(&y) && p3.contains(&z) {
                *subsum.get_mut(&6).unwrap() += 1;
            }

            let y = y * 1000 + z;
            if p6.contains(&y) {
                *subsum.get_mut(&1).unwrap() += 1;
            }
        }

        // 4 5
        let x = x * 10 + v[3];
        let y = v[4] * 10000 + v[5] * 1000 + v[6] * 100 + v[7] * 10 + v[8];
        if p4.contains(&x) && p5.contains(&y) {
            *subsum.get_mut(&1).unwrap() += 1;
        }
    }

    // println!("{:?}", subsum);
    println!("{}", subsum.iter().map(|(k, v)| v / k).sum::<u32>());
}
