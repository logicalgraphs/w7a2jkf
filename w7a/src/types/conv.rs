use chrono::Month;

use book::err_utils::{ErrStr,err_or};

// use super::utils::Lookup;

/// Converts from w7a-types to JKF

pub trait Convert { fn convert(&self, raw: &str) -> ErrStr<String>; }

/*
pub struct Converter { header: Lookup }

impl Default for Converter {
   fn default() -> Self {
      Self { header: headers() }
   }
}

fn populate(raw: &[(&str, &str)]) -> Lookup {
   fn strify((a, b): &(&str, &str)) -> (String, String) {
      (a.to_string(), b.to_string())
   }
   raw.iter().map(strify).collect()
}

fn headers() -> Lookup {
   populate(&[("Black", "後手"), ("White", "先手"),
              ("Event", "棋戦"), ("Date", "開始日時")])
}
*/

// ---- DATE-stuff -------------------------------------------------------

enum DateConverter { MONTH, DATE }

use DateConverter::*;

impl Convert for DateConverter {
   fn convert(&self, s: &str) -> ErrStr<String> {
      match self {
         MONTH => convert_month(s),
         DATE => convert_date(s)
      }
   }
}

fn convert_month(m: &str) -> ErrStr<String> {
   let mos: Month = err_or(m.parse(), &format!("Can't parse month {m}"))?;
   let n = mos.number_from_month();
   Ok(two_digits(n))
}

// the Datish, as you see from the example, is broken up into
// month [day-of-month information] year
fn convert_date(dt: &str) -> ErrStr<String> {
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
      Ok(format!("{year_str}/{m1}/{} 00:00:01", two_digits(d1)))
   } else {
      Err(format!("Cannot parse the year {year_str}"))
   }
}

fn two_digits(n: u32) -> String {
   format!("{}{n}", if n < 10 { "0" } else { "" })
}

#[cfg(test)]
mod tests {

   use super::*;

   fn convert_month(m: &str) -> ErrStr<String> {
      MONTH.convert(m)
   }
   
   fn pass_month(m: &str, exp: &str) {
      let mos = convert_month(m);
      assert_eq!(Ok(exp.to_string()), mos);
   }

   fn fail_month(m: &str) {
      let ans = convert_month(m);
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

   fn convert_date(dt: &str) -> ErrStr<String> {
      DATE.convert(dt)
   }

   fn pass_date(dt: &str, exp: &str) {
      let ans = convert_date(dt);
      assert_eq!(Ok(format!("{exp} 00:00:01")), ans);
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
      let garbage_out = convert_date("12 Caban 15 Ceh");
      assert!(garbage_out.is_err());
   }
}

