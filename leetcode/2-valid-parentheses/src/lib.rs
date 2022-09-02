pub struct Solution;

impl Solution {
  pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for c in s.chars() {
      match c {
        '(' | '[' | '{' => stack.push(c),
        _ => match stack.pop() {
          Some('(') if c == ')' => {}
          Some('[') if c == ']' => {}
          Some('{') if c == '}' => {}
          _ => {
            return false;
          }
        },
      }
    }
    stack.is_empty()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
  }

  #[test]
  fn example3() {
    assert_eq!(Solution::is_valid("(]".to_string()), false);
  }
}
