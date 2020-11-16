//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2089
//! Roman numerals

use once_cell::sync::Lazy;
use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

static ROMAN: Lazy<HashMap<&'static str, usize>> = Lazy::new(|| {
    vec![
        "I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M",
    ]
    .into_iter()
    .zip(vec![1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000].into_iter())
    .collect()
});

static ARABIC: Lazy<BTreeMap<Reverse<usize>, &'static str>> =
    Lazy::new(|| (*ROMAN).iter().map(|(k, v)| (Reverse(*v), *k)).collect());

fn main() {
    let roman_numerals: Vec<String> = load_roman_numerals();
    let arabic_numerals: Vec<usize> = roman_numerals.iter().map(|r| roman_to_arabic(r)).collect();
    let renew_roman_numerals: Vec<String> = arabic_numerals
        .iter()
        .map(|&a| arabic_to_roman(a))
        .collect();

    println!(
        "{}",
        roman_numerals.iter().map(|r| r.len()).sum::<usize>()
            - renew_roman_numerals.iter().map(|r| r.len()).sum::<usize>()
    );
}

fn roman_to_arabic(roman: &str) -> usize {
    let mut i = 0;
    let mut arabic = 0;
    let cs: Vec<char> = roman.chars().collect();

    while i + 1 < cs.len() {
        if let Some(a) = (*ROMAN).get::<str>(&cs[i..=i + 1].iter().collect::<String>()) {
            arabic += a;
            i += 2;
        } else if let Some(a) = (*ROMAN).get::<str>(&cs[i].to_string()) {
            arabic += a;
            i += 1;
        } else {
            panic!("error: {}", roman);
        }
    }

    if i < cs.len() {
        if let Some(a) = (*ROMAN).get::<str>(&cs[i].to_string()) {
            arabic += a;
        }
    }

    arabic
}

fn arabic_to_roman(mut arabic: usize) -> String {
    (*ARABIC).iter().fold(String::new(), |mut roman, (rn, r)| {
        let n = rn.0;
        let c = arabic / n;

        if c != 0 {
            roman.push_str(&r.repeat(c));
            arabic %= n;
        }

        roman
    })
}

fn load_roman_numerals() -> Vec<String> {
    BufReader::new(File::open("files/p089_roman.txt").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .collect()
}
