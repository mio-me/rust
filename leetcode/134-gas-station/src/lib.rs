struct Solution;
impl Solution {
  pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let (total_gas, _, start) = gas.iter().zip(&cost).map(|(a, b)| a - b).enumerate().fold(
      (0, 0, 0),
      |(total_gas, current_gas, start), (i, next_gas_diff)| {
        if current_gas + next_gas_diff < 0 {
          (total_gas + next_gas_diff, 0, i + 1)
        } else {
          (
            total_gas + next_gas_diff,
            current_gas + next_gas_diff,
            start,
          )
        }
      },
    );
    if total_gas >= 0 {
      start as i32
    } else {
      -1
    }
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
