//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2031
//!
//! イギリスでは硬貨はポンド£とペンスpがあり，一般的に流通している硬貨は以下の8種類である.
//!
//! 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
//!
//! 以下の方法で£2を作ることが可能である．
//!
//! 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
//!
//! これらの硬貨を使って£2を作る方法は何通りあるか?

enum Coin {
    Pense(usize),
    Pound(usize),
}

impl Coin {
    fn get_pense(&self) -> usize {
        match self {
            Coin::Pense(p) => *p,
            Coin::Pound(p) => p * 100,
        }
    }

    fn get_under(&self) -> Option<Coin> {
        match self {
            Coin::Pound(2) => Some(Coin::Pound(1)),
            Coin::Pound(1) => Some(Coin::Pense(50)),
            Coin::Pense(50) => Some(Coin::Pense(20)),
            Coin::Pense(20) => Some(Coin::Pense(10)),
            Coin::Pense(10) => Some(Coin::Pense(5)),
            Coin::Pense(5) => Some(Coin::Pense(2)),
            Coin::Pense(2) => Some(Coin::Pense(1)),
            _ => None,
        }
    }
}

fn main() {
    println!("{}", find_pattern(200));
}

fn find_pattern(target: usize) -> usize {
    find_pattern0(&Coin::Pound(2), 0, target)
}

fn find_pattern0(coin: &Coin, total: usize, target: usize) -> usize {
    match coin.get_under() {
        Some(ref under_coin) => (0..=(target - total))
            .step_by(coin.get_pense())
            .map(|n| find_pattern0(under_coin, total + n, target))
            .sum(),
        None => 1,
    }
}
