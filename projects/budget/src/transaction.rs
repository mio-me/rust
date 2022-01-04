use std::error::Error;
use chrono::NaiveDate;
use simple_error::bail;

#[derive(Debug, PartialEq, Hash)]
pub struct Transaction {
  pub date: NaiveDate,
  pub account: String,
  pub purpose: String,
  pub value: i64,
}

pub fn from_vb_csv(record: &csv::StringRecord) -> Result<Transaction, Box<dyn Error>> {
  if record.len() != 13 {
    bail!("Failed to parse. Invalid entry length.");
  }

  if [1usize, 3, 8, 11, 12].iter().any(|i| &record[*i] == "") {
    bail!("Failed to parse. Missing field.");
  }

  Ok(Transaction {
    date: NaiveDate::parse_from_str(&record[1], "%d.%m.%Y")?,
    account: String::from(&record[3]),
    purpose: String::from(&record[8]),
    value: get_value(&record[11], &record[12])?,
  })
}

fn get_value(raw_value: &str, sign: &str) -> Result<i64, Box<dyn Error>> {
  let value = raw_value.replace(",", "").parse::<i64>()?;

  match sign {
    "S" => Ok(-1 * value),
    "H" => Ok(value),
    _ => bail!("Failed to parse. No sign."),
  }
}

#[cfg(test)]
mod tests {
  use chrono::NaiveDate;

  use super::*;

  #[test]
  fn parsing_vb_csv_record() {
    let records = csv::ReaderBuilder::new().has_headers(false).delimiter(b';').from_reader("\"23.12.2021\";\"23.12.2021\";\"Sender\";\"Receiver\";;\"DE0ISBN\";;\"VBBIC\";\"reason bla bla\";;\"EUR\";\"9,09\";\"S\"".as_bytes())
      .records().map(|t| from_vb_csv(&t.unwrap()).unwrap()).collect::<Vec<_>>();

    assert_eq!(
      records.get(0),
      Some(&Transaction {
        date: NaiveDate::from_ymd(2021, 12, 23),
        account: "Receiver".to_string(),
        purpose: "reason bla bla".to_string(),
        value: -909
      })
    );
  }
}
