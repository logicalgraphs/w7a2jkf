use serde_variant::to_variant_name;

use shogi_kifu_converter::jkf::Preset;

use book::{
   json_utils::AsJSON,
   string_utils::{quot,bracket}
};

use super::utils::Lookup;

/*
pub struct JKF {
   header: Header,
   initial: Initial,
   moves: Vec<Move>
}
*/

struct Header { fields: Lookup }

impl AsJSON for Header {
   fn as_json(&self) -> String { json_block("header", &self.fields) }
}

struct Initial { preset: Preset }

impl Default for Initial {
   fn default() -> Self { Self { preset: Preset::PresetHirate } }
}

impl AsJSON for Initial {
   fn as_json(&self) -> String { 
      let foo: Lookup = 
         [("preset".to_string(),
           to_variant_name(&self.preset).unwrap().to_string())]
           .into_iter().collect();
      json_block("initial", &foo)
   }
}

// ----- helper functions in rendering JSON --------------------------------

fn quotty(vals: &[&str]) -> Vec<String> { 
   vals.into_iter().map(|s| quot(s)).collect()
}

fn json_attrib((k,v): (&String, &String)) -> String {
   format!("{}: {}", quot(k), quot(v))
}

fn json_hash(hash: &Lookup) -> String {
   let lin: Vec<String> = hash.iter().map(json_attrib).collect();
   bracket("{{}}", &format!("\n\t\t\t{} ", lin.join(",\n\t\t\t")))
}

fn json_block(name: &str, attribs: &Lookup) -> String {
   format!("\t{}:\n\t\t{}\n", quot(name), json_hash(attribs))
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
mod tests {
   use super::*;
   // use crate::types::conv;

   #[test]
   fn test_initial_json() {
      let init = Initial::default();
      let json = init.as_json();
      assert!(json.starts_with("\t\"initial\":"));
      assert!(json.contains("HIRATE"));
   }

   #[test]
   fn test_header_json() {
      let fields: Lookup = vec![
         ("開始日時", "2013/07/10 00:00:00"),
         ("先手", "Habu Yoshiharu, Oi"),
         ("後手", "Namekata Hisashi, Challenger"),
         ("棋戦", "54th Oi-sen, Game 1")]
         .into_iter()
         .map(|(k,v)| (k.to_string(), v.to_string()))
         .collect();
      let header = Header { fields };
      let json = header.as_json();
      assert!(json.starts_with("\t\"header\":"));
      assert!(json.contains("Habu"));
      assert!(json.contains("2013"));
      assert!(json.contains("54th Oi-sen"));
   }
}

