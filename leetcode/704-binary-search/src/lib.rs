use std::cmp::Ordering;

struct Solution;

impl Solution {
  pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r) = (0, nums.len());

    while l < r {
      let mid = l + (r - l) / 2;

      match nums[mid].cmp(&target) {
        Ordering::Less => l = mid + 1,
        Ordering::Greater => r = mid,
        Ordering::Equal => return mid as i32,
      }
    }
    -1
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn example1() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
  }

  #[test]
  fn example3() {
    assert_eq!(Solution::search(vec![5], 5), 0);
  }
}
