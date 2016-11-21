use num::Num;


#[inline(always)]
pub fn new<T: Num>(m11: T, m12: T, m13: T, m21: T, m22: T, m23: T) -> [T; 6] {
    [
        m11, m21,
        m12, m22,
        m13, m23
    ]
}
#[inline(always)]
pub fn create<T: Num>(m11: T, m12: T, m13: T, m21: T, m22: T, m23: T) -> [T; 6] {
    new(m11, m12, m13, m21, m22, m23)
}
#[test]
fn test_new() {
    let m = new(
        1, 0, 0,
        0, 1, 0
    );
    assert!(m == [
        1, 0,
        0, 1,
        0, 0
    ]);
}

#[inline(always)]
pub fn new_identity<T: Num>() -> [T; 6] {
    new(
        T::one(), T::zero(), T::zero(),
        T::zero(), T::one(), T::zero()
    )
}
#[inline(always)]
pub fn new_zero<T: Num>() -> [T; 6] {
    new(
        T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero()
    )
}

#[inline(always)]
pub fn clone<'b, T: Num>(m: &'b [T; 6]) -> [T; 6] {new(m[0], m[1], m[2], m[3], m[4], m[5])}

#[inline(always)]
pub fn copy<'a, 'b, T: Num>(out: &'a mut [T; 6], a: &'b [T; 6]) -> &'a mut [T; 6] {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out[3] = a[3];
    out[4] = a[4];
    out[5] = a[5];
    out
}
#[test]
fn test_copy() {
    let mut v = [0, 0, 0, 0, 0, 0];
    copy(&mut v, &[1, 2, 3, 4, 5, 6]);
    assert!(v == [1, 2, 3, 4, 5, 6]);
}
