use itertools::Itertools;

use super::*;

type Base = u64; // type that generally fits the data worked on (sint / uint, width, float, etc.)
pub static PROBLEM: Problem = Problem::new(2025, 2, Some(solver_a), Some(solver_b));

pub fn solver_a(input: &str) -> AocResult {
  let mut sum: u64 = 0;

  for range in prepare_input(input)? {
    for i in range {
      let s = i.to_string().into_bytes();

      let len = s.len();
      if len % 2 == 0 && s.chunks_exact(len / 2).all_equal() {
        sum += i;
      }
    }
  }

  sum.into()
}

pub fn solver_b(input: &str) -> AocResult {
  let ids = prepare_input(input)?
    .into_iter()
    .flat_map(core::iter::IntoIterator::into_iter)
    .dedup() // Just to be sure (doesn't make a
    // difference with my input)
    .filter(|v| {
      let digits = v.to_string().into_bytes();

      let len = digits.len();
      (1..=(len / 2)).any(|size| len % size == 0 && digits.chunks_exact(size).all_equal())
    });

  ids.sum::<u64>().into()
}

fn prepare_input(input: &str) -> Result<Vec<std::ops::RangeInclusive<Base>>, Box<dyn std::error::Error>> {
  input
    .trim()
    .split(',')
    .map(|v| {
      let (a, b) = v.split_once('-').ok_or("Unable to parse input")?;
      let a = a.parse()?;
      let b = b.parse()?;
      Ok(std::ops::RangeInclusive::new(a, b))
    })
    .collect()
}

/// make sure we're passing the test cases
#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

  #[test]
  fn test_a() {
    assert_eq!(solver_a(INPUT), "1227775554".into());
  }

  #[test]
  fn test_b() {
    assert_eq!(solver_b(INPUT), "4174379265".into());
  }
}
