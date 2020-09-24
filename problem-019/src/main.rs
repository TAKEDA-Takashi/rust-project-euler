//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2019
//!
//! 次の情報が与えられている.
//!
//! 1900年1月1日は月曜日である.
//! 9月, 4月, 6月, 11月は30日まであり, 2月を除く他の月は31日まである.
//! 2月は28日まであるが, うるう年のときは29日である.
//! うるう年は西暦が4で割り切れる年に起こる. しかし, 西暦が400で割り切れず100で割り切れる年はうるう年でない.
//! 20世紀（1901年1月1日から2000年12月31日）中に月の初めが日曜日になるのは何回あるか?

use std::iter::repeat;

fn main() {
    println!(
        "{}",
        (1901..=2000)
            .flat_map(|y| repeat(y).zip(1..=12))
            .map(|(y, m)| zeller(y, m, 1))
            .filter(|&h| h == 1) // 日曜日
            .count()
    );
}

/// 土曜日が0。
fn zeller(year: usize, month: usize, day: usize) -> usize {
    assert!(year >= 1582);

    if month == 1 || month == 2 {
        return zeller(year - 1, month + 12, day);
    }

    let h = year / 100;
    let y = year % 100;

    (day + (26 * (month + 1) / 10) + y + (y / 4) + 5 * h + (h / 4)) % 7
}
