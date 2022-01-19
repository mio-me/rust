use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut map = HashMap::with_capacity(nums.len());

  for (i, n) in nums.iter().enumerate() {
    match map.get(n) {
      Some(j) => return vec![*j as i32, i as i32],
      None => {
        map.insert(target - *n, i);
      }
    }
  }
  unreachable!();
}

#[cfg(test)]
mod tests {
  #[test]
  fn example1() {
    assert_eq!(super::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  }

  #[test]
  fn example2() {
    assert_eq!(super::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
  }

  #[test]
  fn example3() {
    assert_eq!(super::two_sum(vec![3, 3], 6), vec![0, 1]);
  }
}
