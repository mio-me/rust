struct TwoArr {
  a1: Vec<i32>,
  a2: Vec<i32>,
}

impl Iterator for TwoArr {
  type Item = i32;
  fn next(&mut self) -> Option<Self::Item> {
    match (self.a1.last(), self.a2.last()) {
      (Some(a), Some(b)) if a >= b => self.a1.pop(),
      (Some(_), Some(_)) => self.a2.pop(),
      (Some(_), None) => self.a1.pop(),
      (None, Some(_)) => self.a2.pop(),
      _ => None,
    }
  }
}

impl TwoArr {
  fn len(&self) -> usize {
    self.a1.len() + self.a2.len()
  }
}

struct Solution;
impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut both = TwoArr {
      a1: nums1,
      a2: nums2,
    };
    let half = both.len() / 2;

    if both.len() % 2 == 0 {
      both.skip(half - 1).take(2).sum::<i32>() as f64 / 2.
    } else {
      both.nth(half).unwrap() as f64
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn example1() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.);
  }

  #[test]
  fn example2() {
    assert_eq!(
      Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
      2.5
    );
  }
}
