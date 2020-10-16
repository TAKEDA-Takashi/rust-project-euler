//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2074
//!
//! 145は各桁の階乗の和が145と自分自身に一致することで有名である.
//!
//! 1! + 4! + 5! = 1 + 24 + 120 = 145
//!
//! 169の性質はあまり知られていない. これは169に戻る数の中で最長の列を成す. このように他の数を経て自分自身に戻るループは3つしか存在しない.
//!
//! 169 → 363601 → 1454 → 169
//! 871 → 45361 → 871
//! 872 → 45362 → 872
//!
//! どのような数からスタートしてもループに入ることが示せる.
//!
//! 例を見てみよう.
//!
//! 69 → 363600 → 1454 → 169 → 363601 (→ 1454)
//! 78 → 45360 → 871 → 45361 (→ 871)
//! 540 → 145 (→ 145)
//!
//! 69から始めた場合, 列は5つの循環しない項を持つ. また100万未満の数から始めた場合最長の循環しない項は60個であることが知られている.
//!
//! 100万未満の数から開始する列の中で, 60個の循環しない項を持つものはいくつあるか?

use std::collections::HashMap;

fn main() {
    let fs = get_factorials(9);

    let mut fsum_map: HashMap<usize, usize> = HashMap::new();
    fsum_map.insert(169, 3);
    fsum_map.insert(363601, 3);
    fsum_map.insert(1454, 3);
    fsum_map.insert(871, 2);
    fsum_map.insert(45361, 2);
    fsum_map.insert(872, 2);
    fsum_map.insert(45362, 2);

    println!(
        "{:?}",
        (2..1_000_000)
            .map(|n| digit_fac_sum(&n, &fs, &mut fsum_map))
            .filter(|&cnt| cnt == 60)
            .count()
    )
}

fn digit_fac_sum(n: &usize, fs: &Vec<usize>, map: &mut HashMap<usize, usize>) -> usize {
    fn digit_fac_sum0(
        n: &usize,
        mut v: Vec<usize>,
        fs: &Vec<usize>,
        map: &mut HashMap<usize, usize>,
    ) -> usize {
        let mut sum = 0;
        let mut m = *n;
        while m != 0 {
            sum += fs[m % 10];
            m /= 10;
        }

        if map.contains_key(&sum) || *n == sum {
            let mut len = *map.get(&sum).unwrap_or(&0);
            for k in v.into_iter().rev() {
                map.insert(k, len + 1);
                len += 1
            }
            len
        } else {
            v.push(sum);

            digit_fac_sum0(&sum, v, fs, map)
        }
    }

    if map.contains_key(n) {
        *map.get(n).unwrap()
    } else {
        digit_fac_sum0(n, vec![*n], fs, map)
    }
}

fn get_factorials(u: usize) -> Vec<usize> {
    let mut v = vec![1];
    for n in 1..=u {
        v.push(n * v[n - 1]);
    }
    v
}
