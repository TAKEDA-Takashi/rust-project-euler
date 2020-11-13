//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2068
//!
//! 下に示す図のようなものを"magic" 3-gon ringという. これは1～6の数字を当てはめて, 各列の数字の和が9となっている. これを例として説明する.
//!
//! p_068_1.gif
//! 外側のノードのうち一番小さいものの付いた列(例では4,3,2)から時計回りに回ってそれぞれ列の数字を3つ連ねて説明する. 例えば例のものは4,3,2; 6,2,1; 5,1,3という組で説明することができる.
//!
//! 1～6の数字を当てはめて, 各列の数字の和が等しくなるものは次の8通りある.
//!
//! 合計	組
//! 9	4,2,3; 5,3,1; 6,1,2
//! 9	4,3,2; 6,2,1; 5,1,3
//! 10	2,3,5; 4,5,1; 6,1,3
//! 10	2,5,3; 6,3,1; 4,1,5
//! 11	1,4,6; 3,6,2; 5,2,4
//! 11	1,6,4; 5,4,2; 3,2,6
//! 12	1,5,6; 2,6,4; 3,4,5
//! 12	1,6,5; 3,5,4; 2,4,6
//! この組の各数字を連結して, 9桁の数字で表すことができる. 例えば, 上の図のものは4,3,2; 6,2,1; 5,1,3であるので432621513である.
//!
//! さて, 下の図に1～10の数字を当てはめ, 各列の数字の和が等しくなる"magic" 5-gon ringを作って, それを表す16桁または17桁の数字のうち, 16桁のものの最大の数字を答えよ.
//!
//! (注, 3つの場合の例を見ても分かる通り, 列の始まりの数字を比べた時一番小さい数字で始まる列から時計回りに繋げるという条件のもとで文字列を生成する必要があります. この条件下で最大となる数字を答えてください. )

use itertools::Itertools;
use project_euler::problem_068::{Column, Ring, RingBuilder};
use std::collections::BTreeSet;

fn main() {
    // let (n_gon, length) = (3, None);
    let (n_gon, length) = (5, Some(16));

    let set: BTreeSet<usize> = (1..=2 * n_gon).collect();

    let rings = (1..=2 * n_gon)
        .combinations(n_gon)
        .filter(|combi| combi.iter().sum::<usize>() % n_gon == 0)
        .map(|inner| {
            (
                (inner.iter().sum::<usize>() + n_gon * (1 + 2 * n_gon)) / n_gon,
                set.difference(&inner.iter().copied().collect())
                    .copied()
                    .collect_vec(),
                inner,
            )
        })
        .map(|(sum, outer, inner)| {
            outer
                .into_iter()
                .map(|o| {
                    (
                        o,
                        inner
                            .iter()
                            .copied()
                            .permutations(2)
                            .filter(|perm| perm.iter().sum::<usize>() + o == sum)
                            .collect_vec(),
                    )
                })
                .collect_vec()
        })
        .flat_map(|pool| build_ring(&pool))
        .collect_vec();

    let magic_ngon_ring = rings
        .iter()
        .map(|r| r.to_string())
        .filter(|s| length.map_or(true, |len| s.len() == len))
        .max()
        .unwrap();

    println!("{}", magic_ngon_ring);
}

fn build_ring(pool: &Vec<(usize, Vec<Vec<usize>>)>) -> Vec<Ring> {
    fn build_ring0(
        pool: &Vec<(usize, Vec<Vec<usize>>)>,
        ring_builder: &RingBuilder,
    ) -> Vec<RingBuilder> {
        if pool.len() == 0 {
            return vec![ring_builder.clone()];
        }

        (0..pool.len())
            .map(|i| {
                let mut next_pool = pool.clone();
                (next_pool.remove(i), next_pool)
            })
            .flat_map(|((n, ps), next_pool)| {
                ps.into_iter()
                    .filter(|p| ring_builder.is_accept(p))
                    .map(|p| {
                        let mut rb = ring_builder.clone();
                        rb.add(Column::new(&n, &p));
                        rb
                    })
                    .flat_map(|rb| build_ring0(&next_pool, &rb))
                    .collect_vec()
            })
            .collect_vec()
    }

    let mut next_pool = pool.clone();
    let (n, ps) = next_pool.remove(0);

    ps.into_iter()
        .map(|p| {
            let mut rb = RingBuilder::new(pool.len());
            rb.add(Column::new(&n, &p));
            rb
        })
        .flat_map(|rb| build_ring0(&next_pool, &rb))
        .filter_map(|rb| rb.build())
        .collect_vec()
}
