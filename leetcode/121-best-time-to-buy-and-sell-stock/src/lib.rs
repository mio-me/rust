pub struct Solution;
impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut min = i32::MAX;

    for price in prices {
      if price > min && profit < price - min {
        profit = price - min;
      }
      if price < min {
        min = price;
      }
    }
    profit
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
  }
}
