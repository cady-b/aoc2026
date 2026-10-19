use std::{path::Path, time::Duration};

fn main() {
  let cache = aoc_plumbing::cache::Cache::read(Path::new("cache/")).unwrap();

  let mut total_runtime = Duration::ZERO;

  for problem in problems::DAYS {
    if let Some(runtime) = problem.run(&cache) {
      total_runtime += runtime;
    }
  }

  println!("\nTotal runtime: {}", aoc_plumbing::pretty_duration(total_runtime));
}
