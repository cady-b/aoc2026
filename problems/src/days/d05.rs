use super::*;

type Base = u64; // Type that generally fits the data worked on (sint / uint, width, float, etc.)
pub static PROBLEM: Problem = Problem::new(2025, 5, Some(solver_a), Some(solver_b));

pub fn solver_a(input: &str) -> AocResult {
  let (ids, ranges) = prepare_input(input).expect("Puzzle input is malformed");
  ids.iter().filter(|id| ranges.iter().any(|r| r.contains(id))).count().into()
}

pub fn solver_b(input: &str) -> AocResult {
  let (_, mut ranges) = prepare_input(input).expect("Puzzle input is malformed");
  ranges.sort_by_key(|r| r.start); // Let's look at them in order, shall we?

  let mut count = 0;
  let (mut current_start, mut current_end) = (ranges[0].start, ranges[0].last);

  for r in ranges {
    // If this range's start exceeds the current tracked range, add that to the count and set a new start.
    if r.start > current_end {
      count += (current_start..=current_end).count();
      current_start = r.start;
    }

    // Find the max end.
    current_end = current_end.max(r.last);
  }

  // Finalize from current state.
  (count + (current_start..=current_end).count()).into()
}

use std::range::RangeInclusive;
fn prepare_input(input: &str) -> Result<(Vec<Base>, Vec<RangeInclusive<Base>>), Box<dyn std::error::Error>> {
  let (ranges, ids) = input.trim().split_once("\n\n").ok_or("No empty line found")?;
  let ids: Vec<_> = ids.lines().map(str::parse).collect::<Result<_, _>>()?;
  let ranges: Vec<_> = ranges
    .lines()
    .map(|v| {
      let (l, r) = v.split_once('-').ok_or("Unable to parse range")?;
      Ok::<_, Box<dyn std::error::Error>>(RangeInclusive {
        start: l.parse()?,
        last:  r.parse()?,
      })
    })
    .collect::<Result<_, _>>()?;

  Ok((ids, ranges))
}

/// Make sure we're passing the test cases.
#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

  #[test]
  fn test_a() {
    assert_eq!(solver_a(INPUT), "3".into());
  }

  #[test]
  fn test_b() {
    assert_eq!(solver_b(INPUT), "14".into());
  }
}
