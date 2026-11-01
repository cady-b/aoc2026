#![allow(unused)]

/// 2D Matrix with helpers
pub struct Texture<T> {
  data:  Vec<T>,
  width: usize,
}

impl<T> Texture<T> {
  pub fn new(data: Vec<T>, width: usize) -> Self { Self { data, width } }

  pub fn get(&self, x: usize, y: usize) -> Option<&T> { (x < self.width).then(|| self.data.get(y * self.width + x)).flatten() }

  pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
    (x < self.width).then(|| self.data.get_mut(y * self.width + x)).flatten()
  }

  /// Permit OOB *inputs* (returns None).
  pub fn get_signed(&self, x: isize, y: isize) -> Option<&T> { (x >= 0 && y >= 0).then(|| self.get(x as usize, y as usize)).flatten() }

  pub fn convolve_iter<'a, O: 'a>(&'a self, kernel: fn(x: usize, y: usize, cell: &T, s: &Self) -> O) -> impl Iterator<Item = O> {
    self.data.iter().enumerate().map(move |(n, v)| {
      let y = n / self.width;
      let x = n % self.width;

      kernel(x, y, v, self)
    })
  }
}
