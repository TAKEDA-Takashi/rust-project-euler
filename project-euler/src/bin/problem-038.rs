//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2038
//!
//! 192 に 1, 2, 3 を掛けてみよう.
//!
//! 192 × 1 = 192
//! 192 × 2 = 384
//! 192 × 3 = 576
//!
//! 積を連結することで1から9の パンデジタル数 192384576 が得られる. 192384576 を 192 と (1,2,3) の連結積と呼ぶ.
//!
//! 同じようにして, 9 を 1,2,3,4,5 と掛け連結することでパンデジタル数 918273645 が得られる. これは 9 と (1,2,3,4,5) との連結積である.
//!
//! 整数と (1,2,...,n) (n > 1) との連結積として得られる9桁のパンデジタル数の中で最大のものはいくつか?

use euler_lib::is_pandigital;

fn main() {
    println!(
        "{}",
        (1..10000) // 5桁以上はあり得ない
            .filter_map(|n| concatenated_product_pandigital(n))
            .max()
            .unwrap()
    );
}

fn concatenated_product_pandigital(n: usize) -> Option<usize> {
    concatenated_product(n)
        .filter(|s| is_pandigital(s))
        .and_then(|s| s.parse().ok())
}

fn concatenated_product(n: usize) -> Option<String> {
    concatenated_product0(n, 1, String::from(""))
}

fn concatenated_product0(n: usize, p: usize, d: String) -> Option<String> {
    match d.len() {
        0..=8 => concatenated_product0(n, p + 1, d + &(n * p).to_string()),
        9 => Some(d),
        _ => None,
    }
}
