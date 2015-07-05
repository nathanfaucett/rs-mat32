use num::Num;
use set::identity;


#[inline(always)]
pub fn inverse<T: Num>(out: &mut [T; 6], a: [T; 6]) -> &mut [T; 6] {
    let m11 = a[0];
    let m12 = a[2];
    let m13 = a[4];
    let m21 = a[1];
    let m22 = a[3];
    let m23 = a[5];

    let d = m11 * m22 - m12 * m21;
    if d != T::zero() {
        let inv_d = T::one() / d;

        out[0] = m22 * inv_d;
        out[1] = -m12 * inv_d;
        out[2] = -m21 * inv_d;
        out[3] = m11 * inv_d;
        out[4] = (m21 * m23 - m22 * m13) * inv_d;
        out[5] = -(m11 * m23 - m12 * m13) * inv_d;
        out
    } else {
        identity(out)
    }
}
#[test]
fn test_inverse() {
    let mut v = [0, 0, 0, 0, 0, 0];
    inverse(&mut v, [1, 0, 0, 1, 0, 0]);
    assert!(v == [1, 0, 0, 1, 0, 0]);
}

#[inline(always)]
pub fn determinant<T: Num>(out: [T; 6]) -> T {
    out[0] * out[3] - out[2] * out[1]
}
#[test]
fn test_determinant() {
    assert_eq!(determinant([1, 0, 0, 1, 0, 0]), 1);
}

#[inline(always)]
pub fn transpose<T: Num>(out: &mut [T; 6], a: [T; 6]) -> &mut [T; 6] {
    out[0] = a[0];
    out[1] = a[2];
    out[2] = a[1];
    out[3] = a[3];
    out
}
#[test]
fn test_transpose() {
    let mut v = [0, 0, 0, 0, 0, 0];
    transpose(&mut v, [1, 0, 0, 1, 0, 0]);
    assert_eq!(v, [1, 0, 0, 1, 0, 0]);
}
