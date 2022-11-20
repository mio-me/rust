use std::cell::UnsafeCell;

pub struct Cell<T> {
  value: UnsafeCell<T>,
}

// impl !Sync for Cell<T> {} is implied by UnsafeCell

impl<T> Cell<T> {
  pub fn new(value: T) -> Self {
    Cell {
      value: UnsafeCell::new(value),
    }
  }

  pub fn set(&self, value: T) {
    unsafe { *self.value.get() = value };
  }

  pub fn get(&self) -> T
  where
    T: Copy,
  {
    unsafe { *self.value.get() }
  }
}

#[cfg(test)]
mod tests {
  use std::{sync::Arc, thread};

  use super::Cell;

  #[test]
  fn bad() {
    let x = Arc::new(Cell::new(42));

    let x1 = Arc::clone(&x);
    thread::spawn(|| {
      x1.set(43);
    });

    let x2 = Arc::clone(&x);
    thread::spawn(|| {
      x2.set(44);
    });
  }
}
