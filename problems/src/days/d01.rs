use super::*;

type Base = i32; // type that generally fits the data worked on (sint / uint, width, float, etc.)
pub static PROBLEM: Problem = Problem::new(2025, 1, Some(solver_a), Some(solver_b));

const DIAL_SIZE: Base = 99;

pub fn solver_a(input: &str) -> AocResult {
  let mut pos: Base = 50;
  let mut count: Base = 0;

  for line in input.trim().lines() {
    let (dir, num) = line.split_at(1);
    let num = num.parse::<Base>()?;

    pos = match dir {
      "L" => pos - num,
      "R" => pos + num,
      _ => unreachable!(), // Per problem description
    }
    .rem_euclid(DIAL_SIZE + 1);

    count += (pos == 0) as Base;
  }

  count.into()
}

pub fn solver_b(input: &str) -> AocResult {
  let mut pos: Base = 50;
  let mut count: Base = 0;

  for line in input.trim().lines() {
    let (dir, num) = line.split_at(1);
    let num = num.parse::<Base>()?;

    for _ in 0..num {
      pos = match dir {
        "L" => pos - 1,
        "R" => pos + 1,
        _ => unreachable!(),
      }
      .rem_euclid(DIAL_SIZE + 1);

      count += (pos == 0) as Base;
    }
  }

  count.into()
}

/// make sure we're passing the test cases
#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
  #[test]
  fn test_a() {
    assert_eq!(solver_a(INPUT), "3".into());
  }

  #[test]
  fn test_b() {
    assert_eq!(solver_b(INPUT), "6".into());
  }
}
