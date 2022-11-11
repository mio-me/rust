#[derive(PartialEq, Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
  remainder: Option<&'haystack str>,
  delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
  pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
    Self {
      remainder: Some(haystack),
      delimiter,
    }
  }
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
  type Item = &'haystack str;
  fn next(&mut self) -> Option<Self::Item> {
    let remainder = self.remainder.as_mut()?;
    if let Some(next_delim) = remainder.find(self.delimiter) {
      let until_delim = &remainder[..next_delim];
      *remainder = &remainder[(next_delim + self.delimiter.len())..];
      Some(until_delim)
    } else {
      self.remainder.take()
    }
  }
}

fn until_char<'a>(s: &'a str, c: char) -> &'a str {
  StrSplit::new(s, &format!("{}", c))
    .next()
    .expect("StrSplit always gives at least one result")
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
  }

  #[test]
  fn tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
  }

  #[test]
  fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
  }
}
