use book::{
   err_utils::ErrStr,
   file_utils::lines_from_file,
   utils::get_args
};

fn main() -> ErrStr<()> {
   let args = get_args();
   let filename = args.first().ok_or_else(|| usage())?;
   let lines = lines_from_file(&filename)?;
   let len = lines.len();
   let v = len - 5;
   let last5 = &lines[v..];
   println!("I finished with {last5:?}");
   Ok(())
}

fn usage() -> String {
   println!("$ ./w7a2jkf <filename>

Converts a Western-style Shogi game record to JKF (JSON Kifu Format)

where:

* <filename> is the path (and filename) of the w7a-formatted file");
   "Needs <filename> argument".to_string()
}
