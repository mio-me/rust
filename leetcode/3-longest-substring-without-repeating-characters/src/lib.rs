struct Solution;
impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    s.bytes().enumerate().fold(&mut [0; 258], |map, (i, c)| {
      let c = c as usize;
      map[257] = map[257].max(map[c]);
      map[c] = i + 1;
      map[256] = map[256].max(i - map[257] + 1);
      map
    })[256] as i32
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
