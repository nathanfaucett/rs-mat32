use num::Num;


#[inline]
pub fn sdiv<'a, 'b, T: Copy + Num>(out: &'a mut [T; 6], a: &'b [T; 6], s: T) ->  &'a mut [T; 6] {
    let not_zero = s != T::zero();
    out[0] = if not_zero {a[0] / s} else  {T::zero()};
    out[1] = if not_zero {a[1] / s} else  {T::zero()};
    out[2] = if not_zero {a[2] / s} else  {T::zero()};
    out[3] = if not_zero {a[3] / s} else  {T::zero()};
    out[4] = if not_zero {a[4] / s} else  {T::zero()};
    out[5] = if not_zero {a[5] / s} else  {T::zero()};
    out
}
#[test]
fn test_sdiv() {
    let mut v = [0, 0, 0, 0, 0, 0];
    sdiv(&mut v, &[1, 1, 1, 1, 1, 1], 1);
    assert!(v == [1, 1, 1, 1, 1, 1]);
}
