struct Solution;
impl Solution {
  pub fn max_product(words: Vec<String>) -> i32 {
    let mut max = 0;
    for i in 0..words.len() {
      for j in 1..words.len() {
        if i == j {
          continue;
        }

        if words[i].chars().all(|c| !words[j].contains(c)) {
          let len = words[i].len() * words[j].len();
          if len > max {
            max = len;
          }
        }
      }
    }
    max as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    let data = vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]
      .iter()
      .map(|s| s.to_string())
      .collect();
    assert_eq!(Solution::max_product(data), 16);
  }

  #[test]
  fn example2() {
    let data = vec!["a", "ab", "abc", "d", "cd", "bcd", "abcd"]
      .iter()
      .map(|s| s.to_string())
      .collect();
    assert_eq!(Solution::max_product(data), 4);
  }

  #[test]
  fn example3() {
    let data = vec!["a", "aa", "aaa", "aaaa"]
      .iter()
      .map(|s| s.to_string())
      .collect();
    assert_eq!(Solution::max_product(data), 0);
  }
}
