//! You don’t need to view the contents of this

use std::marker::PhantomData;

pub use crate::barrel::*;
pub use crate::linear::*;
pub use crate::tree::*;
use crate::*;

pub trait RandgeTake<T> {
    fn range(&self) -> Range<T>;

    fn take(&mut self, num: T) -> T;
}

#[derive(Clone)]
pub struct RandgeIter<T, F, R> {
    _t: PhantomData<T>,
    len: usize,
    take: R,
    rand: F,
}

impl<T, F, R> RandgeIter<T, F, R> {
    #[inline(always)]
    pub fn new(len: usize, take: R, rand: F) -> Self {
        Self {
            _t: PhantomData,
            len,
            take,
            rand,
        }
    }
}

impl<T: Debug, F, R: RandgeTake<T> + Debug> Debug for RandgeIter<T, F, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Randge").field("len", &self.len).field("take", &self.take).finish()
    }
}

impl<T, F: FnRand<T>, R: RandgeTake<T>> Iterator for RandgeIter<T, F, R>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let range = self.take.range();
        let rand = self.rand.rand(range.clone());
        if rand >= range.end || rand < range.start {
            panic!("Random number out of range")
        }
        let num = self.take.take(rand);
        Some(num)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.len > 0 {
            self.len = 1;
            self.next()
        } else {
            None
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len.as_() {
            self.len = 0;
            None
        } else {
            self.len -= n - 1;
            self.next()
        }
    }
}
