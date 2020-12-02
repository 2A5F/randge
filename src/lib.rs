use num_traits::{one, zero, PrimInt, Zero};
use std::{fmt::Debug, ops::Range};
use std::{marker::PhantomData, ops::Sub};

#[inline(always)]
fn abs<T: Zero + PartialOrd + Sub<Output = T>>(v: T) -> T {
    if v < zero() {
        zero::<T>() - v
    } else {
        v
    }
}

#[inline(always)]
fn is_negative<T: Zero + PartialOrd>(v: T) -> bool {
    v < zero()
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
    pub fn new(range: Range<T>, n: T, rand: impl FnMut(T) -> T) -> impl Iterator<Item = T> {
        let (len, size, min, max) = Self::check(range, n);
        let take = RangesVec::new(min, max);
        RandgeIter::new(len, size, take, rand)
    }
}

trait RandgeTake<T> {
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

impl<T, F: FnMut(T) -> T, R: RandgeTake<T>> Iterator for RandgeIter<T, F, R>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len.is_zero() {
            return None;
        }
        let rand = (self.rand)(self.size);
        if rand >= self.size {
            panic!("Random number out of range")
        }
        let num = self.take.take(rand);
        self.move_next();
        Some(num)
    }
}

#[derive(Debug, Clone)]
struct RangesVec<T>(Vec<Range<T>>);

impl<T> RangesVec<T> {
    #[inline]
    pub fn new(min: T, max: T) -> Self {
        Self(vec![min..max])
    }
}

impl<T> RandgeTake<T> for RangesVec<T>
where
    T: PrimInt,
{
    #[inline]
    fn take(&mut self, mut num: T) -> T {
        num = num + self.0[0].start - zero();
        for i in 0..self.0.len() {
            let v = unsafe { self.0.get_unchecked_mut(i) };
            let start = v.start;
            debug_assert!(num >= start);
            let end = v.end;
            if num >= end {
                num = num + self.0[i + 1].start - end;
                continue;
            }
            let nstart = num + one();
            let nend = num;
            if nstart == end {
                if nend == start {
                    self.0.remove(i);
                } else {
                    v.end = nend;
                }
            } else {
                v.start = nstart;
                if nend != start {
                    self.0.insert(i, start..nend);
                }
            }
            return num;
        }
        panic!("never")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn test() {
        let mut rng = thread_rng();
        let v = Randge::new(0..10, 5, |max| rng.gen_range(0, max));
        let v: Vec<_> = v.collect();
        println!("{:?}", v);
    }
}
