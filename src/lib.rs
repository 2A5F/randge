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

impl<T: PrimInt + Debug> Randge<T> {
    #[inline]
    pub fn new(min: T, max: T, n: T, rand: impl FnMut(T) -> T) -> impl Iterator<Item = T> + Debug {
        let (min, max) = (min.min(max), min.max(max));
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
        RandgeIter {
            len: size.min(n),
            max,
            vec: Ranges::new(min, max),
            rand,
        }
    }
}

#[derive(Clone)]
struct RandgeIter<T, F> {
    len: T,
    max: T,
    vec: Ranges<T>,
    rand: F,
}

impl<T: Debug, F> Debug for RandgeIter<T, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Randge")
            .field("len", &self.len)
            .field("max", &self.max)
            .field("vec", &self.vec)
            .finish()
    }
}

#[derive(Debug, Clone)]
struct Ranges<T>(Vec<Range<T>>);

impl<T> Ranges<T>
where
    T: Zero + Copy,
{
    #[inline]
    pub fn new(min: T, max: T) -> Self {
        Self(vec![min..max])
    }
}

impl<T> Ranges<T>
where
    T: PrimInt,
{
    #[inline]
    pub fn take(&mut self, mut num: T) -> T {
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

impl<T, F> RandgeIter<T, F>
where
    T: PrimInt,
{
    #[inline]
    fn move_next(&mut self) {
        self.len = self.len - one();
        self.max = self.max - one();
    }
}

impl<T, F: FnMut(T) -> T> Iterator for RandgeIter<T, F>
where
    T: PrimInt,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len.is_zero() {
            return None;
        }
        let rand = (self.rand)(self.max);
        if rand >= self.max {
            panic!("Random number out of range")
        }
        let num = self.vec.take(rand);
        self.move_next();
        Some(num)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn test() {
        let mut rng = thread_rng();
        let v = Randge::new(0, 10, 5, |max| rng.gen_range(0, max));
        let v: Vec<_> = v.collect();
        println!("{:?}", v);
    }

    #[test]
    fn test_2() {
        let mut rng = thread_rng();
        let mut v = Randge::new(0, 105, 100, |max| rng.gen_range(0, max));
        let v2: Vec<_> = (&mut v).collect();
        println!("{:?}", v);
        println!("");
        println!("{:?}", v2);
    }
}
