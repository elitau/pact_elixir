extern crate rustc_serialize;
#[macro_use] extern crate log;
#[macro_use] extern crate p_macro;

pub mod model;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;
    use quickcheck::{TestResult, quickcheck};
}
