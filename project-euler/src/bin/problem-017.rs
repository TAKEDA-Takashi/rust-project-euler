//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2017
//!
//! 1 から 5 までの数字を英単語で書けば one, two, three, four, five であり, 全部で 3 + 3 + 5 + 4 + 4 = 19 の文字が使われている.
//!
//! では 1 から 1000 (one thousand) までの数字をすべて英単語で書けば, 全部で何文字になるか.
//!
//! 注: 空白文字やハイフンを数えないこと. 例えば, 342 (three hundred and forty-two) は 23 文字, 115 (one hundred and fifteen) は20文字と数える. なお, "and" を使用するのは英国の慣習.

use std::collections::HashMap;

fn main() {
    let mut numeral_dict = HashMap::new();
    numeral_dict.insert(1, "one");
    numeral_dict.insert(2, "two");
    numeral_dict.insert(3, "three");
    numeral_dict.insert(4, "four");
    numeral_dict.insert(5, "five");
    numeral_dict.insert(6, "six");
    numeral_dict.insert(7, "seven");
    numeral_dict.insert(8, "eight");
    numeral_dict.insert(9, "nine");
    numeral_dict.insert(10, "ten");
    numeral_dict.insert(11, "eleven");
    numeral_dict.insert(12, "twelve");
    numeral_dict.insert(13, "thirteen");
    numeral_dict.insert(14, "fourteen");
    numeral_dict.insert(15, "fifteen");
    numeral_dict.insert(16, "sixteen");
    numeral_dict.insert(17, "seventeen");
    numeral_dict.insert(18, "eighteen");
    numeral_dict.insert(19, "nineteen");
    numeral_dict.insert(20, "twenty");
    numeral_dict.insert(30, "thirty");
    numeral_dict.insert(40, "forty");
    numeral_dict.insert(50, "fifty");
    numeral_dict.insert(60, "sixty");
    numeral_dict.insert(70, "seventy");
    numeral_dict.insert(80, "eighty");
    numeral_dict.insert(90, "ninety");

    let to_numeral = |n: usize| {
        assert!(0 < n && n <= 1000);

        if n == 1000 {
            return "one thousand".to_string();
        }

        let hundreds_place = n / 100;
        let last_two_digit = n % 100;

        if last_two_digit == 0 {
            return format!("{} hundred", numeral_dict[&hundreds_place]);
        }

        let l2d_numeral;

        if numeral_dict.contains_key(&last_two_digit) {
            l2d_numeral = numeral_dict[&last_two_digit].to_string();
        } else {
            let once_place = last_two_digit % 10;
            let tens_place = last_two_digit - once_place;
            l2d_numeral = format!(
                "{}-{}",
                numeral_dict[&tens_place], numeral_dict[&once_place]
            )
        }

        if hundreds_place == 0 {
            l2d_numeral
        } else {
            format!(
                "{} hundred and {}",
                numeral_dict[&hundreds_place], l2d_numeral
            )
        }
    };

    println!(
        "{}",
        (1..=1000)
            .map(to_numeral)
            .map(|s| s.replace("-", "").replace(" ", "").len())
            .sum::<usize>()
    );
}
