use std::collections::HashMap;

use chrono::Month;

use book::err_utils::{ErrStr,err_or};

/// Converts from w7a-types to JKF

pub trait Convert { fn convert(&self, &str) -> ErrStr<String>; }

type Lookup = HashMap<String, String>;

pub struct Converter { headers: Lookup }

impl Default for Converter {
   fn default() -> Self {
      Self { headers: headers() }
   }
}

fn populate(raw: &[(&str, &str)]) -> Lookup {
   fn strify((a, b): (&str, &str)) -> (String, String) {
      (a.to_string(), b.to_string())
   }
   let list: &[(String, String)] = raw.iter().map(strify).collect();
   from(list)
}

fn headers() -> Lookup {
   populate(&[("Black", "後手"), ("White", "先手"),
              ("Event", "棋戦"), ("Date", "開始日時")])
}

struct Months { }

impl Convert for Months {
   fn convert(&self, m: &str) -> ErrStr<String> {
      let mos: Month = err_or(m.parse(), format!("Cannot parse month {m}"))?;
      let n = mos.number_from_month();
      Ok(two_digits(n + 1))
   }
}

pub fn two_digits(n: usize) -> String {
   format!("{}{n}", if n < 10 { "0" } else { "" })
}

struct Datish { } // not a Date, but a Date-..ish

impl Convert for Datish {
   // the Datish, as you see from the example, is broken up into
   // month [day information] year
   fn convert(&self, dt: &str) -> ErrStr<String> {
      let m = Months::new();
      let date_parts = dt.split(" ");
      let mos = date_parts.first()
                          .ok_or(format!("month not found in empty string"))?;
      let m1 = m.convert(&mos)?;
      let day = date_parts.get(1)
                          .ok_or(format!("Could not fetch day from {dt}"))?;
      let day_nums: String =
         day.chars().filter(|c| c.is_ascii_digit()).collect();
      let d1 = err_or(day_nums.parse(),
                      format!("Cannot parse day from {day_nums}"))?;
      let year_str =
         date_parts.last()
                   .ok_or("Cannot scan year from empty string".to_string())?;
      if year_str.chars().all(|c| c.is_ascii_digit()) {
         Ok(format!("{year_str}/{m1}/{} 00:00:01", two_digits(d1)))
      } else {
         Err(format!("Cannot parse the year {year_str}"))
      }
}

