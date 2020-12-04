use crate::*;

/// Function to generate random numbers
///
/// ```ignore
/// let f: impl FnRand<T> = |min, max| gen_range(min, max);
/// ```
/// ```ignore
/// use rand::thread_rng;
///
/// let rng = thread_rng();
/// let f: impl FnRand<T> = rng;
/// ```
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
