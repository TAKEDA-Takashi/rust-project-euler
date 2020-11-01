//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20101
//! Optimum polynomial

// 多項式OP(k)の生成方法
// k=1; y = 1
// k=2; y = a(x - 1) + 1, x=2,y=8 => a=7; y = 7(x - 1) + 1 => y = 7x - 6
// k=3; y = a(x - 1)(x - 2) + 7x - 6, x=3,y=27 => a=6; y = 6(x - 1)(x - 2) + 7x - 6 => y = 6x^2 - 11x + 6
// 以下繰り返し

use euler_lib::factorial;

fn main() {
    println!("{}", (1..=10).map(|n| op(n)(n + 1)).sum::<isize>());
}

fn generating_function(n: isize) -> isize {
    // 1 - n + n.pow(2) - n.pow(3) + n.pow(4) - n.pow(5) + n.pow(6) - n.pow(7) + n.pow(8) - n.pow(9)
    //     + n.pow(10)

    (0..=10).map(|i| (-n).pow(i)).sum::<isize>()
}

fn op(k: isize) -> Box<dyn Fn(isize) -> isize> {
    if k == 1 {
        Box::new(|_| 1) // generating_function(1)
    } else {
        let f = op(k - 1);
        let a = (generating_function(k) - f(k)) / factorial(&(k - 1));
        Box::new(move |n| a * (1..k).map(|m| n - m).product::<isize>() + f(n))
    }
}
