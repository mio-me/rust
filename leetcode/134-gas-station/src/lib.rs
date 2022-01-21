struct Solution;
impl Solution {
  pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let steps = gas
      .iter()
      .zip(&cost)
      .map(|(g, c)| g - c)
      .collect::<Vec<_>>();

    if steps.len() == 1 {
      if steps[0] >= 0 {
        return 0;
      } else {
        return -1;
      }
    }

    for i in 0..steps.len() {
      if steps[i] < 1 {
        continue;
      };

      let mut current = 0;
      for j in (i..steps.len()).chain(0..i) {
        current += steps[j];

        if (j as i32 == i as i32 - 1i32 || j == steps.len() - 1 && i == 0) && current >= 0 {
          return i as i32;
        }
        if current < 1 {
          break;
        }
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
    assert_eq!(
      Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
      3
    );
  }

  #[test]
  fn example2() {
    assert_eq!(
      Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
      -1
    );
  }

  #[test]
  fn example3() {
    assert_eq!(
      Solution::can_complete_circuit(vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1]),
      4
    );
  }

  #[test]
  fn example4() {
    assert_eq!(Solution::can_complete_circuit(vec![2], vec![2]), 0);
  }
}
