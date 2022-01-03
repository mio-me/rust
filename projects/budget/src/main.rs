use std::collections::hash_map::DefaultHasher;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::hash::{Hash, Hasher};
use std::process;

use rusqlite::{params, Connection};
mod transaction;

fn parse() -> Result<(), Box<dyn Error>> {
  let file_path = get_first_arg()?;
  let mut reader = csv::ReaderBuilder::new()
    .delimiter(b';')
    .flexible(true)
    .has_headers(false)
    .from_path(file_path)?;

  let conn = Connection::open("banking.db")?;
  init_db(&conn)?;

  let mut s = DefaultHasher::new();

  for record in reader.records().skip(13) {
    let record = record?;
    if let Ok(t) = transaction::from_vb_csv(&record) {
      t.hash(&mut s);
      add_transaction(&conn, &t, s.finish())?;
    }
  }
  Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
  match env::args_os().nth(1) {
    None => Err(From::from("Expected 1 argument, but got none")),
    Some(file_path) => Ok(file_path),
  }
}

fn main() {
  if let Err(err) = parse() {
    println!("{}", err);
    process::exit(1);
  }
}

fn add_transaction(
  conn: &Connection,
  t: &transaction::Transaction,
  hash: u64,
) -> rusqlite::Result<usize> {
  println!("{}: {:?}", hash, t);

  conn.execute(
    "INSERT INTO transactions (id, date, account, purpose, value) values (?1, ?2, ?3, ?4, ?5)",
    params![
      hash.to_string(),
      t.date.to_string(),
      t.account,
      t.purpose,
      t.value
    ],
  )
}

fn init_db(conn: &Connection) -> rusqlite::Result<()> {
  conn.execute(
    "create table if not exists transactions (
         id blob primary key,
         date text not null,
         account text not null,
         purpose text not null,
         value integer not null
     )",
    params![],
  )?;

  Ok(())
}
