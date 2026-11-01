use aoc_plumbing::problem::{AocResult, Problem};

use crate::utils::*;

/// convenient access to every solution to be run
pub static DAYS: &[&Problem] = &[
  &d01::PROBLEM,
  &d02::PROBLEM,
  &d03::PROBLEM,
  &d04::PROBLEM,
  //&d05::PROBLEM,
];

mod d01;
mod d02;
mod d03;
mod d04;
