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
    len: T,
    take: R,
    rand: F,
}

impl<T, F, R> RandgeIter<T, F, R> {
    #[inline(always)]
    pub fn new(len: T, take: R, rand: F) -> Self {
        Self { len, take, rand }
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
        if self.len.is_zero() {
            return None;
        }
        self.len = self.len - one();
        let range = self.take.range();
        let rand = self.rand.rand(range.clone());
        if rand >= range.end || rand < range.start {
            panic!("Random number out of range")
        }
        let num = self.take.take(rand);
        Some(num)
    }
}
