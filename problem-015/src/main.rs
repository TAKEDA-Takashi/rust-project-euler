//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2015
//!
//! 2×2 のマス目の左上からスタートした場合, 引き返しなしで右下にいくルートは 6 つある.
//!
//! では, 20×20 のマス目ではいくつのルートがあるか.

fn combination(n: usize, r: usize) -> usize {
    if r == 0 {
        return 1;
    }

    let m = r - 1;
    combination(n, m) * (n - r + 1) / r
}

fn main() {
    println!("{}", combination(40, 20));
}
