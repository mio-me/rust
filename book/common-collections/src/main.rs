#[allow(unused_variables)]
fn main() {
  let v: Vec<i32> = Vec::new();
  let mut v = vec![1, 2, 3];

  v.push(5);
  v.push(6);
  v.push(7);

  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
  }

  let v = vec![100, 32, 57];
  for i in &v {
    println!("{}", i);
  }

  let mut v = vec![100, 32, 57];
  for i in &mut v {
    *i += 50;
  }

  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];

  let data = "initial contents";
  let s = data.to_string();
  let s = "initial contents".to_string();
  let s = String::from(data);
  let mut s = String::from("initial contents");

  s.push_str("bar");
  s.push('l');

  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s2 is {}", s2);

  let s3 = s1 + &s2;

  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }

  use std::collections::HashMap;
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
  scores.insert(String::from("White"), 11);

  let score = scores.get("White");
  match score {
    Some(x) => println!("score is some {}", x),
    None => println!("none"),
  }

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);
  println!("{:?}", scores);

  scores.entry(String::from("Blue")).or_insert(50);
  println!("{:?}", scores);

  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  println!("{:?}", map);
}
