#[macro_use]
#[cfg(test)]
extern crate quickcheck;

mod problems;
mod utils;

use anyhow::{Context, Result};
pub use problems::p201::problem201;
pub use problems::p202::problem202;
pub use problems::p208::problem208;
pub use problems::p209::problem209;
pub use problems::p60::problem60;
use std::env;
use thiserror::Error;
pub(crate) use utils::sieve::*;

#[derive(Debug)]
pub struct InputArgs {
    pub problem_num: usize,
    pub input_parameter: usize,
}

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("Missing arguments")]
    MissingArgument,

    #[error("Problem not implemented")]
    NotImplemented,

    #[error("Input doesn't satisfy assumed guarantees or is malformed")]
    BadInput,

    #[error("Needs larger upper bound to terminate successfully")]
    InitialConditionTooSmall,

    #[error("Oversized integers encountered with bounded Int types")]
    LargeInt,

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub fn read_args() -> Result<InputArgs> {
    let problem = env::args().nth(1);
    let parameter = env::args().nth(2);
    match (problem, parameter) {
        (Some(pr), Some(pa)) => Ok(InputArgs {
            problem_num: pr.parse().context("Problem parsing problem number")?,
            input_parameter: pa.parse().context("Problem parsing parameter")?,
        }),
        _ => Err(ProgramError::MissingArgument.into()),
    }
}
