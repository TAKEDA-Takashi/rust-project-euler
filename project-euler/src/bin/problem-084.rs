//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2084
//! Monopoly odds

// "GO", "A1", "CC1", "A2", "T1", "R1", "B1", "CH1", "B2", "B3",
// "JAIL", "C1", "U1", "C2", "C3", "R2", "D1", "CC2", "D2", "D3",
// "FP", "E1", "CH2", "E2", "E3", "R3", "F1", "F2", "U2", "F3",
// "G2J", "G1", "G2", "CC3", "G3", "R4", "CH3", "H1", "T2", "H2",

use itertools::Itertools;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::iter::{once, repeat};

const BOARD_SISE: usize = 40;
const DICE_SIDE: usize = 4;
const DICE_NUM: usize = 2;

fn main() {
    let dice_table = get_dice_table();

    let mut board_prob: Vec<_> = once(1.0).chain(repeat(0.0)).take(BOARD_SISE).collect();

    for _ in 0..64 {
        let mut bp_updates = vec![];

        for current in 0..BOARD_SISE {
            if board_prob[current] == 0.0 {
                continue;
            }

            let mut bp_update = repeat(0.0).take(BOARD_SISE).collect();
            for dice in &dice_table {
                update_board(current, dice, 0, &mut bp_update);
            }

            bp_updates.push(
                bp_update
                    .iter()
                    .map(|p| p * board_prob[current])
                    .collect_vec(),
            );
        }

        for i in 0..bp_updates.len() {
            for j in 0..bp_updates[i].len() {
                board_prob[j] += bp_updates[i][j];
            }
        }

        board_prob = board_prob.iter().map(|p| p / 2.0).collect();
    }

    // println!("{}", board_prob.iter().sum::<f64>());
    // board_prob.iter().enumerate().for_each(|(n, prob)| {
    //     println!("{}: {:.4}", n, prob * 100.0);
    // });
    board_prob
        .iter()
        .enumerate()
        .sorted_by(|a, b| PartialOrd::partial_cmp(b.1, a.1).unwrap())
        .take(3)
        .for_each(|(n, prob)| {
            println!("{}: {:.4}", n, prob * 100.0);
        });
}

fn get_dice_table() -> HashMap<Vec<Vec<usize>>, f64> {
    (1..=DICE_SIDE)
        .combinations_with_replacement(DICE_NUM)
        .flat_map(|a| {
            if a[0] == a[1] {
                (1..=DICE_SIDE)
                    .combinations_with_replacement(DICE_NUM)
                    .map(|b| vec![a.clone(), b])
                    .collect_vec()
            } else {
                vec![vec![a]]
            }
        })
        .flat_map(|v| {
            if v.len() == 2 && v[1][0] == v[1][1] {
                (1..=DICE_SIDE)
                    .combinations_with_replacement(DICE_NUM)
                    .map(|c| vec![v[0].clone(), v[1].clone(), c])
                    .collect_vec()
            } else {
                vec![v]
            }
        })
        .map(|v| {
            let prob = if v.len() == 1 {
                2.0 / (DICE_SIDE * DICE_SIDE) as f64
            } else if v.len() == 2 {
                2.0 / (DICE_SIDE * DICE_SIDE).pow(2) as f64
            } else if v[2][0] != v[2][1] {
                2.0 / (DICE_SIDE * DICE_SIDE).pow(3) as f64
            } else {
                1.0 / (DICE_SIDE * DICE_SIDE).pow(3) as f64
            };

            (v, prob)
        })
        .collect()
}

fn update_board(
    current: usize,
    (dice, prob): (&Vec<Vec<usize>>, &f64),
    nth_throw: usize,
    board_prob: &mut Vec<f64>,
) {
    if dice.len() <= nth_throw {
        board_prob[current] += prob;
        return;
    }

    // G2J
    if dice.len() == 3 && dice[2][0] == dice[2][1] {
        board_prob[10] += prob;
        return;
    }

    for (n, p) in solve_move_event((current + dice[nth_throw][0] + dice[nth_throw][1]) % BOARD_SISE)
    {
        update_board(n, (dice, &(prob * p)), nth_throw + 1, board_prob)
    }
}

fn solve_move_event(next_number: usize) -> Vec<(usize, f64)> {
    match next_number {
        // CC1, CC2, CC3
        2 | 17 | 33 => vec![(next_number, 7.0 / 8.0), (0, 1.0 / 16.0), (10, 1.0 / 16.0)],
        // CH1, CH2, CH3
        7 | 22 | 36 => vec![
            (next_number, 3.0 / 8.0),
            (0, 1.0 / 16.0),
            (10, 1.0 / 16.0),
            (11, 1.0 / 16.0),
            (24, 1.0 / 16.0),
            (39, 1.0 / 16.0),
            (5, 1.0 / 16.0),
            (next_r(next_number), 1.0 / 8.0),
            (next_u(next_number), 1.0 / 16.0),
            (next_number - 3, 1.0 / 16.0),
        ],
        // G2J
        30 => vec![(10, 1.0)],
        _ => vec![(next_number, 1.0)],
    }
}

fn next_r(n: usize) -> usize {
    match n {
        7 => 15,
        22 => 25,
        36 => 5,
        _ => panic!("error!: {}", n),
    }
}

fn next_u(n: usize) -> usize {
    match n {
        7 | 36 => 12,
        22 => 28,
        _ => panic!("error!: {}", n),
    }
}
