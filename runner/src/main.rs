mod args;

use std::{path::Path, time::Duration};

use args::Args;
use clap::Parser;
use rayon::prelude::*;

fn main() {
  let args = Args::parse();

  let days: Vec<_> = if args.days.is_empty() {
    problems::DAYS.into()
  } else {
    problems::DAYS
      .iter()
      .copied()
      .filter(|p| args.days.contains(&p.get_day().day.get()))
      .collect()
  };

  if !days.is_empty() {
    let cache = aoc_plumbing::cache::Cache::read(Path::new("cache/")).unwrap();

    let total = if args.parallel {
      days
        .par_iter()
        .filter_map(|v| match v.run(&cache) {
          Ok(v) => Some(v),
          Err(e) => {
            eprintln!("{e}");
            None
          }
        })
        .reduce(|| Duration::ZERO, |a, b| a + b)
    } else {
      let mut total = Duration::ZERO;

      for day in days {
        match day.run(&cache) {
          Ok(v) => total += v,
          Err(e) => eprintln!("{e}"),
        }
      }

      total
    };

    println!("\nTotal runtime: {}", aoc_plumbing::pretty_duration(total));
  }

  println!("Merry Advent!");
}
