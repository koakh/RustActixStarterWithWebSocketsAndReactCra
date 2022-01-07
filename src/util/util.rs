use log::debug;

/// generate a fixed size char line
pub fn gen_line_char(character: char, width: u8) -> String {
  let mut buf = String::new();
  for _ in 0..width {
    buf.push(character);
  }
  buf
}

/// simple helper, just to prevent extra code lines
pub fn out_message(message: String, indent: u8) {
  println!("{}{}", gen_line_char(' ', indent), &message);
  debug!("{}", &message);
}
