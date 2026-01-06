use book::{
   file_utils::lines_from_file,
   err_utils::ErrStr,
   string_utils::to_string,
   compose
};

use super::utils::Lookup;

pub struct W7A {
   header: Header
   // moves: Moves
   // state: ParserState
}

pub struct Header {
   pub header: Lookup
}

trait Scanner {
   fn ingest(lines: &[String]) -> ErrStr<(Self, Vec<String>)>
      where Self: Sized;
}

impl Scanner for W7A {
   fn ingest(lines: &[String]) -> ErrStr<(Self, Vec<String>)> {
      let (hdr, rest) = Header::ingest(lines)?;
      Ok((W7A { header: hdr }, rest))
   }
}

impl Scanner for Header {
   fn ingest(lines: &[String]) -> ErrStr<(Self, Vec<String>)> {
      ingest_header(lines)
   }
}

fn ingest_header(lines: &[String]) -> ErrStr<(Header, Vec<String>)> {
   let (hdr, tail): (Vec<&String>, Vec<&String>) =
      lines.into_iter().partition(|line| line.starts_with("["));
   let hash: Lookup = hdr.into_iter()
                         .filter_map(compose!(Result::ok)(scan_header_line))
                         .collect();
   let rest: Vec<String> = tail.into_iter().map(String::to_string).collect();
   Ok((Header { header: hash },rest))
}

fn scan_header_line(line: &String) -> ErrStr<(String, String)> {
   let tokens: Vec<String> = line.split("\"").map(to_string).collect();
   let key = tokens.first().and_then(|k| {
      let k1: String = k.chars().filter(|c| c.is_alphabetic()).collect();
      Some(k1)
   }).ok_or("Cannot get key from empty line".to_string())?;
   let value = tokens.get(1).ok_or(format!("No quotes in header line {line}"))?;
   Ok((key, value.to_string()))
}

/*
#[derive(Debug,Clone)]
enum ParserState { START, HEADER, MOVES, END };
*/

fn load_file(filename: &str) -> ErrStr<Vec<String>> {
   // test filename = "data/tests/sample-header.w7a"
   let lines = lines_from_file(filename);
   assert!(lines.is_ok());
   lines
}

pub fn load_w7a_header(filename: &str) -> ErrStr<(Header, Vec<String>)> {
   let lines = load_file(filename)?;
   ingest_header(&lines)
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_scan_header_line() {
      let line = &"[Black \"Habu Yoshiharu, Oi\"]".to_string();
      let scanned = scan_header_line(line);
      assert!(scanned.is_ok());
      let _ = scanned.and_then(|(k, v)| {
         assert_eq!("Black", &k);
         assert!(v.starts_with("Habu"));
         Ok("foo")
      });
   }

   fn load_test_file() -> ErrStr<Vec<String>> {
      load_file("data/tests/sample-header.w7a")
   }

   #[test]
   fn test_scan_header() -> ErrStr<()> {
      let file = load_test_file()?;
      let scanned = ingest_header(&file);
      assert!(scanned.is_ok());
      scanned.and_then(|(header, rest)| {
         assert!(!rest.is_empty());
         assert_eq!(4, header.header.len());
         Ok(())
      })
   }

   #[test]
   fn test_create_header_from_scan() -> ErrStr<()> {
      let file = load_test_file()?;
      let (header, rest) = Header::ingest(&file)?;
      assert!(!rest.is_empty());
      assert_eq!(4, header.header.len());
      Ok(())
   }

   #[test]
   fn test_create_w7a_from_scan() -> ErrStr<()> {
      let file = load_test_file()?;
      let (_game, rest) = W7A::ingest(&file)?;
      assert!(rest.is_empty());
      Ok(())
   }
}

