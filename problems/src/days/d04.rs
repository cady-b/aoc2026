use super::*;

pub static PROBLEM: Problem = Problem::new(2025, 4, Some(solver_a), Some(solver_b));

const MAX_ROLLS: usize = 4;

pub fn solver_a(input: &str) -> AocResult {
  let grid = prepare_input(input);

  grid
    .convolve_iter(|x, y, &cell, g| -> bool {
      if !cell {
        return false;
      }

      (-1..=1)
        .flat_map(|ny| (-1..=1).map(move |nx| g.get_signed(nx + x as isize, ny + y as isize)))
        .filter(|&v| v == Some(&true))
        .count()
        < MAX_ROLLS + 1 // also finds self again; this is probably the cheapest way to deal with that
    })
    .filter(|&v| v)
    .count()
    .into()
}

pub fn solver_b(input: &str) -> AocResult {
  let mut grid = prepare_input(input);
  let mut count: usize = 0;

  loop {
    let n = grid
      .convolve_iter(|x, y, &cell, g| -> Option<(usize, usize)> {
        if !cell {
          return None;
        }

        let accessible = (-1..=1)
          .flat_map(|ny| (-1..=1).map(move |nx| g.get_signed(nx + x as isize, ny + y as isize)))
          .filter(|&v| v == Some(&true))
          .count()
          < MAX_ROLLS + 1;

        accessible.then_some((x, y))
      })
      .flatten()
      .collect::<Vec<_>>();

    // No more rolls can be removed; we're done here.
    if n.is_empty() {
      break;
    }

    // Count and remove the accessible rolls.
    for (x, y) in n {
      count += 1;
      *grid.get_mut(x, y).unwrap() = false;
    }
  }

  count.into()
}

fn prepare_input(input: &str) -> Texture<bool> {
  let data = input
    .trim()
    .lines()
    .flat_map(|v| {
      v.bytes().map(|b| match b {
        b'.' => false,
        b'@' => true,
        _ => unreachable!(),
      })
    })
    .collect();

  Texture::new(data, input.lines().next().map(|v| v.len()).expect("Couldn't get grid width"))
}

/// Make sure we're passing the test cases.
#[cfg(test)]
mod test {
  use super::*;

  const INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

  #[test]
  fn test_a() {
    assert_eq!(solver_a(INPUT), "13".into());
  }

  #[test]
  fn test_b() {
    assert_eq!(solver_b(INPUT), "43".into());
  }
}
