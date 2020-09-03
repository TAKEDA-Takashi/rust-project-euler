#[derive(Debug)]
pub struct Fibo {
    current: usize,
    prev: usize,
}

impl Fibo {
    /// 初項が1、第2項が2のフィボナッチ数列を生成するイテレータを生成する。
    ///
    /// # Example
    ///
    /// ```
    /// use problem_2::Fibo;
    ///
    /// let mut fibo = Fibo::new();
    /// assert_eq!(fibo.next(), Some(1));
    /// assert_eq!(fibo.next(), Some(2));
    /// ```
    pub fn new() -> Fibo {
        Fibo {
            current: 1,
            prev: 0,
        }
    }
}

impl Iterator for Fibo {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current + self.prev;
        self.prev = self.current;
        self.current = next;

        Some(next)
    }
}
