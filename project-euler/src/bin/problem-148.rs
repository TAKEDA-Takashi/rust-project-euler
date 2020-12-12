//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20148
//!
//! パスカルの三角形の最初の7列には7で割り切れる要素は一つもないことが簡単に分かる.
//!
//! 1
//! 1 1
//! 1 2 1
//! 1 3 3 1
//! 1 4 6 4 1
//! 1 5 10 10 5 1
//! 1 6 15 20 15 6 1
//! しかし最初の100列を調べると, 5050個の要素の内, 7で割り切れないものは2361個しかない.
//!
//! パスカルの三角形の最初の10億列 (109列) の要素で7で割り切れないものの数を答えよ.

fn main() {
    let mut target = 10_usize.pow(9);
    let mut pow = (target as f64).log(7.) as u32;

    let mut mag = 1;
    let mut count = 0;

    loop {
        let base = 7_usize.pow(pow as u32);
        let base_mul = target / base;

        count += mag * 28_usize.pow(pow) * base_mul * (base_mul + 1) / 2;

        if pow == 0 {
            break;
        }

        target -= base * base_mul;
        pow -= 1;
        mag *= base_mul + 1;
    }

    println!("{}", count);
}
