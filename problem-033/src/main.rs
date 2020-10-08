//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2033
//!
//! 49/98は面白い分数である.「分子と分母からそれぞれ9を取り除くと, 49/98 = 4/8 となり, 簡単な形にすることができる」と経験の浅い数学者が誤って思い込んでしまうかもしれないからである. (方法は正しくないが，49/98の場合にはたまたま正しい約分になってしまう．)
//!
//! 我々は 30/50 = 3/5 のようなタイプは自明な例だとする.
//!
//! このような分数のうち, 1より小さく分子・分母がともに2桁の数になるような自明でないものは, 4個ある.
//!
//! その4個の分数の積が約分された形で与えられたとき, 分母の値を答えよ.

use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    println!(
        "{}",
        (11..=99)
            .tuple_combinations()
            .filter(|(m, d)| m % 10 != 0 && d % 10 != 0) // trivial and either ten multiple
            .filter_map(|(m, d)| {
                let g = gcd(m, d);
                if g == 1 {
                    return None;
                }

                let digit_canceling = |t| {
                    let (m2, d2) = trip_number(m, d, t);

                    if m * d2 == d * m2 {
                        return Some((m, d));
                    } else {
                        None
                    }
                };

                let mn = mutual_number(m, d);

                if mn.len() == 1 {
                    digit_canceling(mn[0])
                } else if mn.len() == 2 {
                    mn.iter().flat_map(|&t| digit_canceling(t)).next()
                } else {
                    None
                }
            })
            .fold((1, 1), |acc, f| {
                let m = acc.0 * f.0;
                let d = acc.1 * f.1;
                (m / gcd(m, d), d / gcd(m, d))
            })
            .1 // 分母
    );
}

fn trip_number(a: usize, b: usize, t: usize) -> (usize, usize) {
    let t = t.to_string();
    (
        a.to_string().replacen(&t, "", 1).parse().unwrap(),
        b.to_string().replacen(&t, "", 1).parse().unwrap(),
    )
}

fn mutual_number(a: usize, b: usize) -> Vec<usize> {
    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();

    s1.insert(a % 10);
    s1.insert(a / 10);

    s2.insert(b % 10);
    s2.insert(b / 10);

    s1.intersection(&s2).copied().collect()
}

// Same problem-005
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
