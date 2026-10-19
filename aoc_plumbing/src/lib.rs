#![feature(try_trait_v2)]

pub mod cache;
pub mod problem;

pub fn pretty_duration(duration: std::time::Duration) -> String {
  if duration.as_secs() > 0 {
    return format!("{:.2}s", duration.as_secs_f32());
  }

  if duration.as_millis() > 0 {
    return format!("{:.2}ms", duration.as_micros() as f32 / 1000.);
  }

  format!("{}μs", duration.as_micros())
}
