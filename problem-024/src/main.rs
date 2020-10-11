//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2024
//!
//! 順列とはモノの順番付きの並びのことである. たとえば, 3124は数 1, 2, 3, 4 の一つの順列である. すべての順列を数の大小でまたは辞書式に並べたものを辞書順と呼ぶ. 0と1と2の順列を辞書順に並べると
//!
//! 012 021 102 120 201 210
//! になる.
//!
//! 0,1,2,3,4,5,6,7,8,9からなる順列を辞書式に並べたときの100万番目はいくつか?

fn main() {
    let digit = 10;
    let fs = get_factorials(digit);

    let (ns, _) = (0..digit).fold(
        ((0..digit).collect::<Vec<_>>(), 1_000_000 - 1), // 0-origin
        |(ns, target), i| {
            let j = target / fs[i]; // 小数点以下切り捨て

            (
                (i + 1..=j + i).rev().fold(ns, |mut acc, k| {
                    let tmp = acc[k];
                    acc[k] = acc[k - 1];
                    acc[k - 1] = tmp;
                    acc
                }),
                target - fs[i] * j,
            )
        },
    );

    println!("{}", ns.iter().map(|n| n.to_string()).collect::<String>());
}

fn get_factorials(u: usize) -> Vec<usize> {
    let mut v = vec![1];
    for n in 1..=u {
        v.push(n * v[n - 1]);
    }
    v.into_iter().rev().skip(1).collect()
}
