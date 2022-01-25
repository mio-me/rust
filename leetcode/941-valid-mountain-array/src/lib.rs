use std::cmp::Ordering;
struct Solution;
impl Solution {
  pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    let mut inc = true;
    arr.len() > 1
      && arr[0] < arr[1]
      && !arr.windows(2).skip(1).any(|x| match x[0].cmp(&x[1]) {
        Ordering::Equal => true,
        Ordering::Greater => {
          inc = false;
          inc
        }
        Ordering::Less => !inc,
      })
      && !inc
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false);
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::valid_mountain_array(vec![3, 5, 5]), false);
  }

  #[test]
  fn example3() {
    assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
  }

  #[test]
  fn example4() {
    assert_eq!(
      Solution::valid_mountain_array(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
      false
    );
  }
}
