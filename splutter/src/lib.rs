#![feature(pattern)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(unused)]

extern crate core;

pub use {output::*, validators::*};

mod output;
mod validators;
