//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2050
//!
//! 素数41は6つの連続する素数の和として表せる:
//!
//! 41 = 2 + 3 + 5 + 7 + 11 + 13.
//! 100未満の素数を連続する素数の和で表したときにこれが最長になる.
//!
//! 同様に, 連続する素数の和で1000未満の素数を表したときに最長になるのは953で21項を持つ.
//!
//! 100万未満の素数を連続する素数の和で表したときに最長になるのはどの素数か?

use euler_lib::Prime;
use std::collections::HashSet;

fn main() {
    let ubound = 1_000_000;
    let prime_list: Vec<_> = Prime::new().iter().take_while(|&p| p < ubound).collect();
    let prime_set: HashSet<_> = prime_list.iter().collect();

    let (.., max_prime, _max_len) = (0..prime_list.len())
        .map(|i| {
            prime_list[i..]
                .iter()
                .try_fold(
                    /* sum, len, prime, max_len */ (0, 0, 0, 0),
                    |acc, &n| {
                        if acc.0 >= ubound {
                            Err(acc)
                        } else {
                            let sum = acc.0 + n;
                            let len = acc.1 + 1;

                            if prime_set.contains(&sum) {
                                Ok((sum, len, sum, len))
                            } else {
                                Ok((sum, len, acc.2, acc.3))
                            }
                        }
                    },
                )
                .unwrap_or_else(|n| n)
        })
        .max_by_key(|(.., len)| *len)
        .unwrap();

    println!("{}", max_prime);
}
