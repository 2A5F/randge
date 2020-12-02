use num_traits::{one, zero, PrimInt, Signed, Unsigned, Zero};
use std::marker::PhantomData;
use std::ops::Range;

pub struct Randge<T> {
    _t: PhantomData<T>,
}

impl<T> Randge<T>
where
    T: PrimInt + Signed,
{
    #[inline]
    pub fn int(min: T, max: T, n: T, rand: impl FnMut(T) -> T) -> impl Iterator<Item = T> {
        let (min, max) = (min.min(max), min.max(max));
        let size = (max - min).abs();
        if size.is_zero() {
            panic!("Range cannot be 0")
        }
        if n.is_negative() {
            panic!("n must be >= 0")
        }
        if size < n {
            panic!("The required count is greater than the allowed range")
        }
        RandgeIter {
            len: size.min(n),
            max,
            tree: Ranges::new(min, max),
            rand,
        }
    }
}

impl<T> Randge<T>
where
    T: PrimInt + Unsigned,
{
    #[inline]
    pub fn uint(min: T, max: T, n: T, rand: impl FnMut(T) -> T) -> impl Iterator<Item = T> {
        let (min, max) = (min.min(max), min.max(max));
        let size = max - min;
        if size.is_zero() {
            panic!("Range cannot be 0")
        }
        if size > n {
            panic!("The required count is greater than the allowed range")
        }
        RandgeIter {
            len: size.min(n),
            max,
            tree: Ranges::new(min, max),
            rand,
        }
    }
}

#[derive(Debug, Clone)]
struct RandgeIter<T, F> {
    len: T,
    max: T,
    tree: Ranges<T>,
    rand: F,
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
        todo!()
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
        let num = self.tree.take(rand);
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
        let mut items = [9, 7, 1, 5, 0].iter();
        let mut rng = thread_rng();
        let v = Randge::int(0, 10, 5, |max| {
            // let r = rng.gen_range(0, max);
            // println!("{}", r);
            // r
            //max
            //*items.next().unwrap()
            0
        });
        let v: Vec<_> = v.collect();
        println!("{:?}", v);
    }
}
