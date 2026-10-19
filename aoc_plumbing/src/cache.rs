use std::{collections::HashMap, path::Path};

use itertools::Itertools;

use crate::problem::AocDay;

#[derive(Debug, Clone)]
pub struct Entry {
  pub title:   String,
  pub input:   String,
  pub answers: (Option<String>, Option<String>),
}

#[derive(Debug, Clone)]
pub struct Cache {
  pub entries: HashMap<AocDay, Entry>,
}

impl Cache {
  pub fn read(dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    let mut entries = HashMap::new();

    for entry in dir.read_dir()?.filter_map(Result::ok) {
      match entry.file_type() {
        Ok(e) if e.is_file() => {
          let s = std::fs::read_to_string(entry.path())?;

          let (title, s) = s
            .split_once('\n')
            .ok_or_else(|| format!("Empty cache file: {}", entry.path().display()))?;

          let (answers, input) = s
            .split_once('\n')
            .ok_or_else(|| format!("Cache file does not contain an input: {}", entry.path().display()))?;

          let [year, day, title] = title
            .splitn(3, ':')
            .next_array()
            .ok_or_else(|| format!("Unable to parse title for: {}", entry.path().display()))?;

          let answers = answers
            .split_once(',')
            .map(|(a, b)| {
              let a = (!a.is_empty()).then(|| a.to_owned());
              let b = (!b.is_empty()).then(|| b.to_owned());

              (a, b)
            })
            .unwrap_or((None, None));

          entries.insert(
            AocDay {
              year: year.parse()?,
              day:  day.parse()?,
            },
            Entry {
              title: title.to_owned(),
              input: input.to_owned(),
              answers,
            },
          );
        }
        _ => {}
      }
    }

    Ok(Self { entries })
  }
}
