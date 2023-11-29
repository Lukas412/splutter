#![allow(unused)]

extern crate core;

pub use {
    output::{Output, Separated},
    validators::{StrRefIdentifier, StrRefInteger, StrValidationExt},
};

mod output;
mod validators;
