use chrono::Duration;

use book::{
   file_utils::lines_from_file,
   err_utils::ErrStr,
   list_utils::ht,
   string_utils::to_string,
   compose
};

use super::utils::Lookup;

trait Scanner {
   fn ingest(lines: &[String]) -> ErrStr<(Self, Vec<String>)>
      where Self: Sized;
}

pub struct W7A {
   header: Header,
   game_comment: GameComment
   // moves: Vec<Move>
}

impl Scanner for W7A {
   fn ingest(lines: &[String]) -> ErrStr<(Self, Vec<String>)> {
      let (header, rest) = Header::ingest(lines)?;
      let (game_comment, tail) = GameComment::ingest(&rest)?;
      Ok((W7A { header, game_comment }, tail))
   }
}

pub struct Header {
   pub header: Lookup
}

impl Scanner for Header {
   fn ingest(lines: &[String]) -> ErrStr<(Self, Vec<String>)> {
      ingest_header(lines)
   }
}

type Comment = Option<String>;

/// The GameComment is special:

/// 1. There's only one of them (if present)
/// 2. it occurs between the Header and the first Move

pub struct GameComment { comment: Comment } 

impl Scanner for GameComment {
   fn ingest(lines: &[String]) -> ErrStr<(Self, Vec<String>)> {
      // From here to the line starting with "1." is either the GameComment
      // or a set of empty lines, which we ignore
      let (comment, rest) = collect_comment(lines)?;
      Ok((GameComment { comment }, rest))
   }
}

// e.g.: 1.P7g-7f     00:00:00  00:00:00
pub struct Move {
   n: usize,
   piece: Piece,
   from: Option<Position>, // None meaning from hand
   to: Position,
   promote: bool,
   capture: bool,
   drop: bool,
   total_time: Duration,
   comment: Comment
}

#[derive(Debug,Clone,PartialEq)]
pub enum Color { BLACK, WHITE }

use Color::*;

pub fn color(m: &Move) -> Color {
   if m.n % 2 == 1 { BLACK } else { WHITE }
}

/*
impl Scanner for Move {
   fn ingest(lines: &[String]) -> ErrStr<(Self, Vec<String>)> {
      // xxx
   }
}
*/

pub fn dur(a: Option<&Move>, b: &Move) -> Duration {
   let start =
      a.and_then(|m| Some(m.total_time))
       .or(Some(Duration::zero()))
       .unwrap();
   b.total_time - start
}

#[derive(Debug,Clone,PartialEq)]
pub enum Piece { PAWN }

use Piece::*;

#[derive(Debug,Clone,PartialEq)]
struct Position { x: usize, y: String } 

// ----- helper functions for scanning the W7A file -------------------------

fn parse_piece(c: char) -> ErrStr<Piece> {
   match c.to_ascii_uppercase() {
      'P' => Ok(PAWN),
      _   => Err(format!("No piece exists for char {c}"))
   }
}

fn is_move(line: &str) -> bool {
   if line.is_empty() {
      false
   } else {
      let thunks: Vec<&str> = line.split(".").collect();
      thunks.first()
            .and_then(|word| Some(word.chars().all(|c| c.is_ascii_digit())))
            .or(Some(false))
            .unwrap()
   }
}

/*
fn parse_move(line: &String) -> ErrStr<Move> {
   if !is_move(line) {
      Err(format!("line {line} is not a move"))
   } else {
      parse_move1(line)
   }
}

fn parse_move1(line: &String) -> ErrStr<Move> {
   
}
*/

