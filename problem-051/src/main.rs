//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2051
//!
//! *3の第1桁を置き換えることで, 13, 23, 43, 53, 73, 83という6つの素数が得られる.
//!
//! 56**3の第3桁と第4桁を同じ数で置き換えることを考えよう. この5桁の数は7つの素数をもつ最初の例である: 56003, 56113, 56333, 56443, 56663, 56773, 56993. よって, この族の最初の数である56003は, このような性質を持つ最小の素数である.
//!
//! 桁を同じ数で置き換えることで8つの素数が得られる最小の素数を求めよ. (注:連続した桁でなくても良い)

use euler_lib::Prime;
use itertools::Itertools;
use std::cell::RefCell;

fn main() {
    let prime = RefCell::new(Prime::new());

    let (prime, _) = (11..)
        .step_by(2)
        .filter(|&n| prime.borrow_mut().is_prime(&n))
        .map(|p| (p, (p as f64).log10() as usize + 1))
        .flat_map(|(p, digit)| {
            (1..digit)
                .flat_map(|c| (0..digit - 1).combinations(c))
                .map(|rep| (p / 10_usize.pow((digit - rep[0] - 1) as u32) % 10, rep))
                .filter(|(base_number, rep)| {
                    rep.iter()
                        .all(|j| p / 10_usize.pow((digit - j - 1) as u32) % 10 == *base_number)
                })
                .map(|(base_number, rep)| {
                    (
                        p,
                        (0..=9 - base_number)
                            .filter(|a| {
                                prime.borrow_mut().is_prime(&rep.iter().fold(p, |acc, j| {
                                    acc + a * 10_usize.pow((digit - j - 1) as u32)
                                }))
                            })
                            .count(),
                    )
                })
                .collect_vec()
        })
        .find(|&(_, len)| len == 8)
        .unwrap();

    println!("{}", prime);
}
