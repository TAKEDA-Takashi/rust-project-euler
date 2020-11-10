//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20120
//!
//! (a-1)n+(a+1)n を a2 で割った余りを r と定義する.
//!
//! 例えば, a=7, n=3 の時, r=42 である: 63 + 83 = 728 ≡ 42 mod 49. n が変われば r も変わるが, a=7 の時 r の最大値 rmax は 42 であることがわかる.
//!
//! 3 ≤ a ≤ 1000 において, Σ rmax を求めよ.

fn main() {
    let sum = (3..=1000_u64)
        .map(|a| if a % 2 == 0 { (a - 2) * a } else { (a - 1) * a })
        .sum::<u64>();

    println!("{}", sum);
}
