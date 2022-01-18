pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
  flowerbed.push(0);

  flowerbed
    .windows(2)
    .scan((0, 0), |(count, left), w| {
      println!("{:?}, {:?}, {:?}", count, left, w);
      if *left + w[0] + w[1] == 0 {
        *count = *count + 1;
        *left = 1;
      } else {
        *left = w[0];
      }
      Some(*count)
    })
    .last()
    .unwrap()
    >= n as usize
}

#[cfg(test)]
mod tests {
  #[test]
  fn can_place() {
    assert!(super::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
  }

  #[test]
  fn cant_place() {
    assert_eq!(super::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
  }

  #[test]
  fn cant_place2() {
    assert_eq!(super::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2), false);
  }
}
