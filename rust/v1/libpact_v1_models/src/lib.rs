extern crate rustc_serialize;
#[macro_use] extern crate log;
#[macro_use] extern crate p_macro;
#[macro_use] extern crate maplit;
#[macro_use] extern crate lazy_static;
extern crate regex;

#[macro_export]
macro_rules! s {
    ($e:expr) => ($e.to_string())
}

pub mod model;

use std::iter::FromIterator;

pub fn strip_whitespace<'a, T: FromIterator<&'a str>>(val: &'a String, split_by: &'a str) -> T {
    val.split(split_by).map(|v| v.trim().clone() ).collect()
}

#[cfg(test)]
#[macro_use(expect)] extern crate expectest;

#[cfg(test)] extern crate quickcheck;

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;
    use quickcheck::{TestResult, quickcheck};
}
