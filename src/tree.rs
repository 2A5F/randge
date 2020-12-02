use crate::*;

#[derive(Debug, Clone)]
pub enum RangesTree<T> {
    Range(Range<T>),
    Tree {
        l: Box<RangesTree<T>>,
        r: Box<RangesTree<T>>,
    },
}

impl<T> RangesTree<T> {
    #[inline]
    pub fn new(min: T, max: T) -> Self {
        Self::Range(min..max)
    }
}

impl<T> RandgeTake<T> for RangesTree<T>
where
    T: PrimInt,
{
    fn range(&self) -> Range<T> {
        todo!()
    }

    fn take(&mut self, num: T) -> T {
        let mut this = self;
        match this {
            RangesTree::Range(v) => {
                debug_assert!(v.start >= num && num < v.end);
            }
            RangesTree::Tree { l, r } => {}
        }
        todo!()
    }
}
