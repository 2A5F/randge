mod fn_rand;
pub mod impls;
mod linear;
mod tree;
mod utils;
pub use fn_rand::*;
use impls::*;
use num_traits::{one, zero, PrimInt, Zero};
use std::{fmt::Debug, ops::Range};
use utils::*;

#[inline(always)]
fn check<T: PrimInt>(range: Range<T>, n: T) -> (T, T, T) {
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
    (size.min(n), min, max)
}

#[inline]
pub fn randge_linear<T: PrimInt>(
    range: Range<T>,
    n: T,
    rand: impl FnRand<T>,
) -> impl Iterator<Item = T> {
    let (len, min, max) = check(range, n);
    let take = RangesLinear::new(min, max);
    RandgeIter::new(len, take, rand)
}

#[inline]
pub fn randge_tree<T: PrimInt>(
    range: Range<T>,
    n: T,
    rand: impl FnRand<T>,
) -> impl Iterator<Item = T> {
    let (len, min, max) = check(range, n);
    let take = RangesTree::new(min, max);
    RandgeIter::new(len, take, rand)
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_vec() {
        let v = randge_linear(5..15, 5, thread_rng());
        let v: Vec<_> = v.collect();
        println!("{:?}", v);
    }

    #[test]
    fn test_tree() {
        let v = randge_tree(0..10, 5, thread_rng());
        let v: Vec<_> = v.collect();
        println!("{:?}", v);
    }
}
