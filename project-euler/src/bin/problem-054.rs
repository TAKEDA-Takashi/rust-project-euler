//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2054
//! Poker hands

// ロイヤルフラッシュ同士では勝負が付かない。ストレートフラッシュ同士で勝負が付かない場合がある

use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{}",
        BufReader::new(File::open("files/p054_poker.txt").unwrap())
            .lines()
            .filter_map(|result| result.ok())
            .filter(|line| poker_score(&line[0..14]) > poker_score(&line[15..]))
            .count()
    );
}

fn poker_score(hand: &str) -> u64 {
    assert!(hand.len() == 2 * 5 + 4);

    let (ns, ss): (Vec<_>, Vec<_>) = hand
        .split(" ")
        .map(|h| h.chars().collect_tuple().unwrap())
        .unzip();

    let number = ns
        .iter()
        .filter_map(|c| match c {
            '1'..='9' => c.to_digit(10).map(|n| n as u64),
            'T' => Some(10),
            'J' => Some(11),
            'Q' => Some(12),
            'K' => Some(13),
            'A' => Some(14),
            _ => None,
        })
        .sorted()
        .collect_vec();

    let is_flush = ss.iter().all(|&c| c == ss[0]);
    let is_straight = (number[0]..).take(5).collect_vec() == number;

    let mut score = 0;

    if is_straight && is_flush {
        if number[0] == 10 {
            score |= PokerHand::RoyalFlush as u64;
        } else {
            score |= PokerHand::StraightFlush as u64 * number[4];
        }
    }

    if is_flush {
        score |= PokerHand::Flush as u64 * number[4];
    }

    if is_straight {
        score |= PokerHand::Straight as u64 * number[4];
    }

    let pair = number
        .iter()
        .group_by(|&&e| e)
        .into_iter()
        .map(|(key, group)| (group.count(), key))
        .sorted_by(|(c1, _), (c2, _)| c2.cmp(c1))
        .collect_vec();

    if pair[0].0 == 4 {
        score |= PokerHand::FourOfAKind as u64 * pair[0].1;
    }

    if pair[0].0 == 3 {
        if pair[1].0 == 2 {
            score |= PokerHand::FullHouse as u64 * pair[0].1;
        } else {
            score |= PokerHand::ThreeOfAKind as u64 * pair[0].1;
        }
    }

    if pair[0].0 == 2 {
        if pair[1].0 == 2 {
            score |= PokerHand::TwoPair as u64 * pair[0].1;
        } else {
            score |= PokerHand::OnePair as u64 * pair[0].1;
        }
    }

    score |= PokerHand::HighCard as u64 * number[4];

    score
}

enum PokerHand {
    HighCard = 0x0000000001,
    OnePair = 0x0000000010,
    TwoPair = 0x0000000100,
    ThreeOfAKind = 0x0000001000,
    Straight = 0x0000010000,
    Flush = 0x0000100000,
    FullHouse = 0x0001000000,
    FourOfAKind = 0x0010000000,
    StraightFlush = 0x0100000000,
    RoyalFlush = 0x1000000000,
}
