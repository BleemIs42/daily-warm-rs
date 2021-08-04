mod api;
mod mail;

use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct MailTo {
    email: String,
    local: String,
}

#[derive(Deserialize)]
pub struct Config {
    username: String,
    password: String,
    host: String,
    port: u16,
    subject: String,
    cron: String,
    from: String,
    to: MailTo,
    mode: String,
}

fn main() {
    let config = get_config();
    let data = api::get_data(&config);
    let mr = mail::Render::new(data);
    println!("Hello, world!");
    let mail_content = mr.get_content();
    // println!("{}", mail_content);
    // fs::write("mail.html", mail_content.clone()).unwrap();
    mail::send(&config, mail_content);
}

fn get_config() -> Config {
    let toml_config = String::from_utf8_lossy(&fs::read(".env").unwrap())
        .parse::<String>()
        .unwrap();
    toml::from_str(&toml_config).unwrap()
}
