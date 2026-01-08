use std::{
   collections::HashMap,
   fmt
};

use chrono::Month;

use book::{
   err_utils::{ErrStr,err_or},
   json_utils::AsJSON
};

use super::{
   jkf::{Header as JHdr, mk_jhdr, Initial},
   w7a::Header as Hdr,
   utils::Lookup
};

/// Converts from w7a-types to JKF

pub trait Convert<W, J: AsJSON> { 
   fn convert(&self, domain: &W) -> ErrStr<J>; 
}

type XformJ = dyn Fn(String) -> ErrStr<JsonString>;
type Transform<'a> = HashMap<(String, String), &'a XformJ>;

pub struct Converter<'a> { header: Transform<'a> }

impl<'a> Default for Converter<'a> {
   fn default() -> Self {
      Self { header: headers() }
   }
}

fn to_j_str(s: String) -> ErrStr<JsonString> { Ok(mk_jstr(&s)) }
fn to_j_dt(s: String) -> ErrStr<JsonString> { convert_date(&s) }

fn populate<'a>(raw: &[((&str, &str), &'a XformJ)]) -> Transform<'a> {
   fn strify((a, b): &(&str, &str)) -> (String, String) {
      (a.to_string(), b.to_string())
   }
   raw.into_iter().map(|(k,v)| (strify(k), v.clone())).collect()
   // raw.into_iter().map(first(strify)).collect() would be better, but ...
   // the type-signatures go crazy on dynamic function cloning. Oh, well!
}

fn headers<'a>() -> Transform<'a> {
   populate(&[(("Black", "後手"), &to_j_str), (("White", "先手"), &to_j_str),
              (("Event", "棋戦"), &to_j_str), (("Date", "開始日時"), &to_j_dt)])
}

// The Predule encapsulates the header- and initial-sections of the JKF
struct Prelude {
   header: JHdr,
   initial: Initial
}

impl AsJSON for Prelude {
   fn as_json(&self) -> String {
      format!("{},\n{}", self.header.as_json(), self.initial.as_json())
   }
}

impl<'a> Convert<Hdr, Prelude> for Converter<'a> {
   fn convert(&self, domain: &Hdr) -> ErrStr<Prelude> {
      let mut hdr: Lookup = HashMap::new();
      for (key,f) in &self.header {
         let (k,v) = key;
         if let Some(raw_val) = domain.header.get(k) {
            let ans = f(raw_val.to_string())?.as_json();
            hdr.insert(v.clone(), ans);
         }
      }
      Ok(Prelude { header: mk_jhdr(hdr), initial: Initial::default() })
   }
}

// ---- DATE-stuff -------------------------------------------------------

#[derive(Debug,Clone,PartialEq)]
struct JsonString { string: String }
fn mk_jstr(s: &str) -> JsonString {
   JsonString { string: s.to_string() }
}

impl AsJSON for JsonString {
   fn as_json(&self) -> String { self.string.clone() }
}
impl fmt::Display for JsonString {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.string)
   }
}

enum DateConverter { MONTH, DATE }

use DateConverter::*;

impl Convert<String, JsonString> for DateConverter {
   fn convert(&self, s: &String) -> ErrStr<JsonString> {
      match self {
         MONTH => convert_month(s),
         DATE => convert_date(s)
      }
   }
}

fn convert_month(m: &str) -> ErrStr<JsonString> {
   let mos: Month = err_or(m.parse(), &format!("Can't parse month {m}"))?;
   let n = mos.number_from_month();
   Ok(two_digits(n))
}

// the Datish, as you see from the example, is broken up into
// month [day-of-month information] year
fn convert_date(dt: &str) -> ErrStr<JsonString> {
   let date_parts: Vec<&str> = dt.split(" ").collect();
   let mos = date_parts.first()
                       .ok_or(format!("no month in empty string"))?;
   let m1 = convert_month(&mos)?;
   let day = date_parts.get(1)
                       .ok_or(&format!("Could not fetch day from {dt}"))?;
   let day_nums: String =
            day.chars().filter(|c| c.is_ascii_digit()).collect();
   let d1 = err_or(day_nums.parse(),
                   &format!("Cannot parse day from {day_nums}"))?;
   let year_str = date_parts.last()
                            .ok_or("Cannot scan year from empty string")?;
   if year_str.chars().all(|c| c.is_ascii_digit()) {
      let json = mk_jstr(&format!("{year_str}/{m1}/{} 00:00:01",
                                  two_digits(d1)));
      Ok(json)
   } else {
      Err(format!("Cannot parse the year {year_str}"))
   }
}

fn two_digits(n: u32) -> JsonString {
   mk_jstr(&format!("{}{n}", if n < 10 { "0" } else { "" }))
}

#[cfg(test)]
mod tests {

   use super::*;

   use crate::types::w7a::load_w7a_header;

   fn convert_month(m: &String) -> ErrStr<JsonString> {
      MONTH.convert(m)
   }
   
   fn pass_month(m: &str, exp: &str) {
      let mos = convert_month(&m.to_string());
      assert_eq!(Ok(mk_jstr(exp)), mos);
   }

   fn fail_month(m: &str) {
      let ans = convert_month(&m.to_string());
      assert!(ans.is_err());
   }

   #[test]
   fn test_convert_month() {
      pass_month("April", "04");
   }

   #[test]
   fn fail_convert_month() {
      fail_month("Lavinge");
   }

   fn convert_date(dt: &String) -> ErrStr<JsonString> {
      DATE.convert(dt)
   }

   fn pass_date(dt: &str, exp: &str) {
      let ans = convert_date(&dt.to_string());
      assert_eq!(Ok(mk_jstr(&format!("{exp} 00:00:01"))), ans);
   }

   #[test]
   fn test_my_birthday() {
      pass_date("April 26th, 1967", "1967/04/26");
   }

   // The date of the 54th Oi, game 1 is: [Date "July 10th and 11th 2013"]
   #[test] 
   fn test_54th_oi_game_1_date() {
      pass_date("July 10th and 11th 2013", "2013/07/10");
   }

   #[test]
   fn fail_mayan_date() {
      let garbage_out = convert_date(&"12 Caban 15 Ceh".to_string());
      assert!(garbage_out.is_err());
   }

   #[test]
   fn test_convert_w7a_header() -> ErrStr<()> {
      let (hdr, _rest) = load_w7a_header("data/tests/sample-header.w7a")?;
      let conv = Converter::default();
      let json = conv.convert(&hdr);
      assert!(!json.is_err());
      // I don't know how to test JSON-output? Schema-verification, perhaps?
      // json.and_then(|j| { assert_eq!(" ", j.as_json()); Ok(())})
      Ok(())
   }
}

