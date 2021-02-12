use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::iter::once;

#[derive(Clone, Debug)]
pub struct Ring {
    columns: Vec<Column>,
}

impl Display for Ring {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.columns.iter().join(""))
    }
}

#[derive(Clone, Debug)]
pub struct RingBuilder {
    n_gon: usize,
    ring: Ring,
}

impl RingBuilder {
    pub fn new(n: usize) -> RingBuilder {
        RingBuilder {
            n_gon: n,
            ring: Ring { columns: vec![] },
        }
    }

    pub fn add(&mut self, col: Column) {
        self.ring.columns.push(col);
    }

    pub fn is_accept(&self, next_inner: &Vec<usize>) -> bool {
        if next_inner.len() != 2 {
            return false;
        }

        self.ring.columns.last().unwrap().inner_second() == next_inner[0]
    }

    pub fn build(&self) -> Option<Ring> {
        (self.ring.columns.len() == self.n_gon
            && self.ring.columns.first().unwrap().inner_first()
                == self.ring.columns.last().unwrap().inner_second())
        .then(|| self.ring.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Column {
    outer: usize,
    inner: Vec<usize>,
}

impl Column {
    pub fn new(outer: &usize, inner: &Vec<usize>) -> Column {
        Column {
            outer: *outer,
            inner: inner.clone(),
        }
    }

    // pub fn outer(&self) -> usize {
    //     self.numbers[0]
    // }

    pub fn inner_first(&self) -> usize {
        self.inner[0]
    }

    pub fn inner_second(&self) -> usize {
        self.inner[1]
    }
}

impl Display for Column {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            once(self.outer).chain(self.inner.iter().copied()).join("")
        )
    }
}
