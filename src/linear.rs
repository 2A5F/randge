use crate::*;

#[derive(Debug, Clone)]
pub struct RangesVec<T> {
    rs: Vec<Range<T>>,
    size: T,
}

impl<T> RangesVec<T>
where
    T: PrimInt,
{
    #[inline]
    pub fn new(min: T, max: T) -> Self {
        Self {
            rs: vec![min..max],
            size: max,
        }
    }
}

impl<T> RandgeTake<T> for RangesVec<T>
where
    T: PrimInt,
{
    fn range(&self) -> Range<T> {
        zero()..self.size
    }

    #[inline]
    fn take(&mut self, mut num: T) -> T {
        self.size = self.size - one();
        num = num + self.rs[0].start - zero();
        for i in 0..self.rs.len() {
            let v = unsafe { self.rs.get_unchecked_mut(i) };
            let start = v.start;
            debug_assert!(num >= start);
            let end = v.end;
            if num >= end {
                num = num + self.rs[i + 1].start - end;
                continue;
            }
            let nstart = num + one();
            let nend = num;
            if nstart == end {
                if nend == start {
                    self.rs.remove(i);
                } else {
                    v.end = nend;
                }
            } else {
                v.start = nstart;
                if nend != start {
                    self.rs.insert(i, start..nend);
                }
            }
            return num;
        }
        panic!("never")
    }
}
