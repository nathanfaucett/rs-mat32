use num::Num;


#[inline(always)]
pub fn set<'a, 'b, T: Num>(out: &'a mut [T; 6], m11: T, m12: T, m13: T, m21: T, m22: T, m23: T) -> &'a mut [T; 6] {
    out[0] = m11; out[2] = m21; out[4] = m13;
    out[1] = m12; out[3] = m22; out[5] = m23;
    out
}
#[test]
fn test_set() {
    let mut v = [0, 0, 0, 0, 0, 0];
    set(&mut v,
        1, 2, 3,
        4, 5, 6
    );
    assert!(v == [1, 2, 4, 5, 3, 6]);
}

#[inline(always)]
pub fn zero<'a, 'b, T: Num>(out: &'a mut [T; 6]) -> &'a mut [T; 6] { set(out, T::zero(), T::zero(), T::zero(), T::zero(), T::zero(), T::zero()) }
#[inline(always)]
pub fn identity<'a, 'b, T: Num>(out: &'a mut [T; 6]) -> &'a mut [T; 6] { set(out, T::one(), T::zero(), T::zero(), T::zero(), T::one(), T::zero()) }
