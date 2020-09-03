// http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%202

// フィボナッチ数列の項は前の2つの項の和である. 最初の2項を 1, 2 とすれば, 最初の10項は以下の通りである.
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// 数列の項の値が400万以下の, 偶数値の項の総和を求めよ.

#[derive(Debug)]
struct Fibo {
    current: usize,
    prev: usize,
}

impl Fibo {
    fn new() -> Fibo {
        Fibo {
            current: 1,
            prev: 0,
        }
    }
}

impl Iterator for Fibo {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current + self.prev;
        self.prev = self.current;
        self.current = next;

        Some(next)
    }
}

fn is_even(n: &usize) -> bool {
    n % 2 == 0
}

fn main() {
    println!(
        "{:?}",
        Fibo::new()
            .take_while(|n| { *n <= 4_000_000 })
            .filter(is_even)
            .sum::<usize>()
    );
}
