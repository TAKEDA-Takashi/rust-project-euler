//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20131
//!
//! いくつかの素数pでは, ある正の整数nが存在して, n3+pn2が立方数になる.
//!
//! 例えば, p = 19のときには, 83+19×82=123である.
//!
//! このような性質を持つ各素数について, nの値は一意に定まる. また, 100未満の素数では4つしかこの性質を満たさない.
//!
//! この性質を持つ100万未満の素数は何個あるだろうか?

use euler_lib::Prime;

fn main() {
    let prime = Prime::<u128>::new();

    println!(
        "{}",
        (1_u128..)
            .map(|i| (i.pow(3), (i + 1).pow(3)))
            .take_while(|(n, m)| m - n < 1_000_000)
            .filter(|(n, m)| prime.is_prime(&(m - n)))
            .count()
    );
}
