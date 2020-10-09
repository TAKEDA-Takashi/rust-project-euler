//! https://projecteuler.net/problem=57
//!
//! 2の平方根は無限に続く連分数で表すことができる.
//!
//! √ 2 = 1 + 1/(2 + 1/(2 + 1/(2 + ... ))) = 1.414213...
//! 最初の4回の繰り返しを展開すると以下が得られる.
//!
//! 1 + 1/2 = 3/2 = 1.5
//! 1 + 1/(2 + 1/2) = 7/5 = 1.4
//! 1 + 1/(2 + 1/(2 + 1/2)) = 17/12 = 1.41666...
//! 1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29 = 1.41379...
//!
//! 次の3つの項は99/70, 239/169, 577/408である. 第8項は1393/985である. これは分子の桁数が分母の桁数を超える最初の例である.
//!
//! 最初の1000項を考えたとき, 分子の桁数が分母の桁数を超える項はいくつあるか?

use num::traits::{one, zero};
use num::{BigInt, BigRational};

fn main() {
    println!(
        "{}",
        (0..1000)
            .scan(Some(BigRational::new(zero(), one())), |ratio, _| {
                let r = one::<BigRational>()
                    / (ratio.take().unwrap() + BigRational::from_integer(BigInt::from(2)));

                *ratio = Some(r.clone());
                Some(r + one::<BigRational>())
            })
            .filter(|ratio| ratio.numer().to_string().len() > ratio.denom().to_string().len())
            .count()
    );
}
