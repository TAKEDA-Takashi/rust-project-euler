//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20139
//!
//! (a, b, c)で各辺の長さが整数の直角三角形の三辺を表す. 一辺の長さがcの正方形中に先ほどの三角形を4つ配置することが可能である.
//!
//! 例えば (3, 4, 5)-三角形は5×5の正方形に4つ配置される. このとき, 中央部に1×1の穴が空いている. また, 5×5の正方形は25個の1×1の正方形で敷き詰めることが出来る.
//!
//! p139.png
//! しかし, (5, 12, 13)-三角形を使った場合は穴のサイズが7×7になり, 7×7の正方形では13×13の正方形を敷き詰めることが出来ない.
//!
//! では, 108未満の周囲長を持つ直角三角形を考え, 上のような敷き詰め方を許す直角三角形の数を答えよ.

// 2m^2 + 2m - 10^8 < 0 がubound

use euler_lib::primitive_pythagorean_triple;
use std::iter::repeat;

fn main() {
    let ubound = 10_u64.pow(8);

    println!(
        "{}",
        (2_u64..7200)
            .flat_map(|m| repeat(m).zip(1..m))
            .filter_map(|(m, n)| primitive_pythagorean_triple(m, n))
            .filter(|(a, b, c)| a + b + c < ubound && c % (b - a) == 0)
            .map(|(a, b, c)| ubound / (a + b + c))
            .sum::<u64>()
    );
}
