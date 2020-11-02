//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20101
//! Optimum polynomial

// 多項式OP(k)の生成方法
// k=1; y = 1
// k=2; y = a(x - 1) + 1, x=2,y=8 => a=7; y = 7(x - 1) + 1 => y = 7x - 6
// k=3; y = a(x - 1)(x - 2) + 7x - 6, x=3,y=27 => a=6; y = 6(x - 1)(x - 2) + 7x - 6 => y = 6x^2 - 11x + 6
// 以下繰り返し

use euler_lib::factorial;
use std::rc::Rc;

fn main() {
    println!(
        "{}",
        Op::new(|n: isize| (0..=10).map(|i| (-n).pow(i)).sum::<isize>())
            .take(10)
            .enumerate()
            .map(|(i, f)| f(i as isize + 2))
            .sum::<isize>()
    );
}

struct Op {
    generating_function: Box<dyn Fn(isize) -> isize>,
    op: Option<Rc<dyn Fn(isize) -> isize>>,
    k: isize,
}

impl Op {
    fn new(gf: impl Fn(isize) -> isize + 'static) -> Self {
        Op {
            generating_function: Box::new(gf),
            op: None,
            k: 0,
        }
    }
}

impl Iterator for Op {
    type Item = Rc<dyn Fn(isize) -> isize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.k += 1;

        if let Some(f) = self.op.take() {
            let k = self.k;
            let a = (self.generating_function.as_ref()(k) - f(k)) / factorial(&(k - 1));
            let f = move |n| a * (1..k).map(|m| n - m).product::<isize>() + f(n);

            self.op = Some(Rc::new(f));
        } else {
            let r = self.generating_function.as_ref()(self.k);
            let f = move |_| r;

            self.op = Some(Rc::new(f));
        }

        Some(Rc::clone(self.op.as_ref()?))
    }
}

// #[test]
// fn test_op() {
//     let actual = vec![
//         0_isize, //dummy
//         1,
//         683,
//         44287,
//         838861,
//         8138021,
//         51828151,
//         247165843,
//         954437177,
//         3138105961,
//         9090909091,
//         23775972551,
//     ];

//     let op = op(&|n: isize| (0..=10).map(|i| (-n).pow(i)).sum::<isize>(), 10);
//     for n in 1..11 {
//         assert_eq!(actual[n], op(n as isize));
//     }

//     assert_ne!(actual[11], op(11));
// }

// fn op(gf: &impl Fn(isize) -> isize, k: isize) -> Box<dyn Fn(isize) -> isize> {
//     if k == 1 {
//         let a = gf(1);
//         Box::new(move |_| a)
//     } else {
//         let f = op(gf, k - 1);
//         let a = (gf(k) - f(k)) / factorial(&(k - 1));
//         Box::new(move |n| a * (1..k).map(|m| n - m).product::<isize>() + f(n))
//     }
// }
