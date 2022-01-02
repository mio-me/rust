use std::env;
use std::error::Error;
use std::ffi::OsString;
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

  for record in reader.records().skip(13) {
    if let Ok(t) = transaction::from_vb_csv(&record?) {
      println!("{:?}", t);
      add_transaction(&conn, &t)?;
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

fn add_transaction(conn: &Connection, t: &transaction::Transaction) -> rusqlite::Result<usize> {
  conn.execute(
    "INSERT INTO transactions (date, account, value) values (?1, ?2, ?3)",
    params![t.date.to_string(), t.account, t.value],
  )
}

fn init_db(conn: &Connection) -> rusqlite::Result<()> {
  conn.execute(
    "create table if not exists transactions (
         id integer primary key,
         date text not null,
         account text not null,
         value integer not null
     )",
    params![],
  )?;

  conn.execute(
    "create table if not exists account (
         id integer primary key,
         name text not null
     )",
    params![],
  )?;

  Ok(())
}
