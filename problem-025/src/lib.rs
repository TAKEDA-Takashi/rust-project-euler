use num_bigint::BigUint;

#[derive(Debug)]
pub struct Fibo {
    current: Option<BigUint>,
    prev: Option<BigUint>,
}

impl Fibo {
    /// 初項が1、第2項が2のフィボナッチ数列を生成するイテレータを生成する。
    ///
    /// # Example
    ///
    /// ```
    /// use problem_025::Fibo;
    /// use num_bigint::BigUint;
    ///
    /// let mut fibo = Fibo::new();
    /// assert_eq!(fibo.next(), Some(BigUint::from(1_u32)));
    /// assert_eq!(fibo.next(), Some(BigUint::from(1_u32)));
    /// assert_eq!(fibo.next(), Some(BigUint::from(2_u32)));
    /// ```
    pub fn new() -> Fibo {
        Fibo {
            current: Some(BigUint::from(1_u32)),
            prev: Some(BigUint::from(0_u32)),
        }
    }
}

impl Iterator for Fibo {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.current.take();

        self.current = c.as_ref().zip(self.prev.as_ref()).map(|(a, b)| a + b);
        self.prev = c.clone();

        c
    }
}
