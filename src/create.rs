use num::Num;


#[inline(always)]
pub fn create<T: Num>(m11: T, m12: T, m13: T, m21: T, m22: T, m23: T) -> [T; 6] {
    [m11, m12, m13, m21, m22, m23]
}
#[test]
fn test_create() {
    let m = create(
        1, 0, 0,
        0, 1, 0
    );
    assert!(m == [
        1, 0, 0,
        0, 1, 0
    ]);
}

#[inline(always)]
pub fn create_identity<T: Num>() -> [T; 6] {
    create(
        T::one(), T::zero(), T::zero(),
        T::zero(), T::one(), T::zero()
    ) 
}
#[inline(always)]
pub fn create_zero<T: Num>() -> [T; 6] {
    create(
        T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero()
    )
}

#[inline(always)]
pub fn clone<T: Num>(m: [T; 6]) -> [T; 6] {create(m[0], m[1], m[2], m[3], m[4], m[5])}

#[inline(always)]
pub fn copy<T: Num>(out: &mut [T; 6], a: [T; 6]) -> &mut [T; 6] {
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
    copy(&mut v, [1, 2, 3, 4, 5, 6]);
    assert!(v == [1, 2, 3, 4, 5, 6]);
}