fn collect_comment(lines: &[String]) -> ErrStr<(Comment, Vec<String>)> {
   let mut comment_lines: Vec<String> = Vec::new();
   let mut file = lines.to_vec();
   loop {
      if let (Some(line), rest) = ht(&file) {
         if is_move(&line) { break; }
         comment_lines.push(line);
         file = rest.clone();
         if rest.is_empty() { break; }
      } else {
         break;
      }
   };
   Ok((if comment_lines.iter().all(String::is_empty) {
      None
   } else {
      Some(comment_lines.join(" "))
   }, file))
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

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
mod tests {
   use super::*;

   // --- HEADER TESTS ----------------------------------------
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

   fn load_test_header() -> ErrStr<Vec<String>> {
      load_file("data/tests/sample-header.w7a")
   }

   #[test]
   fn test_scan_header() -> ErrStr<()> {
      let file = load_test_header()?;
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
      let file = load_test_header()?;
      let (header, rest) = Header::ingest(&file)?;
      assert!(!rest.is_empty());
      assert_eq!(4, header.header.len());
      Ok(())
   }

   // --- BODY TESTS -------------------------------------

   // --- game comment -----------------------------------

   fn load_test_comment() -> ErrStr<Vec<String>> {
      load_file("data/tests/just-comment.w7a")
   }
   fn load_game_comment() -> ErrStr<Vec<String>> {
      load_file("data/tests/sample-game-comment-with-no-moves.w7a")
   }
   fn load_oi_game() -> ErrStr<Vec<String>> {
      let game_dir = "../data/game_records/reijer_grimberger";
      let game = "2013-07-11-54th-oi-sen-game-1.w7a";
      load_file(&format!("{game_dir}/{game}"))
   }

   #[test]
   fn test_read_just_a_comment() -> ErrStr<()> {
      let file = load_test_comment()?;
      let (comment, rest) = collect_comment(&file)?;
      assert!(comment.is_some());
      let _ = comment.and_then(|c| { assert!(!c.is_empty()); Some(c) });
      assert!(rest.is_empty());
      Ok(())
   }

   #[test]
   fn test_ingest_game_comment() -> ErrStr<()> {
      let file = load_test_comment()?;
      let (game_comment, rest) = GameComment::ingest(&file)?;
      assert!(game_comment.comment.is_some());
      let _ = game_comment.comment.and_then(|c| {
         assert!(!c.is_empty());
         Some(())
      });
      assert!(rest.is_empty());
      Ok(())
   }

   #[test]
   fn test_ingest_game_comment_no_moves() -> ErrStr<()> {
      let file = load_game_comment()?;
      let (game, rest) = W7A::ingest(&file)?;
      assert!(game.game_comment.comment.is_some());
      let _ = game.game_comment.comment.and_then(|c| {
         assert!(!c.is_empty());
         Some(())
      });
      assert!(rest.is_empty());
      Ok(())
   }

   // --- move tests -------------------------------------

   #[test]
   fn test_move_line() {
      assert!(is_move("75.G3bx3c    07:40:00  07:18:00"));
   }

   #[test]
   fn fail_move_line() {
      let sentence1 = "This is the move that Namekata had put his hopes on";
      let sentence2 = "It defends against the mating";
      assert!(!is_move(&format!("{sentence1}. {sentence2}")));
   }

// Move-format: 1.P7g-7f     00:00:00  00:00:00

   fn mk_test_move(n: usize, seggs: i64) -> Move {
      Move { n, piece: PAWN, from: None, 
             to: Position { x: 7, y: "d".to_string() },
             promote: false, capture: false, drop: false,
             total_time: Duration::seconds(seggs), comment: None }
   }

   // --- Duration ----------------------------------------

   #[test]
   fn test_duration_first_move() {
      let dur2 = mk_test_move(3, 3);
      assert_eq!(Duration::seconds(3), dur(None, &dur2));
   }

   #[test]
   fn test_duration_later_move() {
      let dur1 = mk_test_move(15, 963);
      let dur2 = mk_test_move(17, 1150);
      assert_eq!(Duration::seconds(187), dur(Some(&dur1), &dur2));
   }

   // --- Color --------------------------------------------------

   #[test]
   fn test_sente_color() {
      let sente = mk_test_move(17, 99);
      assert_eq!(BLACK, color(&sente));
   }

   #[test]
   fn test_gote_color() {
      let gote = mk_test_move(18, 999);
      assert_eq!(WHITE, color(&gote));
   }

   // --- pieces -------------------------------------------------

   #[test]
   fn test_parse_piece() -> ErrStr<()> {
      let p = parse_piece('p')?;
      assert_eq!(PAWN, p);
      Ok(())
   }

   #[test]
   fn fail_parse_piece() {
      let q = parse_piece('Q'); // ... No QUEENS! Take that, Dictator-for-Life!
      assert!(q.is_err());
   }

   // --- ingest -------------------------------------------------

   #[test]
   fn test_create_w7a_from_scan() -> ErrStr<()> {
      let file = load_oi_game()?;
      let (_game, rest) = W7A::ingest(&file)?;
      assert!(rest.is_empty());
      Ok(())
   }
}

