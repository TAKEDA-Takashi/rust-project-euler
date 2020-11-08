//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20116
//!
//! 5 個の黒い正方形のタイルの列を, 赤(長さ 2), 緑(長さ 3), 青(長さ 4)から選んで, この色のついた長方形のタイルでいくつか置き換える.
//!
//! もし赤のタイルを選んだ場合は, ちょうど 7 通りの方法がある.
//!
//! 116_1.png
//! もし緑のタイルを選んだ場合は, 3 通りである.
//!
//! 116_2.png
//! もし青のタイルを選んだ場合は, 2 通りである.
//!
//! 116_3.png
//! 複数の色を混ぜられない場合は, 5 ユニットの長さの 1 列に並んだ黒いタイルを置き換える方法は 7 + 3 + 2 = 12 通りある.
//!
//! 50 ユニットの長さの 1 列に並んだ黒いタイルを置き換える方法は何通りあるか. ただし複数の色を混ぜることはできず, 少なくとも 1 個は色のついたタイルを使うこと.
//!
//! 注: この問題は Problem 117 に関連する

use cached::proc_macro::cached;

fn main() {
    let n = 50;

    println!(
        "{}",
        block_combinations(2, n) + block_combinations(3, n) + block_combinations(4, n)
    );
}

#[cached]
fn block_combinations(m: usize, n: usize) -> usize {
    if m > n {
        0
    } else if m == n {
        1
    } else {
        block_combinations(m, n - 1) + block_combinations(m, n - m) + 1
    }
}
