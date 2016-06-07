extern crate vec2;


use num::Num;


#[inline(always)]
pub fn compose<T: Num>(out: &mut [T; 6], position: [T; 2], scale: [T; 2], rotation: T) -> &mut [T; 6] {
    let sx = scale[0];
    let sy = scale[1];
    let c = rotation.cos();
    let s = rotation.sin();

    out[0] = c * sx;
    out[1] = s * sx;
    out[2] = -s * sy;
    out[3] = c * sy;
    out[4] = position[0];
    out[5] = position[1];
    out
}

#[inline(always)]
pub fn decompose<T: Num>(out: [T; 6], position: &mut [T; 2], scale: &mut [T; 2]) -> T {
    let m11 = out[0];
    let m12 = out[1];
    let sx = vec2::length_values(m11, m12);
    let sy = vec2::length_values(out[2], out[3]);

    position[0] = out[4];
    position[1] = out[5];

    scale[0] = sx;
    scale[1] = sy;

    m12.atan2(m11)
}

#[inline(always)]
pub fn look_at<T: Num>(out: &mut [T; 6], eye: [T; 2], target: [T; 2]) -> &mut [T; 6] {
    let x = target[0] - eye[0];
    let y = target[1] - eye[1];
    let a = y.atan2(x) - T::half_pi();
    let c = a.cos();
    let s = a.sin();

    out[0] = c;
    out[1] = s;
    out[2] = -s;
    out[3] = c;
    out
}

#[inline(always)]
pub fn set_rotation<T: Num>(out: &mut [T; 6], rotation: T) -> &mut [T; 6] {
    let c = rotation.cos();
    let s = rotation.sin();

    out[0] = c;
    out[1] = s;
    out[2] = -s;
    out[3] = c;
    out
}

#[inline(always)]
pub fn get_rotation<T: Num>(out: [T; 6]) -> T {
    out[1].atan2(out[0])
}

#[inline(always)]
pub fn set_position<T: Num>(out: &mut [T; 6], v: [T; 2]) -> &mut [T; 6] {
    out[4] = v[0];
    out[5] = v[1];
    out
}

#[inline(always)]
pub fn get_position<T: Num>(out: [T; 6], v: &mut [T; 2]) -> &mut [T; 2] {
    v[0] = out[4];
    v[1] = out[5];
    v
}

#[inline(always)]
pub fn extract_position<T: Num>(out: &mut [T; 6], a: [T; 6]) -> &mut [T; 6] {
    out[4] = a[4];
    out[5] = a[5];
    out
}

#[inline(always)]
pub fn extract_rotation<T: Num>(out: &mut [T; 6], a: [T; 6]) -> &mut [T; 6] {
    let m11 = a[0];
    let m12 = a[2];
    let m21 = a[1];
    let m22 = a[3];

    let x = m11 * m11 + m21 * m21;
    let y = m12 * m12 + m22 * m22;

    let sx = if x != T::zero() {T::one() / x.sqrt()} else {T::zero()};
    let sy = if y != T::zero() {T::one() / y.sqrt()} else {T::zero()};

    out[0] = m11 * sx;
    out[1] = m21 * sx;
    out[2] = m12 * sy;
    out[3] = m22 * sy;
    out
}

#[inline(always)]
pub fn translate<T: Num>(out: &mut [T; 6], a: [T; 6], v: [T; 2]) -> &mut [T; 6] {
    out[4] = a[0] * v[0] + a[2] * v[1] + a[4];
    out[5] = a[1] * v[0] + a[3] * v[1] + a[5];
    out
}

#[inline(always)]
pub fn rotate<T: Num>(out: &mut [T; 6], a: [T; 6], rotation: T) -> &mut [T; 6] {
    let m11 = a[0];
    let m12 = a[2];
    let m21 = a[1];
    let m22 = a[3];
    let c = rotation.cos();
    let s = rotation.sin();

    out[0] = m11 * c + m12 * s;
    out[1] = m11 * -s + m12 * c;
    out[2] = m21 * c + m22 * s;
    out[3] = m21 * -s + m22 * c;
    out
}

#[inline(always)]
pub fn scale<T: Num>(out: &mut [T; 6], a: [T; 6], v: [T; 2]) -> &mut [T; 6] {
    let x = v[0];
    let y = v[1];

    out[0] = a[0] * x;
    out[1] = a[1] * x;
    out[4] = a[4] * x;

    out[2] = a[2] * y;
    out[3] = a[3] * y;
    out[5] = a[5] * y;
    out
}

#[inline(always)]
pub fn orthographic<T: Num>(out: &mut [T; 6], left: T, right: T, top: T, bottom: T) -> &mut [T; 6] {
    let w = right - left;
    let h = top - bottom;

    let x = (right + left) / w;
    let y = (top + bottom) / h;

    out[0] = T::from_isize(2isize) / w;
    out[1] = T::zero();
    out[2] = T::zero();
    out[3] = T::from_isize(2isize) / h;
    out[4] = -x;
    out[5] = -y;
    out
}
