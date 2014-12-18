#![deny(missing_docs)]
#![deny(warnings)]

//! A simple general coercion trait.

/// General conversion between two types.
pub trait From<I> {
    /// Convert I to Self.
    fn from(I) -> Self;
}

impl<T> From<T> for T {
    fn from(x: T) -> T { x }
}

impl<'a, T: Clone> From<&'a T> for T {
    fn from(x: &'a T) -> T { x.clone() }
}

impl<'a, T> From<&'a Vec<T>> for &'a [T] {
    fn from(x: &'a Vec<T>) -> &'a [T] { x.as_slice() }
}

impl<'a> From<&'a String> for &'a str {
    fn from(x: &'a String) -> &'a str { x.as_slice() }
}

impl<'a> From<&'a str> for String {
    fn from(x: &'a str) -> String { x.into_string() }
}

impl<'a, T: Clone> From<&'a [T]> for Vec<T> {
    fn from(x: &'a [T]) -> Vec<T> { x.to_vec() }
}

