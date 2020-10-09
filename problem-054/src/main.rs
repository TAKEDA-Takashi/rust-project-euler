//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2054
//!
//! カードゲームのポーカーでは, 手札は5枚のカードからなりランク付けされている. 役を低い方から高い方へ順に並べると以下である.
//!
//! 役無し(ハイカード): 一番値が大きいカード
//! ワン・ペア: 同じ値のカードが2枚
//! ツー・ペア: 2つの異なる値のペア
//! スリーカード: 同じ値のカードが3枚
//! ストレート: 5枚の連続する値のカード
//! フラッシュ: 全てのカードが同じスート (注: スートとはダイヤ・ハート・クラブ/スペードというカードの絵柄のこと)
//! フルハウス: スリーカードとペア
//! フォーカード: 同じ値のカードが4枚
//! ストレートフラッシュ: ストレートかつフラッシュ
//! ロイヤルフラッシュ: 同じスートの10, J, Q, K, A
//! ここでカードの値は小さい方から2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K, Aである. (訳注：データ中で10は'T'と表される)
//!
//! もし2人のプレイヤーが同じ役の場合には, 役を構成する中で値が最も大きいカードによってランクが決まる: 例えば, 8のペアは5のペアより強い (下の例1を見よ). それでも同じランクの場合には (例えば, 両者ともQのペアの場合), 一番値が大きいカードによってランクが決まる (下の例4を見よ). 一番値が大きいカードが同じ場合には, 次に値が大きいカードが比べれられ, 以下同様にランクを決定する.
//!
//! 例:
//!
//! 試合	プレイヤー1	プレイヤー2	勝者
//! 1	5H 5C 6S 7S KD
//! 5のペア	2C 3S 8S 8D TD
//! 8のペア	プレイヤー2
//! 2	5D 8C 9S JS AC
//! 役無し, A	2C 5C 7D 8S QH
//! 役無し, Q	プレイヤー1
//! 3	2D 9C AS AH AC
//! Aのスリーカード	3D 6D 7D TD QD
//! ダイヤのフラッシュ	プレイヤー2
//! 4	4D 6S 9H QH QC
//! Qのペア, 9	3D 6D 7H QD QS
//! Qのペア, 7	プレイヤー1
//! 5	2H 2D 4C 4D 4S
//! 4-2のフルハウス	3C 3D 3S 9S 9D
//! 3-9のフルハウス	プレイヤー1
//! poker.txtには1000個のランダムな手札の組が含まれている. 各行は10枚のカードからなる (スペースで区切られている): 最初の5枚がプレイヤー1の手札であり, 残りの5枚がプレイヤー2の手札である. 以下のことを仮定してよい
//!
//! 全ての手札は正しい (使われない文字が出現しない. 同じカードは繰り返されない)
//! 各プレイヤーの手札は特に決まった順に並んでいるわけではない
//! 各勝負で勝敗は必ず決まる
//! 1000回中プレイヤー1が勝つのは何回か? (訳注 : この問題に置いてA 2 3 4 5というストレートは考えなくてもよい)

// ロイヤルフラッシュ同士では勝負が付かない。ストレートフラッシュ同士で勝負が付かない場合がある

use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!(
        "{}",
        BufReader::new(File::open("p054_poker.txt").unwrap())
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
