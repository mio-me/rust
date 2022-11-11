#[derive(PartialEq, Debug)]
pub struct StrSplit<'a> {
  remainder: Option<&'a str>,
  delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
  pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
    Self {
      remainder: Some(haystack),
      delimiter,
    }
  }
}

impl<'a> Iterator for StrSplit<'a> {
  type Item = &'a str;
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
}
