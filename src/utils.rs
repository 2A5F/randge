use crate::*;

#[inline(always)]
pub fn abs<T: Zero + PartialOrd + Sub<Output = T>>(v: T) -> T {
    if v < zero() {
        zero::<T>() - v
    } else {
        v
    }
}

#[inline(always)]
pub fn is_negative<T: Zero + PartialOrd>(v: T) -> bool {
    v < zero()
}
