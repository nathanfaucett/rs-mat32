use approx_eq::ApproxEq;
use num::Num;
use signed::Signed;

use set::identity;


#[inline]
pub fn inverse<'a, 'b, T: Copy + Signed>(out: &'a mut [T; 6], a: &'b [T; 6]) -> &'a mut [T; 6] {
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
    inverse(&mut v, &[1, 0, 0, 1, 0, 0]);
    assert!(v == [1, 0, 0, 1, 0, 0]);
}

#[inline]
pub fn determinant<'a, 'b, T: Copy + Num>(out: &'b [T; 6]) -> T {
    out[0] * out[3] - out[2] * out[1]
}
#[test]
fn test_determinant() {
    assert_eq!(determinant(&[1, 0, 0, 1, 0, 0]), 1);
}

#[inline]
pub fn transpose<'a, 'b, T: Copy + Num>(out: &'a mut [T; 6], a: &'b [T; 6]) -> &'a mut [T; 6] {
    out[0] = a[0];
    out[1] = a[2];
    out[2] = a[1];
    out[3] = a[3];
    out
}
#[test]
fn test_transpose() {
    let mut v = [0, 0, 0, 0, 0, 0];
    transpose(&mut v, &[1, 0, 0, 1, 0, 0]);
    assert_eq!(v, [1, 0, 0, 1, 0, 0]);
}


#[inline]
pub fn eq<'a, T: Copy + Num + ApproxEq>(a: &'a [T; 6], b: &'a [T; 6]) -> bool {
    !ne(a, b)
}

#[inline]
pub fn ne<'a, T: Copy + Num + ApproxEq>(a: &'a [T; 6], b: &'a [T; 6]) -> bool {
    !a[0].approx_eq(&b[0]) ||
    !a[1].approx_eq(&b[1]) ||
    !a[2].approx_eq(&b[2]) ||
    !a[3].approx_eq(&b[3]) ||
    !a[4].approx_eq(&b[4]) ||
    !a[5].approx_eq(&b[5])
}
#[test]
fn test_ne() {
    assert_eq!(ne(
        &[1f32, 1f32, 1f32, 1f32, 1f32, 1f32],
        &[1f32, 1f32, 1f32, 1f32, 1f32, 1f32]),
        false
    );
    assert_eq!(ne(
        &[0f32, 0f32, 0f32, 0f32, 0f32, 0f32],
        &[1f32, 1f32, 1f32, 1f32, 1f32, 1f32]),
        true
    );
}
