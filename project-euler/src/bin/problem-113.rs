//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20113
//!
//! ある数の桁を左から右へと順に見たとき, 任意の桁の数が自身の左にある桁の数以上であるとき, その数を増加数 (increasing number) と呼ぶ; 例えば134468は増加数である.
//!
//! 同様に, 任意の桁の数が自身の右にある桁の数以上であるとき, その数を減少数 (decreasing number) と呼ぶ; 例えば66420がそうである.
//!
//! 増加数でも減少数でもない正の整数を "はずみ"数 ("bouncy" number) と呼ぶ; 155349がそうである.
//!
//! nが大きくなるにつれ, n以下のはずみ数の割合は大きくなる. 例えば, 100万未満では, はずみ数でない数は12951個しかない. 同様に, 1010未満では277032個しかない.
//!
//! googol数 (10100) 未満ではずみ数でないものの数を答えよ.

fn main() {
    let pow = 100;

    let mut non_bouncy_count = 99; // 1桁と2桁はすべてnonはずみ数

    let mut increasing_head = vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut increasing_tail = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut decreasing_head = vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut decreasing_tail = vec![9, 9, 8, 7, 6, 5, 4, 3, 2, 1];

    for _ in 0..pow - 2 {
        for n in (2..=9).rev() {
            increasing_head[n - 1] += increasing_head[n];
        }

        for n in 2..=9 {
            increasing_tail[n] += increasing_tail[n - 1];
        }

        decreasing_head[0] = 1;
        for n in (1..=9).rev() {
            let s = decreasing_head[0..=n].iter().sum::<usize>();
            decreasing_head[n] = s;
        }
        decreasing_head[0] = 0;

        for n in (0..=8).rev() {
            decreasing_tail[n] += decreasing_tail[n + 1];
        }

        let all = vec![
            &increasing_head,
            &increasing_tail,
            &decreasing_head,
            &decreasing_tail,
        ];

        non_bouncy_count += all.iter().map(|v| v.iter().sum::<usize>()).sum::<usize>() / 2 - 9;
    }

    println!("{}", non_bouncy_count);
}
