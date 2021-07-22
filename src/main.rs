pub mod mail;

fn main() {
    println!("Hello, world!");
    let addr_info = mail::parse_mail_addr(&String::from("user<user@test.com>"));
    println!("addr: {}", addr_info.addr);
}
