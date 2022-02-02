#![feature(is_sorted)]

#[cfg(test)]
mod tests;

pub mod algorithms;

pub mod prelude {
    pub use crate::algorithms::*;
}

