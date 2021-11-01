use serde::Deserialize;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

#[derive(Deserialize, Debug)]
struct Transaction {
  from: String,
  to: String,
  value: f32,
  date: chrono::NaiveDate,
}

fn parse() -> Result<Vec<Transaction>, Box<dyn Error>> {
  let file_path = get_first_arg()?;
  println!("Reading {:?}", file_path);
  let file = File::open(file_path)?;
  let mut reader = csv::ReaderBuilder::new()
    .delimiter(b',')
    .flexible(true)
    .has_headers(true)
    .from_reader(file);
  let mut res = vec![];
  for record in reader.deserialize() {
    let record: Result<Transaction, csv::Error> = record;
    match record {
      Ok(t) => {
        println!("{:?}", t);
        res.push(t);
      }
      Err(e) => {
        println!("Error while parsing: {:?}", e);
      }
    }
  }
  Ok(res)
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
  match env::args_os().nth(1) {
    None => Err(From::from("expected 1 argument, but got none")),
    Some(file_path) => Ok(file_path),
  }
}

fn main() {
  if let Err(err) = parse() {
    println!("{}", err);
    process::exit(1);
  }
}
