use num::Num;


#[inline(always)]
pub fn mul<'a, 'b, T: Num>(out: &'a mut [T; 6], a: &'b [T; 6], b: &'b [T; 6]) ->  &'a mut [T; 6] {
    let a11 = a[0];
    let a12 = a[2];
    let a13 = a[4];
    let a21 = a[1];
    let a22 = a[3];
    let a23 = a[5];

    let b11 = b[0];
    let b12 = b[2];
    let b13 = b[4];
    let b21 = b[1];
    let b22 = b[3];
    let b23 = b[5];

    out[0] = a11 * b11 + a21 * b12;
    out[2] = a12 * b11 + a22 * b12;

    out[1] = a11 * b21 + a21 * b22;
    out[3] = a12 * b21 + a22 * b22;

    out[4] = a11 * b13 + a12 * b23 + a13;
    out[5] = a21 * b13 + a22 * b23 + a23;
    out
}
#[test]
fn test_mul() {
    let mut v = [1, 0, 0, 1, 0, 0];
    mul(&mut v, &[1, 0, 0, 1, 0, 0], &[1, 0, 0, 1, 0, 0]);
    assert!(v == [1, 0, 0, 1, 0, 0]);
}

#[inline(always)]
pub fn smul<'a, 'b, T: Num>(out: &'a mut [T; 6], a: &'b [T; 6], s: T) ->  &'a mut [T; 6] {
    out[0] = a[0] * s;
    out[1] = a[1] * s;
    out[2] = a[2] * s;
    out[3] = a[3] * s;
    out[4] = a[4] * s;
    out[5] = a[5] * s;
    out
}
#[test]
fn test_smul() {
    let mut v = [0, 0, 0, 0, 0, 0];
    smul(&mut v, &[1, 0, 0, 1, 0, 0], 1);
    assert!(v == [1, 0, 0, 1, 0, 0]);
}
