#![allow(dead_code)]
use crate::*;

#[derive(Debug, Clone)]
pub struct RangesTree<T> {
    tree: TheTree<T>,
}

#[derive(Debug, Clone)]
enum TheTree<T> {
    Range(Range<T>),
    Tree {
        l: Box<TheTree<T>>,
        r: Box<TheTree<T>>,
    },
}

impl<T> RangesTree<T> {
    #[inline]
    pub fn new(min: T, max: T) -> Self {
        Self {
            tree: TheTree::Range(min..max),
        }
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
        let mut tree = &mut self.tree;
        match &mut tree {
            TheTree::Range(v) => {
                debug_assert!(v.start >= num && num < v.end);
            }
            TheTree::Tree { l: _, r: _ } => {}
        }
        todo!()
    }
}
