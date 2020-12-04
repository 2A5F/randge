//! Generate random numbers that are not repeated in the range  
//! 
//! # Example
//! 
//! ```
//! use rand::thread_rng;
//! # use randge::*;
//! 
//! let v = randge(-15..15, 5, thread_rng());
//! let v: Vec<_> = v.collect();
//! # drop(v);
//! // output: like [13, -3, -14, 5, 3]
//! ```
//!
//! # Features
//! - `rand`
//! 

mod barrel;
mod fn_rand;
pub mod impls;
mod linear;
mod tree;
mod utils;
pub use fn_rand::*;
use impls::*;
use num_traits::{one, zero, AsPrimitive, PrimInt, Zero};
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

/// Alias of [`randge_tree`](fn.randge_tree.html)
#[inline(always)]
pub fn randge<T: PrimInt>(range: Range<T>, n: T, rand: impl FnRand<T>) -> RandgeIter<T, impl FnRand<T>, RangesTree<T>> {
    randge_tree(range, n, rand)
}

/// Generate random numbers that are not repeated in the range  
/// - Based on linear algorithm, the worst time complexity is `O(n)` and the best `O(1)`
/// - Minimal memory usage
/// - Slowest
/// 
/// # Example
/// 
/// ```
/// use rand::thread_rng;
/// # use randge::*;
/// 
/// let v = randge_linear(-15..15, 5, thread_rng());
/// let v: Vec<_> = v.collect();
/// # drop(v);
/// // output: like [13, -3, -14, 5, 3]
/// ```
#[inline]
pub fn randge_linear<T: PrimInt>(range: Range<T>, n: T, rand: impl FnRand<T>) -> RandgeIter<T, impl FnRand<T>, RangesLinear<T>> {
    let (len, min, max) = check(range, n);
    let take = RangesLinear::new(min, max);
    RandgeIter::new(len, take, rand)
}

/// Generate random numbers that are not repeated in the range  
/// - Using tree-based algorithm, the time complexity is `O(logn)`
/// - Moderate memory usage
/// 
/// # Example
/// 
/// ```
/// use rand::thread_rng;
/// # use randge::*;
/// 
/// let v = randge_tree(-15..15, 5, thread_rng());
/// let v: Vec<_> = v.collect();
/// # drop(v);
/// // output: like [13, -3, -14, 5, 3]
/// ```
#[inline]
pub fn randge_tree<T: PrimInt>(range: Range<T>, n: T, rand: impl FnRand<T>) -> RandgeIter<T, impl FnRand<T>, RangesTree<T>> {
    let (len, min, max) = check(range, n);
    let take = RangesTree::new(min, max);
    RandgeIter::new(len, take, rand)
}

/// Generate random numbers that are not repeated in the range  
/// - Space for time, time complexity is `O(1)`
/// - Maximum memory usage
/// - Fastest
/// 
/// # Example
/// 
/// ```
/// use rand::thread_rng;
/// # use randge::*;
/// 
/// let v = randge_barrel(-15..15, 5, thread_rng());
/// let v: Vec<_> = v.collect();
/// # drop(v);
/// // output: like [13, -3, -14, 5, 3]
/// ```
#[inline]
pub fn randge_barrel<T: PrimInt>(range: Range<T>, n: T, rand: impl FnRand<T>) -> RandgeIter<T, impl FnRand<T>, RangesBarrel<T>>
where
    T: AsPrimitive<usize>,
    Range<T>: Iterator<Item = T>,
{
    let (len, min, max) = check(range, n);
    let take = RangesBarrel::new(min, max);
    RandgeIter::new(len, take, rand)
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_linear() {
        let v = randge_linear(5..15, 5, thread_rng());
        let v: Vec<_> = v.collect();
        println!("{:?}", v);
    }

    #[test]
    fn test_tree() {
        let v = randge_tree(5..15, 5, thread_rng());
        let v: Vec<_> = v.collect();
        println!("{:?}", v);
    }

    #[test]
    fn test_barrel() {
        let v = randge_barrel(5..15, 5, thread_rng());
        let v: Vec<_> = v.collect();
        println!("{:?}", v);
    }
}
