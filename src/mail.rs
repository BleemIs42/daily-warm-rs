  use mailparse::{addrparse, MailAddr, SingleInfo};

pub struct Config {
  host: String,
  port: u32,
  username: String,
  password: String,
  from: String,
}

// static DEFAULT_CONFIG:Config = Config {
//   host: String::from("smtp.qq.com"),
//   port: 25,
//   username:  String::from("smtp.qq.com"),
//   password:  String::from("smtp.qq.com"),
//   from:  String::from("smtp.qq.com"),
// };

pub struct RMail{
  from: String,
  to: Vec<String>,
  cc: Vec<String>,
  bcc: Vec<String>,
  subject: String,
  content: String,
}

pub fn parse_mail_addr(address: &str) -> SingleInfo {
  match &addrparse(address).unwrap()[0] {
      MailAddr::Single(info) => info.clone(),
      _ => panic!()
  }
}