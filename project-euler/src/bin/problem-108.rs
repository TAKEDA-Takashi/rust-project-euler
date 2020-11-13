//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20108
//!
//! 次の等式で x, y, n は正の整数である.
//!
//! 1/x + 1/y = 1/n
//!
//! n = 4 では 3 つの異なる解がある.
//!
//! 1/5 + 1/20 = 1/4
//!
//! 1/6 + 1/12 = 1/4
//!
//! 1/8 + 1/8 = 1/4
//!
//! 解の数が 1000 を超える最小の n を求めよ.
//!
//! 注: この問題は Problem 110 の易しいケースである. こちらを先に解く事を強く勧める.

// (x - n)(y - n) = n^2 と式変形し、
// n^2の約数の個数/2 が1000を超える境界を探索する。
// 約数の個数は素因数の数で決まるので先に大雑把に計算しておく。

use euler_lib::{divisor_count, Prime};

fn main() {
    let prime = Prime::<u64>::new();
    let n = prime.iter().take(6).product::<u64>();

    for m in 2..=17 {
        let n = n * m;
        let solve = divisor_count(&prime.factorization(&(n * n))) / 2;

        if solve >= 1000 {
            println!("{}: {}", n, solve);
        }
    }
}
