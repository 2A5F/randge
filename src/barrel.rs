use crate::*;
use num_traits::AsPrimitive;
use std::mem::swap;

#[derive(Debug, Clone)]
pub struct RangesBarrel<T> {
    size: Range<T>,
    barrel: Box<[T]>,
}

impl<T> RangesBarrel<T>
where
    T: PrimInt + AsPrimitive<usize>,
    Range<T>: Iterator<Item = T>,
{
    #[inline]
    pub fn new(min: T, max: T) -> Self {
        Self {
            size: min..max,
            barrel: (min..max + one()).collect(),
        }
    }
}

impl<T> RangesBarrel<T>
where
    T: PrimInt + AsPrimitive<usize>,
{
    #[inline]
    fn offset(&self, val: T) -> usize {
        (zero::<T>() - self.size.start + val).as_()
    }
}

impl<T> RandgeTake<T> for RangesBarrel<T>
where
    T: PrimInt + AsPrimitive<usize>,
{
    fn range(&self) -> Range<T> {
        self.size.clone()
    }

    fn take(&mut self, num: T) -> T {
        debug_assert!(self.size.start <= num && num < self.size.end);
        let index: usize = self.offset(num);
        let max = self.offset(self.size.end);
        self.size.end = self.size.end - one();
        let barrel = &mut self.barrel[index..max + 1];
        let (val, max) = if let [first, .., last] = barrel { Some((first, last)) } else { None }.unwrap();
        let num = *val;
        swap(val, max);
        num
    }
}

#[test]
fn test() {
    println!("");
    let mut t = RangesBarrel::new(5, 15);
    let v = t.take(5);
    println!("{}\n{:?}\n", v, t);
    let v = t.take(5);
    println!("{}\n{:?}\n", v, t);
    let v = t.take(5);
    println!("{}\n{:?}\n", v, t);
}
