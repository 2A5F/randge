mod linear;
mod tree;
mod utils;
use linear::*;
use num_traits::{one, zero, PrimInt, Zero};
use std::{fmt::Debug, ops::Range};
use std::{marker::PhantomData, ops::Sub};
use tree::*;
use utils::*;

pub trait FnRand<T> {
    fn rand(&mut self, range: Range<T>) -> T;
}
impl<T, F: FnMut(T, T) -> T> FnRand<T> for F {
    fn rand(&mut self, range: Range<T>) -> T {
        self(range.start, range.end)
    }
}

#[cfg(feature = "rand")]
use rand::{
    distributions::uniform::SampleUniform,
    rngs::{OsRng, StdRng, ThreadRng},
    Rng,
};

#[cfg(feature = "rand")]
impl<T: SampleUniform> FnRand<T> for ThreadRng {
    fn rand(&mut self, range: Range<T>) -> T {
        self.gen_range(range.start, range.end)
    }
}

#[cfg(feature = "rand")]
impl<T: SampleUniform> FnRand<T> for OsRng {
    fn rand(&mut self, range: Range<T>) -> T {
        self.gen_range(range.start, range.end)
    }
}

#[cfg(feature = "rand")]
impl<T: SampleUniform> FnRand<T> for StdRng {
    fn rand(&mut self, range: Range<T>) -> T {
        self.gen_range(range.start, range.end)
    }
}

pub struct Randge<T> {
    _t: PhantomData<T>,
}

impl<T: PrimInt> Randge<T> {
    #[inline(always)]
    fn check(range: Range<T>, n: T) -> (T, T, T, T) {
        let (min, max) = (range.start.min(range.end), range.start.max(range.end));
        let size = abs(max - min);
        if size.is_zero() {
            panic!("Range cannot be 0")
        }
        if is_negative(n) {
            panic!("n must be >= 0")
        }
        if size < n {
            panic!("The required count is greater than the allowed range")
        }
        (size.min(n), max, min, max)
    }

    #[inline]
    pub fn new(range: Range<T>, n: T, rand: impl FnRand<T>) -> impl Iterator<Item = T> {
        Self::vec(range, n, rand)
    }

    #[inline]
    pub fn vec(range: Range<T>, n: T, rand: impl FnRand<T>) -> impl Iterator<Item = T> {
        let (len, size, min, max) = Self::check(range, n);
        let take = RangesVec::new(min, max);
        RandgeIter::new(len, size, take, rand)
    }

    #[inline]
    pub fn tree(range: Range<T>, n: T, rand: impl FnRand<T>) -> impl Iterator<Item = T> {
        let (len, size, min, max) = Self::check(range, n);
        let take = RangesTree::new(min, max);
        RandgeIter::new(len, size, take, rand)
    }
}

trait RandgeTake<T> {
    fn range(&self) -> Range<T>;

    fn take(&mut self, num: T) -> T;
}

#[derive(Clone)]
struct RandgeIter<T, F, R> {
    len: T,
    size: T,
    take: R,
    rand: F,
}

impl<T, F, R> RandgeIter<T, F, R> {
    #[inline(always)]
    pub fn new(len: T, size: T, take: R, rand: F) -> Self {
        Self {
            len,
            size,
            take,
            rand,
        }
    }
}

impl<T: Debug, F, R: RandgeTake<T> + Debug> Debug for RandgeIter<T, F, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Randge")
            .field("len", &self.len)
            .field("size", &self.size)
            .field("take", &self.take)
            .finish()
    }
}

impl<T, F, R> RandgeIter<T, F, R>
where
    T: PrimInt,
{
    #[inline]
    fn move_next(&mut self) {
        self.len = self.len - one();
        self.size = self.size - one();
    }
}

impl<T, F: FnRand<T>, R: RandgeTake<T>> Iterator for RandgeIter<T, F, R>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len.is_zero() {
            return None;
        }
        let range = self.take.range();
        let rand = self.rand.rand(range.clone());
        if rand >= range.end || rand < range.start {
            panic!("Random number out of range")
        }
        let num = self.take.take(rand);
        self.move_next();
        Some(num)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_vec() {
        let v = Randge::vec(0..10, 5, thread_rng());
        let v: Vec<_> = v.collect();
        println!("{:?}", v);
    }

    #[test]
    fn test_tree() {
        let v = Randge::tree(0..10, 5, thread_rng());
        let v: Vec<_> = v.collect();
        println!("{:?}", v);
    }
}
