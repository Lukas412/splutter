#![allow(unused)]

extern crate core;

pub use {
    brackets::{SurroundWithAngleBrackets, SurroundWithBrackets, SurroundWithCurlyBrackets},
    output::{Output, Separated},
    quoted::{DoubleQuotedStr, SingleQuotedStr},
    validators::{StrRefIdentifier, StrRefInteger, StrValidationExt},
};

mod brackets;
mod output;
mod quoted;
mod validators;
