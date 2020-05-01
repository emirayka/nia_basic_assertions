use pretty_assertions::assert_eq;
use pretty_assertions::assert_ne;

#[macro_export]
macro_rules! nia_assert {
    ($what:expr) => {
        assert!($what);
    }
}

#[macro_export]
macro_rules! nia_assert_equal {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right);
    }
}

#[macro_export]
macro_rules! nia_assert_nequal {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right);
    }
}

#[macro_export]
macro_rules! nia_assert_is_ok {
    ($what:expr) => {
        assert!($what.is_ok());
    }
}

#[macro_export]
macro_rules! nia_assert_is_err {
    ($what:expr) => {
        assert!($what.is_err());
    }
}
