use std::fmt::Debug;

use float_cmp::ApproxEq;
use pretty_assertions::assert_eq;
use pretty_assertions::assert_ne;

pub fn nia_assert_equal<T>(v1: T, v2: T)
    where T: PartialEq + Eq + Debug
{
    assert_eq!(v1, v2);
}

pub fn nia_assert_nequal<T>(v1: T, v2: T)
    where T: PartialEq + Eq + Debug
{
    assert_ne!(v1, v2);
}

pub fn nia_assert_fequal(v1: f64, v2: f64)
{
    assert!(v1.approx_eq(v2, (0.0, 2)));
}

pub fn nia_assert_fnequal(v1: f64, v2: f64)
{
    assert!(!v1.approx_eq(v2, (0.0, 2)));
}

pub fn nia_assert(result: bool) {
    assert!(result);
}

pub fn nia_assert_is_ok<V, E>(result: &Result<V, E>) {
    assert!(result.is_ok())
}

pub fn nia_assert_is_err<V, E>(result: &Result<V, E>) {
    assert!(result.is_err())
}

pub fn nia_assert_is_true(value: bool) {
    assert_eq!(true, value);
}

pub fn nia_assert_is_false(value: bool) {
    assert_eq!(false, value);
}
