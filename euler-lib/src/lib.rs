use num::{one, range_step, zero, CheckedAdd, FromPrimitive, Integer};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::rc::Rc;

pub mod func;

pub use func::*;

#[derive(Debug)]
pub struct Prime<T> {
    prime_factor_map: RefCell<BTreeMap<T, Vec<T>>>,
    prime_list: Rc<RefCell<Vec<T>>>,
}

impl<T> Prime<T>
where
    T: Integer + CheckedAdd + Clone + FromPrimitive,
{
    pub fn new() -> Prime<T> {
        Prime {
            prime_factor_map: RefCell::new(BTreeMap::new()),
            prime_list: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn factorization(&self, n: &T) -> Vec<T> {
        if self.prime_factor_map.borrow().contains_key(n) {
            return self.prime_factor_map.borrow()[n].clone();
        }

        let mut prime_iter = PrimeIterator {
            ps: Rc::clone(&self.prime_list),
            index: 0,
        };

        let v = prime_factorization(n, &mut prime_iter, &self.prime_factor_map.borrow());
        self.prime_factor_map.borrow_mut().insert(n.clone(), v);

        self.prime_factor_map.borrow()[n].clone()
    }

    pub fn iter(&self) -> PrimeIterator<T> {
        PrimeIterator {
            ps: Rc::clone(&self.prime_list),
            index: 0,
        }
    }

    pub fn is_prime(&self, n: &T) -> bool {
        if *n <= one() {
            return false;
        }

        let f = self.factorization(n);
        f.len() == 1
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
pub struct PrimeIterator<T> {
    ps: Rc<RefCell<Vec<T>>>,
    index: usize,
}

impl<T> Iterator for PrimeIterator<T>
where
    T: Integer + CheckedAdd + Clone + FromPrimitive,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let two = T::from_u32(2)?;
        let three = T::from_u32(3)?;

        let mut mut_ps = self.ps.borrow_mut();

        let n = mut_ps.get(self.index);
        self.index += 1;

        if let Some(p) = n {
            return Some(p.clone());
        }

        let last = mut_ps.last();

        if last == None {
            mut_ps.push(two.clone());
            Some(two.clone())
        } else if last == Some(&two) {
            mut_ps.push(three.clone());
            Some(three.clone())
        } else {
            let start = last.map(|a| a.clone() + two.clone()).unwrap();
            let next_prime =
                range_step(start.clone(), start * T::from_u32(2)?, two.clone()).find(|n| {
                    mut_ps
                        .iter()
                        .take_while(|&p| p.clone() * p.clone() <= *n)
                        .all(|p| n.clone() % p.clone() != zero())
                })?;

            mut_ps.push(next_prime.clone());
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

pub struct Intersect<T1, T2>
where
    T1: Iterator,
    T2: Iterator,
{
    iter1: T1,
    iter2: T2,
}

impl<T1, T2> Iterator for Intersect<T1, T2>
where
    T1: Iterator,
    T2: Iterator<Item = T1::Item>,
    T1::Item: Ord + Copy,
{
    type Item = T1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut t1 = self.iter1.next();
        let mut t2 = self.iter2.next();

        loop {
            match t1.zip(t2).map(|(a, b)| a.cmp(&b)) {
                Some(Ordering::Less) => t1 = self.iter1.next(),
                Some(Ordering::Greater) => t2 = self.iter2.next(),
                Some(Ordering::Equal) => break,
                None => return None,
            }
        }

        t1
    }
}

pub fn iter_intersect<T1, T2>(iter1: T1, iter2: T2) -> Intersect<T1, T2>
where
    T1: Iterator,
    T2: Iterator,
{
    Intersect { iter1, iter2 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::BigUint;

    #[test]
    fn prime_factorization() {
        let prime = Prime::new();

        assert_eq!(vec![2, 2, 2], prime.factorization(&8));
        assert_eq!(vec![5, 5], prime.factorization(&25));
        assert_eq!(vec![2, 2, 3, 5], prime.factorization(&60));
        assert_eq!(vec![17], prime.factorization(&17));
        assert_eq!(vec![23, 29], prime.factorization(&667));
    }

    #[test]
    fn prime_iterator() {
        let prime: Prime<BigUint> = Prime::new();

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

    #[test]
    fn intersect() {
        let mut intersect = iter_intersect(
            vec![1, 3, 5, 9, 11, 13].into_iter(),
            vec![1, 2, 3, 5, 8, 13].into_iter(),
        );
        assert_eq!(intersect.next(), Some(1));
        assert_eq!(intersect.next(), Some(3));
        assert_eq!(intersect.next(), Some(5));
        assert_eq!(intersect.next(), Some(13));
        assert_eq!(intersect.next(), None);
    }
}
