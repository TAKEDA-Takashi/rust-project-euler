// // Same problem-041

// use rand::random;

// const MILLER_RABIN_ROUND: usize = 20;

// pub fn is_prime(n: usize) -> bool {
//     if n == 2 {
//         true
//     } else if n < 2 || n % 2 == 0 {
//         false
//     } else {
//         miller_rabin_test(n)
//     }
// }

// fn miller_rabin_test(n: usize) -> bool {
//     let (d, s) = find_odd(n - 1);

//     (0..MILLER_RABIN_ROUND).all(|_| {
//         let a = (random::<f64>() * ((n - 3) as f64)) as usize + 2;
//         let y = modpow(a, d, n);
//         if y == 1 {
//             true
//         } else {
//             (0..s).any(|r| modpow(y, 2_usize.pow(r as u32), n) == n - 1)
//         }
//     })
// }

// fn find_odd(q: usize) -> (usize, usize) {
//     fn find_odd0(q: usize, s: usize) -> (usize, usize) {
//         if q & 1 == 1 {
//             (q, s)
//         } else {
//             find_odd0(q >> 1, s + 1)
//         }
//     }

//     find_odd0(q, 0)
// }

// fn modpow(b: usize, e: usize, m: usize) -> usize {
//     fn modpow0(b: usize, e: usize, m: usize, r: usize) -> usize {
//         if e == 0 {
//             r
//         } else {
//             modpow0(b * b % m, e >> 1, m, if e & 1 == 1 { r * b % m } else { r })
//         }
//     }

//     modpow0(b, e, m, 1)
// }

// // Same problem-007

// #[derive(Debug)]
// pub struct Prime {
//     ps: Vec<usize>,
// }

// impl Prime {
//     /// 素数列を生成するイテレータを生成する。
//     ///
//     /// # Example
//     ///
//     /// ```
//     /// use problem_060::Prime;
//     ///
//     /// let mut prime = Prime::new();
//     /// assert_eq!(prime.next(), Some(2));
//     /// assert_eq!(prime.next(), Some(3));
//     /// assert_eq!(prime.next(), Some(5));
//     /// assert_eq!(prime.next(), Some(7));
//     /// ```
//     pub fn new() -> Prime {
//         Prime { ps: vec![] }
//     }
// }

// impl Iterator for Prime {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.ps.last() {
//             None => {
//                 self.ps.push(2);
//                 Some(2)
//             }
//             Some(2) => {
//                 self.ps.push(3);
//                 Some(3)
//             }
//             Some(last_prime) => {
//                 let next_prime = ((last_prime + 2)..).step_by(2).find(|&n| {
//                     self.ps
//                         .iter()
//                         .take_while(|&&p| p * p <= n)
//                         .all(|&p| n % p != 0)
//                 })?;

//                 self.ps.push(next_prime);
//                 Some(next_prime)
//             }
//         }
//     }
// }
