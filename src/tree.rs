#![allow(dead_code)]
use std::ptr::NonNull;

use crate::*;

#[derive(Debug, Clone)]
pub struct RangesTree<T> {
    size: Range<T>,
    tree: TheTree<T>,
}

#[derive(Debug, Clone)]
struct TheTree<T> {
    range: Range<T>,
    tree: Option<Box<TheTreeSub<T>>>,
    lr: TheTreeLR,
}

impl<T> TheTree<T> {
    pub fn new(range: Range<T>) -> Self {
        Self {
            range,
            tree: None,
            lr: TheTreeLR::Root,
        }
    }
    pub fn l(range: Range<T>) -> Self {
        Self {
            range,
            tree: None,
            lr: TheTreeLR::L,
        }
    }
    pub fn r(range: Range<T>) -> Self {
        Self {
            range,
            tree: None,
            lr: TheTreeLR::R,
        }
    }
}

#[derive(Debug, Clone)]
struct TheTreeSub<T> {
    l: TheTree<T>,
    r: TheTree<T>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum TheTreeLR {
    Root,
    L,
    R,
}

impl<T> RangesTree<T>
where
    T: PrimInt,
{
    #[inline]
    pub fn new(min: T, max: T) -> Self {
        Self {
            size: min..max,
            tree: TheTree::new(min..max),
        }
    }
}

impl<T> RandgeTake<T> for RangesTree<T>
where
    T: PrimInt,
{
    fn range(&self) -> Range<T> {
        self.size.clone()
    }

    fn take(&mut self, mut num: T) -> T {
        self.size.end = self.size.end - one();
        let mut tree = &mut self.tree;
        let mut parent: Option<NonNull<TheTree<T>>> = None;
        num = num + tree.range.start - self.size.start;
        'root: loop {
            if let None = tree.tree {
                let start = tree.range.start;
                let end = tree.range.end;
                debug_assert!(start <= num && num < end);

                let nstart = num + one();
                let nend = num;
                let l = start..nend;
                let r = nstart..end;
                if l.start == l.end {
                    if r.start == r.end {
                        let lr = tree.lr;
                        'remove: loop {
                            if let TheTreeLR::Root = lr {
                                break 'remove;
                            }
                            tree = unsafe { &mut *parent.unwrap().as_ptr() };
                            let sub = tree.tree.take().unwrap();
                            if let TheTreeLR::L = lr {
                                tree.range = sub.r.range;
                                tree.tree = sub.r.tree;
                            } else {
                                tree.range = sub.l.range;
                                tree.tree = sub.l.tree;
                            }
                            break 'remove;
                        }
                    } else {
                        tree.range = r;
                    }
                } else if r.start == r.end {
                    tree.range = l;
                } else {
                    let sub = Box::new(TheTreeSub {
                        l: TheTree::l(l),
                        r: TheTree::r(r),
                    });
                    tree.tree = Some(sub);
                    tree.range.end = end - one();
                }
                return num;
            }

            let parent_ptr = tree as *mut _;
            let TheTreeSub { l, r } = tree.tree.as_deref_mut().unwrap();
            debug_assert!(num >= l.range.start);
            // select r
            if num >= l.range.end {
                num = num + r.range.start - l.range.end;
                debug_assert!(num >= r.range.start);
                parent = Some(unsafe { NonNull::new_unchecked(parent_ptr) });
                tree = r;
                continue 'root;
            }
            // select l
            else {
                parent = Some(unsafe { NonNull::new_unchecked(parent_ptr) });
                tree = l;
                continue 'root;
            }
        }
    }
}

#[test]
fn test() {
    println!("");
    let mut t = RangesTree::new(5, 15);
    let v = t.take(12);
    println!("{}\n{:?}\n", v, t);
    let v = t.take(6);
    println!("{}\n{:?}\n", v, t);
    let v = t.take(5);
    println!("{}\n{:?}\n", v, t);
    let v = t.take(7);
    println!("{}\n{:?}\n", v, t);
}
