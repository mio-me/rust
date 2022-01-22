struct Solution;
impl Solution {
  pub fn winner_square_game(n: i32) -> bool {
    let total_stones = n as usize;
    (0..=total_stones).fold(&mut vec![false; total_stones + 1], |v, path| {
      if !v[path] {
        (1..)
          .map(|sqrt| path + sqrt * sqrt)
          .take_while(|&remaining_stones| remaining_stones <= total_stones)
          .for_each(|winning_turn| v[winning_turn] = true);
      }
      v
    })[total_stones]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(Solution::winner_square_game(1), true);
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::winner_square_game(2), false);
  }

  #[test]
  fn example3() {
    assert_eq!(Solution::winner_square_game(4), true);
  }
}
