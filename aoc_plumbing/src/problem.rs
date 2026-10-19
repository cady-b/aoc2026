//! Datastructures capturing an Advent of Code problem

use std::{num::NonZeroU8, time::Duration};

use crate::{cache::Cache, pretty_duration};

pub type Day = std::num::NonZeroU8;
pub type Year = i64;

/// signature of part-solvers
pub type Solver = fn(&str) -> AocResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AocProblemPart {
  A,
  B,
}

impl std::fmt::Display for AocProblemPart {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      AocProblemPart::A => "A",
      AocProblemPart::B => "B",
    })
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AocDay {
  pub day:  Day,
  pub year: Year,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AocProblemId {
  pub day_id: AocDay,
  pub part:   AocProblemPart,
}

#[derive(Debug)]
pub enum AocResult {
  Ok(String),
  Err(Box<dyn std::error::Error>),
}

impl<T: core::fmt::Display> From<T> for AocResult {
  fn from(value: T) -> Self { Self::Ok(value.to_string()) }
}

impl<E> std::ops::FromResidual<Result<!, E>> for AocResult
where Box<dyn std::error::Error>: From<E>
{
  fn from_residual(residual: Result<!, E>) -> Self {
    match residual {
      Err(e) => Self::Err(From::from(e)),
    }
  }
}

// For Debugging
impl PartialEq for AocResult {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Ok(l0), Self::Ok(r0)) => l0 == r0,
      (Self::Err(l0), Self::Err(r0)) => l0.to_string() == r0.to_string(),
      _ => false,
    }
  }
}

/// structure holding everything related to one aoc day
#[derive(Debug)]
pub struct Problem {
  day:     AocDay,
  solvers: (Option<Solver>, Option<Solver>),
}

impl Problem {
  /// [panics](panic) if `day` is zero to avoid having to leak [NonZeroU8] everywhere outside
  pub const fn new(year: i64, day: u8, solver_a: Option<Solver>, solver_b: Option<Solver>) -> Self {
    let day = NonZeroU8::new(day).expect("day has to be non-zero");

    Self {
      day:     AocDay { year, day },
      solvers: (solver_a, solver_b),
    }
  }

  pub const fn get_day(&self) -> AocDay { self.day }

  pub fn run(&self, cache: &Cache) -> Option<Duration> {
    let entry = match cache.entries.get(&self.day).ok_or_else(|| {}) {
      Ok(e) => e,
      Err(_) => {
        eprintln!("Cannot find input for day: {}/{:02}", self.day.year, self.day.day);
        return None;
      }
    };

    let a = self.solvers.0.map(|s| (s, AocProblemPart::A));
    let b = self.solvers.1.map(|s| (s, AocProblemPart::B));

    let mut total_runtime = Duration::ZERO;

    let res = [a, b].into_iter().flatten().map(|(solver, part)| {
      let start_time = std::time::Instant::now();
      let result = solver(&entry.input);
      let duration = start_time.elapsed();

      total_runtime += duration;

      match result {
        AocResult::Ok(res) => {
          let answer = match part {
            AocProblemPart::A => &entry.answers.0,
            AocProblemPart::B => &entry.answers.1,
          };

          let indicator = match answer {
            Some(a) if a == &res => "✔",
            Some(_) => "✘",
            None => "？",
          };

          format!("{part} = {res} {indicator} (in {})", pretty_duration(duration))
        }
        AocResult::Err(e) => format!("Error in part {part}: {e}"),
      }
    });

    println!("╭──┄┈ ★ ☃  ★\n│ {}/{:02}: {}", self.day.year, self.day.day, entry.title);
    for result in res {
      println!("│ {result}");
    }
    println!("{}", "╰──┄┈");

    Some(total_runtime)
  }
}
