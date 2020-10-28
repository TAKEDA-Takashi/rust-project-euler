//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2078
//!
//! n 枚のコインを異なった方法で山に分ける場合の数を p(n) と表わす. 例えば, 5枚のコインを山に分ける異なったやり方は７通りなので p(5)=7 となる.
//!
//! OOOOO
//! OOOO   O
//! OOO   OO
//! OOO   O   O
//! OO   OO   O
//! OO   O   O   O
//! O   O   O   O   O
//!
//! p(n) が100万で割り切れる場合に最小となる n を求めよ.

// https://www.math.nagoya-u.ac.jp/ja/public/agora/download/agora-2006c-note.pdf

const MODULO: i32 = 1_000_000;

fn main() {
    let mut memo: Vec<i32> = vec![1];

    for n in 1.. {
        let a = (1..)
            .take_while(|&k| k * (3 * k - 1) / 2 <= n)
            .map(|k| if (k - 1) % 2 == 0 { 1 } else { -1 } * memo[(n - k * (3 * k - 1) / 2) as usize])
            .sum::<i32>();

        let b = (1..)
            .take_while(|&l| l * (3 * l + 1) / 2 <= n)
            .map(|l| if (l - 1) % 2 == 0 { 1 } else { -1 } * memo[(n - l * (3 * l + 1) / 2) as usize])
            .sum::<i32>();

        let r = (a + b) % MODULO;
        if r == 0 {
            println!("{}", n);
            break;
        }

        memo.push(r);
    }
}
