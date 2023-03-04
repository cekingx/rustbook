//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convinient

/// Adds one to the number given.
/// # Examples
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}