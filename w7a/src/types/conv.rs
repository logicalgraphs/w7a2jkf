use std::collections::HashMap;

use chrono::Month;

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
      Ok(format!("{}{n}", if n < 10 { "0" } else { "" }))
   }
}
