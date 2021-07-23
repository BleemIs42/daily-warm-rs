use std::{collections::hash_map::DefaultHasher, hash::Hasher};
use std::hash::Hash;

use mailparse::{addrparse, MailAddr, SingleInfo};
use chrono::{Utc};

pub struct Config {
  host: String,
  port: u32,
  username: String,
  password: String,
  from: String,
}

// const DEFAULT_CONFIG:Config = Config {
//   host: String::from("smtp.qq.com"),
//   port: 25,
//   username:  String::from("smtp.qq.com"),
//   password:  String::from("smtp.qq.com"),
//   from:  String::from("smtp.qq.com"),
// };

pub struct RMail{
  pub from: String,
  pub to: Vec<String>,
  pub cc: Vec<String>,
  pub bcc: Vec<String>,
  pub subject: String,
  pub content: String,
}

pub fn parse_mail_addr(address: &str) -> SingleInfo {
  match &addrparse(address).unwrap()[0] {
      MailAddr::Single(info) => info.clone(),
      _ => panic!()
  }
}


// boundary := getBoundary()
// fmt.Fprintf(&buf, "Date: %s%s", time.Now().UTC().Format(time.RFC822), crlf)
// fmt.Fprintf(&buf, "Subject: %s%s", gm.Subject, crlf)
// fmt.Fprintf(&buf, "Content-Type: multipart/alternative; boundary=%s%s%s", boundary, crlf, crlf)
// fmt.Fprintf(&buf, "%s%s", "--"+boundary, crlf)
// fmt.Fprintf(&buf, "Content-Type: text/HTML; charset=UTF-8%s", crlf)
// fmt.Fprintf(&buf, "%s%s%s%s", crlf, gm.Content, crlf, crlf)
// fmt.Fprintf(&buf, "%s%s", "--"+boundary+"--", crlf)

impl RMail {
    pub fn create(self) -> String{
      let crlf = "\r\n";
      let boundary = format!("{:x}", md5::compute(Utc::now().to_rfc2822())) ;
      let mail_str = format!(
        concat!(
          "From: {from}{crlf}",
          "To: {to}{crlf}",
          "Cc: {cc}{crlf}",
          "Bcc: {bcc}{crlf}",
          "Date: {date}{crlf}",
          "Subject: {subject}{crlf}",
          "Content-Type: multipart/alternative; boundary={boundary}{crlf}{crlf}",
          "--{boundary}{crlf}",
          "Content-Type: text/HTML; charset=UTF-8{crlf}",
          "{crlf}{content}{crlf}{crlf}",
          "--{boundary}--{crlf}",
        ),
        crlf = crlf,
        from = self.from, 
        to = self.to.join(", "),
        cc = self.cc.join(", "),
        bcc = self.bcc.join(", "),
        date = Utc::now().to_rfc2822(),
        subject = self.subject,
        boundary = boundary,
        content = self.content,
    );
      mail_str
    }
}