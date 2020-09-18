//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2014
//!
//! 正の整数に以下の式で繰り返し生成する数列を定義する.
//!
//! n → n/2 (n が偶数)
//!
//! n → 3n + 1 (n が奇数)
//!
//! 13からはじめるとこの数列は以下のようになる.
//!
//! 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
//! 13から1まで10個の項になる. この数列はどのような数字からはじめても最終的には 1 になると考えられているが, まだそのことは証明されていない(コラッツ問題)
//!
//! さて, 100万未満の数字の中でどの数字からはじめれば最長の数列を生成するか.
//!
//! 注意: 数列の途中で100万以上になってもよい

struct Collatz {
    num: Option<usize>,
}

impl Collatz {
    fn new(seed: usize) -> Collatz {
        Collatz { num: Some(seed) }
    }
}

impl Iterator for Collatz {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(num) = self.num {
            if num % 2 == 0 {
                self.num = Some(num / 2)
            } else if num == 1 {
                self.num = None
            } else {
                self.num = Some(3 * num + 1)
            }
            Some(num)
        } else {
            None
        }
    }
}

fn main() {
    // for n in Collatz::new(13) {
    //     print! {"{} ", n};
    // }

    // println!("{}", Collatz::new(13).count());

    println!(
        "{:?}",
        (2..1_000_000)
            .max_by_key(|&n| Collatz::new(n).count())
            .unwrap()
    );
}
