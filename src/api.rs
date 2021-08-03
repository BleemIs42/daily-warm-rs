use std::string;

// use std::collections::HashMap;
use reqwest;
use scraper::{ElementRef, Html, Selector};
use crate::mail::{Weather, One, English, Poem, Wallpaper, Trivia};
use serde::{ Serialize, Deserialize, de::DeserializeOwned };

pub fn get_html(url: &str) -> Html{
  let html_str = reqwest::blocking::get(url).unwrap().text().unwrap();
  Html::parse_document(html_str.as_str())
}

fn get_element<'a, 'b>(elm: ElementRef<'a>, selector_str: &'b str) -> ElementRef<'a>{
  let selector = Selector::parse(selector_str).unwrap();
  let element = elm.select(&selector).next();

  let sl = Selector::parse("span").unwrap();
  let default_elmRef = Html::parse_fragment("<span></span>").select(&sl).next().unwrap();

  let elmRef = element.unwrap_or(default_elmRef);
  elmRef
}

pub fn get_json<T>(url: &str) -> T
where
  T: DeserializeOwned
{
  reqwest::blocking::get(url).unwrap().json().unwrap()
}

pub fn get_weather(local: &str) -> Weather{
  let url = "https://tianqi.moji.com/weather/china/".to_string() + local;
  let html = get_html(&url);
  let root_html = html.root_element();
  let wrap_dom = get_element(root_html, ".wea_info .left");
  let humidity_content = get_element(wrap_dom, ".wea_about span").inner_html();
  let humidity_desc = humidity_content.split(" ").collect::<Vec<&str>>()[1].to_string();
  let mut humidity = "未知".to_string();
  if humidity_desc != "" {
    humidity = humidity_desc
  }

  let limit_desc = get_element(wrap_dom, ".wea_about b").inner_html();

  let mut limit = "".to_string();
  if limit_desc.len() <= 4{
    limit = limit_desc;
  }else{
    limit = limit_desc[4..].to_string();
  }
  Weather{
    city: get_element(root_html, "#search .search_default em").inner_html().to_string(),
    temp: get_element(wrap_dom, ".wea_weather em").inner_html().to_string(),
    weather: get_element(wrap_dom, ".wea_weather b").inner_html().to_string(),
    air: get_element(wrap_dom, ".wea_alert em").inner_html().to_string(),
    humidity,
    wind: get_element(wrap_dom, ".wea_about em").inner_html().to_string(),
    limit,
    note: get_element(wrap_dom, ".wea_about em").inner_html().to_string().replace("。", "")
  }
}

pub fn get_one() -> One {
  let html = get_html("http://wufazhuce.com/");
  let root_html = html.root_element();
  let wrap_dom = get_element(root_html, ".fp-one .carousel .item.active");
  let day = get_element(wrap_dom, ".dom").inner_html();
  let month_year = get_element(wrap_dom, ".may").inner_html();
  let img = get_element(wrap_dom, ".fp-one-imagen").value().attr("src").unwrap().to_string();
  let sentence = get_element(wrap_dom, ".fp-one-cita a").inner_html();

  One{
    date: day,
    img_url: img,
    sentence: sentence
  }
}

pub fn get_english() -> English {
  let html = get_html("http://dict.eudic.net/home/dailysentence/");
  let root_html = html.root_element();
  let wrap_dom = get_element(root_html, ".containter .head-img");
  let img = get_element(wrap_dom, ".himg").value().attr("src").unwrap().to_string();
  let sentence = get_element(wrap_dom, ".sentence .sect_en").inner_html();

  English{
    img_url: img,
    sentence: sentence
  }
}

#[derive(Deserialize, Debug)]
struct PoemResData{
  origin: Poem
}

#[derive(Deserialize, Debug)]
struct PoemRes{
  status: String,
  data: PoemResData
}

pub fn get_poem() -> Poem {
  let res: PoemRes = get_json::<PoemRes>("https://v2.jinrishici.com/one.json");
  println!("poem {:#?}", res);
  res.data.origin
}

pub fn get_wallpaper() -> Wallpaper {
  let url = "https://cn.bing.com/";
  let html = get_html(url);
  let root_html = html.root_element();
  let img_path = get_element(root_html, "#bgLink").value().attr("href").unwrap();
  let title = get_element(root_html, "#sh_cp").value().attr("title").unwrap().to_string();

  Wallpaper{
    title,
    img_url: url.to_owned() + img_path
  }
}

pub fn get_trivia() -> Trivia {
  let html = get_html("http://www.lengdou.net/random");
  let root_html = html.root_element();
  let wrap_dom = get_element(root_html, ".container .media .media-body");
  let img = get_element(wrap_dom, ".topic-img img").value().attr("src").unwrap().to_string();
  let description = get_element(wrap_dom, ".topic-content").inner_html().split("#").collect::<Vec<&str>>()[0].to_string();

  Trivia{
    description,
    img_url: img
  }
}