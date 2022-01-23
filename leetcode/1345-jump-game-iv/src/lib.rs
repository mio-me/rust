use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;
impl Solution {
  pub fn min_jumps(arr: Vec<i32>) -> i32 {
    let n = arr.len() as i32;

    if n < 2 {
      return 0;
    }

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for (i, x) in arr.iter().enumerate() {
      match map.get_mut(x) {
        Some(v) => v.push(i as i32),
        _ => {
          map.insert(*x, vec![i as i32]);
        }
      }
    }

    let mut visited = HashSet::new();
    visited.insert(-1);
    let mut queue: VecDeque<Vec<i32>> = std::collections::VecDeque::new();
    queue.push_back(vec![0]);
    for step in 0.. {
      let mut next = vec![];
      for pos in queue.pop_front().unwrap() {
        visited.insert(pos);
        let mut x = vec![pos - 1, pos + 1];
        if let Some(same) = map.get_mut(&arr[pos as usize]) {
          x.append(same);
        }
        for i in x {
          if i == n - 1 {
            return step + 1;
          }
          if !visited.contains(&i) {
            next.push(i)
          }
        }
      }
      queue.push_back(next);
    }
    -1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(
      Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
      3
    );
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::min_jumps(vec![7]), 0);
  }

  #[test]
  fn example3() {
    assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
  }
}
