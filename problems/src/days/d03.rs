use super::*;

type Base = u64; // type that generally fits the data worked on (sint / uint, width, float, etc.)
pub static PROBLEM: Problem = Problem::new(2025, 3, Some(solver_a), Some(solver_b));

pub fn solver_a(input: &str) -> AocResult {
  prepare_input(input)
    .iter()
    .map(|bank| {
      (1..bank.len())
        .into_iter()
        .map(|v| {
          let base = bank[v - 1];
          let max = bank[v..].iter().max().cloned().unwrap_or_default();

          base * 10 + max
        })
        .max()
        .unwrap_or_default()
    })
    .sum::<Base>()
    .into()
}

pub fn solver_b(input: &str) -> AocResult {
  const NUM_BATTERIES: usize = 12;

  prepare_input(input)
    .into_iter()
    .map(|mut bank| {
      // Iteratively make the highest-value bank
      for _ in 0..(bank.len() - NUM_BATTERIES) {
        let _ = bank.remove(
          (0..bank.len())
            .map(|s| {
              // Faster than iterating
              let mut filtered_bank = bank.clone();
              let _ = filtered_bank.remove(s);

              (s, filtered_bank)
            })
            .max_by(|(_, j1), (_, j2)| j1.cmp(j2))
            .map(|(s, _)| s)
            .unwrap_or_default(),
        );
      }

      // Also doing this above overflows, but here we gotta get the actual value
      bank.iter().fold(0, |acc, &v| acc * 10 + v)
    })
    .sum::<Base>()
    .into()
}

fn prepare_input(input: &str) -> Vec<Vec<Base>> {
  input
    .trim()
    .lines()
    .map(|l| l.bytes().map(|v| (v - b'0') as Base).collect())
    .collect()
}

/// make sure we're passing the test cases
#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

  #[test]
  fn test_a() {
    assert_eq!(solver_a(INPUT), "357".into());
  }

  #[test]
  fn test_b() {
    assert_eq!(solver_b(INPUT), "3121910778619".into());
  }
}
