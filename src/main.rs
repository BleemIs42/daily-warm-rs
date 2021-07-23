mod mail;

use mail::Render;

fn main() {
    let data = "{\"Weather\":{\"City\":\"test city\"}}".to_string();
    let mr = Render::new(data);
    println!("Hello, world!");
    let mail =  mr.get_content();
    println!("{}",mail);
}
