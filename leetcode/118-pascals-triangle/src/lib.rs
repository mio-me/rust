struct Solution;
impl Solution {
  pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut cur = vec![vec![1i32]];

    for row in 1..num_rows {
      let mut new_row: Vec<i32> = cur
        .last()
        .unwrap()
        .windows(2)
        .map(|x| x[0] + x[1])
        .collect();
      cur.push([&[1], new_row.as_slice(), &[1]].concat());
    }

    cur
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn example1() {
    assert_eq!(
      Solution::generate(5),
      vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1]
      ]
    );
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::generate(1), vec![vec![1],]);
  }
}
