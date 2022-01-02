use core::panic;
use std::error::Error;

use chrono::NaiveDate;
use simple_error::bail;

#[derive(Debug, PartialEq)]
pub struct Transaction {
  pub account: String,
  pub date: NaiveDate,
  pub value: i64,
}

pub fn from_vb_csv(record: &csv::StringRecord) -> Result<Transaction, Box<dyn Error>> {
  // "Buchungstag";
  // "Valuta";
  // "Auftraggeber/Zahlungsempf�nger";
  // "Empf�nger/Zahlungspflichtiger";
  // "Konto-Nr.";
  // "IBAN";
  // "BLZ";
  // "BIC";
  // "Vorgang/Verwendungszweck";
  // "Kundenreferenz";
  // "W�hrung";
  // "Umsatz";
  // " "

  if record.len() != 13 {
    bail!("Failed to parse. Entry too short.");
  }

  if [1usize, 3, 11, 12].iter().any(|i| &record[*i] == "") {
    bail!("Failed to parse. Missing fields.");
  }

  Ok(Transaction {
    account: String::from(&record[3]),
    date: NaiveDate::parse_from_str(&record[1], "%d.%m.%Y")?,
    value: get_value(&record[11], &record[12])?,
  })
}

fn get_value(raw_value: &str, sign: &str) -> Result<i64, Box<dyn Error>> {
  let value = raw_value.replace(",", "").parse::<i64>()?;

  match sign {
    "S" => Ok(-1 * value),
    "H" => Ok(value),
    _ => panic!("Failed to parse. No sign."),
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
        account: "Receiver".to_string(),
        date: NaiveDate::from_ymd(2021, 12, 23),
        value: -909
      })
    );
  }
}
