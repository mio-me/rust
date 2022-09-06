pub struct Solution;
impl Solution {
  pub fn is_anagram(s: String, t: String) -> bool {
    let mut sv = s.chars().collect::<Vec<_>>();
    sv.sort();

    let mut tv = t.chars().collect::<Vec<_>>();
    tv.sort();
    tv == sv
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn example1() {
    assert_eq!(
      Solution::is_anagram("anagram".to_owned(), "nagaram".to_owned()),
      true
    );
  }

  #[test]
  fn example2() {
    assert_eq!(
      Solution::is_anagram("car".to_owned(), "rat".to_owned()),
      false
    );
  }

  #[test]
  fn example3() {
    assert_eq!(
      Solution::is_anagram("wwe".to_owned(), "wew".to_owned()),
      true
    );
  }
}
