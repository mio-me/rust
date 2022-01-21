struct Solution;
impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest = String::from("");
    let mut current = String::from("");

    for c in s.chars() {
      if current.contains(c) {
        if current.len() > longest.len() {
          longest = current.clone();
        }
        current = current.split(c).skip(1).collect::<String>();
      }
      current.push(c);
    }
    std::cmp::max(longest.len(), current.len()) as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(
      Solution::length_of_longest_substring(String::from("abcabcbb")),
      3
    );
  }

  #[test]
  fn example2() {
    assert_eq!(
      Solution::length_of_longest_substring(String::from("bbbbb")),
      1
    );
  }

  #[test]
  fn example3() {
    assert_eq!(
      Solution::length_of_longest_substring(String::from("pwwkew")),
      3
    );
  }

  #[test]
  fn example4() {
    assert_eq!(
      Solution::length_of_longest_substring(String::from("aab")),
      2
    );
  }
}
