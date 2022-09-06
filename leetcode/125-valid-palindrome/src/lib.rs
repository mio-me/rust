pub struct Solution;
impl Solution {
  pub fn is_palindrome(s: String) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    let mut l = 0;
    let mut r = chars.len() - 1;

    if l == r {
      return true;
    }

    while l < r {
      if !chars[l].is_alphanumeric() {
        l += 1;
        continue;
      }

      if !chars[r].is_alphanumeric() {
        r -= 1;
        continue;
      }

      if chars[l].to_ascii_lowercase() != chars[r].to_ascii_lowercase() {
        return false;
      }

      l += 1;
      r -= 1;
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn example1() {
    assert_eq!(
      Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()),
      true
    );
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::is_palindrome("race a car".to_owned()), false);
  }

  #[test]
  fn example3() {
    assert_eq!(Solution::is_palindrome(" ".to_owned()), true);
  }
}
