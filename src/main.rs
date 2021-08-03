mod mail;
mod api;

use mail::Render;

fn main() {
    // let data = 
    let data = "{\"weather\":{\"city\":\"test city\"}}".to_string();
    let mr = Render::new(data);
    println!("Hello, world!");
    // let mail =  mr.get_content();
    // println!("{}",mail);

    let weather = api::get_weather("shaanxi/xian");
    let one = api::get_one();
    let english = api::get_english();
    let poem = api::get_poem();
    let wallpaper = api::get_wallpaper();
    // let trivia = api::get_trivia();

    println!("{:?} \n{:?}\n{:?}\n{:?}\n{:?}", weather, one, english, poem, wallpaper);
}
