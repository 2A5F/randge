#![allow(dead_code)]
use std::{fmt::Display, ptr::NonNull};

use crate::*;

impl<T: Debug> Display for RangesTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RangesTree (size: {:?}) ", self.size)?;
        self.tree.show(f, "  ", true)
    }
}

fn show_tab(f: &mut std::fmt::Formatter<'_>, tab: &str, lf: bool) -> std::fmt::Result {
    if !lf {
        write!(f, "{}", tab)?;
    }
    Ok(())
}

impl<T: Debug> TheTree<T> {
    fn show(&self, f: &mut std::fmt::Formatter<'_>, tab: &str, lf: bool) -> std::fmt::Result {
        show_tab(f, tab, lf)?;
        write!(f, "{:?} (range: {:?})", self.lr, self.range)?;

        if let Some(tree) = &self.tree {
            let ntab = format!("{}|  ", tab);

            writeln!(f, "")?;

            show_tab(f, tab, false)?;
            write!(f, "l: ")?;
            tree.l.show(f, &ntab, true)?;

            writeln!(f, "")?;

            show_tab(f, tab, false)?;
            write!(f, "r: ")?;
            tree.r.show(f, &ntab, true)?;
        }

        Ok(())
    }
}

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
    /// Temporary pointer to the parent on the stack(maybe)
    parent: Option<NonNull<TheTree<T>>>,
}

impl<T> TheTree<T> {
    pub fn new(range: Range<T>) -> Self {
        Self {
            range,
            tree: None,
            lr: TheTreeLR::Root,
            parent: None,
        }
    }
    pub fn l(range: Range<T>) -> Self {
        Self {
            range,
            tree: None,
            lr: TheTreeLR::L,
            parent: None,
        }
    }
    pub fn r(range: Range<T>) -> Self {
        Self {
            range,
            tree: None,
            lr: TheTreeLR::R,
            parent: None,
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
                            tree = unsafe { &mut *tree.parent.unwrap().as_ptr() };
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

                'sync: loop {
                    if let TheTreeLR::Root = tree.lr {
                        break 'sync;
                    }
                    let parent = unsafe { &mut *tree.parent.unwrap().as_ptr() };
                    parent.range.start = tree.range.start;
                    let pt = parent.tree.as_deref().unwrap();
                    parent.range.end = parent.range.start
                        + (pt.l.range.end - pt.l.range.start)
                        + (pt.r.range.end - pt.r.range.start);
                    tree = parent;
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
                r.parent = Some(unsafe { NonNull::new_unchecked(parent_ptr) });
                tree = r;
                continue 'root;
            }
            // select l
            else {
                l.parent = Some(unsafe { NonNull::new_unchecked(parent_ptr) });
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
    let v = t.take(6);
    println!("{}\n{}\n", v, t);
    let v = t.take(9);
    println!("{}\n{}\n", v, t);
    let v = t.take(8);
    println!("{}\n{}\n", v, t);
    let v = t.take(7);
    println!("{}\n{}\n", v, t);
}
