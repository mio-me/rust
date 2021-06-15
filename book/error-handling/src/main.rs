use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

#[allow(unused_variables)]
fn main() {
  // panic!("crash and burn");

  let v = vec![1, 2, 3];
  // v[99];

  use std::fs::File;
  let f = File::open("hello.txt");
  let f = match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };

  let f = File::open("hello.txt").unwrap();
  let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

#[allow(unused)]
fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}

#[allow(unused)]
fn short_read_username_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

#[allow(unused)]
fn even_shorter_read_username_from_file() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
