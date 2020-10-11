use num::{one, range_step, zero, CheckedAdd, FromPrimitive, Integer};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Prime<T> {
    prime_factor_map: BTreeMap<T, Vec<T>>,
    prime_list: Vec<T>,
}

impl<T> Prime<T>
where
    T: Integer + CheckedAdd + Clone + FromPrimitive,
{
    pub fn new() -> Prime<T> {
        Prime {
            prime_factor_map: BTreeMap::new(),
            prime_list: vec![],
        }
    }

    pub fn factorization(&mut self, n: T) -> &Vec<T> {
        if self.prime_factor_map.contains_key(&n) {
            return &self.prime_factor_map[&n];
        }

        let mut prime_iter = PrimeIterator {
            ps: &mut self.prime_list,
            index: 0,
        };

        self.prime_factor_map.insert(
            n.clone(),
            prime_factorization(&n, &mut prime_iter, &self.prime_factor_map),
        );

        &self.prime_factor_map[&n]
    }

    pub fn iter(&mut self) -> PrimeIterator<T> {
        PrimeIterator {
            ps: &mut self.prime_list,
            index: 0,
        }
    }
}

fn prime_factorization<T>(
    n: &T,
    p_iter: &mut PrimeIterator<T>,
    prime_factor_map: &BTreeMap<T, Vec<T>>,
) -> Vec<T>
where
    T: Integer + CheckedAdd + Clone + FromPrimitive,
{
    if *n == zero() {
        panic!("0 cannot be factored");
    }

    if prime_factor_map.contains_key(&n) {
        return prime_factor_map[&n].clone();
    }

    let p = p_iter.next().unwrap();

    let mut n = n.clone();
    let mut ps = vec![];

    while n.clone() % p.clone() == zero() {
        ps.push(p.clone());
        n = n / p.clone();
    }

    if n == one() {
        // pass
    } else if n < p.clone() * p.clone() {
        ps.push(n);
    } else {
        ps.extend(prime_factorization(&n, p_iter, prime_factor_map))
    }

    ps
}

#[derive(Debug)]
pub struct PrimeIterator<'a, T> {
    ps: &'a mut Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for PrimeIterator<'a, T>
where
    T: Integer + CheckedAdd + Clone + FromPrimitive,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let two = T::from_u32(2)?;
        let three = T::from_u32(3)?;

        let n = self.ps.get(self.index);
        self.index += 1;

        if let Some(p) = n {
            return Some(p.clone());
        }

        let last = self.ps.last();

        if last == None {
            self.ps.push(two.clone());
            Some(two.clone())
        } else if last == Some(&two) {
            self.ps.push(three.clone());
            Some(three.clone())
        } else {
            let start = last.map(|a| a.clone() + two.clone()).unwrap();
            let next_prime =
                range_step(start.clone(), start * T::from_u32(2)?, two.clone()).find(|n| {
                    self.ps
                        .iter()
                        .take_while(|&p| p.clone() * p.clone() <= *n)
                        .all(|p| n.clone() % p.clone() != zero())
                })?;

            self.ps.push(next_prime.clone());
            Some(next_prime.clone())
        }
    }
}

#[derive(Debug)]
pub struct Fibo<T> {
    current: Option<T>,
    prev: Option<T>,
}

impl<T> Iterator for Fibo<T>
where
    T: Integer + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.current.take();

        self.current = c
            .as_ref()
            .zip(self.prev.as_ref())
            .map(|(a, b)| a.clone() + b.clone());
        self.prev = c.clone();

        c
    }
}

pub fn fibo_iter<T>() -> Fibo<T>
where
    T: Integer + Clone,
{
    Fibo {
        current: Some(one::<T>()),
        prev: Some(zero::<T>()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::BigUint;

    #[test]
    fn prime_factorization() {
        let mut prime = Prime::new();

        assert_eq!(vec![2, 2, 2], *prime.factorization(8));
        assert_eq!(vec![5, 5], *prime.factorization(25));
        assert_eq!(vec![2, 2, 3, 5], *prime.factorization(60));
        assert_eq!(vec![17], *prime.factorization(17));
        assert_eq!(vec![23, 29], *prime.factorization(667));
    }

    #[test]
    fn prime_iterator() {
        let mut prime: Prime<BigUint> = Prime::new();

        let mut prime_iter = prime.iter();
        assert_eq!(Some(BigUint::from(2_u32)), prime_iter.next());
        assert_eq!(Some(BigUint::from(3_u32)), prime_iter.next());
        assert_eq!(Some(BigUint::from(5_u32)), prime_iter.next());
        assert_eq!(Some(BigUint::from(7_u32)), prime_iter.next());
        assert_eq!(Some(BigUint::from(11_u32)), prime_iter.next());
    }

    #[test]
    fn fibo() {
        let mut fibo = fibo_iter::<usize>();

        assert_eq!(Some(1), fibo.next());
        assert_eq!(Some(1), fibo.next());
        assert_eq!(Some(2), fibo.next());
        assert_eq!(Some(3), fibo.next());
        assert_eq!(Some(5), fibo.next());
        assert_eq!(Some(8), fibo.next());
    }
}
