use crate::{
    mail::{CombinedData, English, One, Poem, Trivia, Wallpaper, Weather},
    Config,
};
use reqwest;
use scraper::{ElementRef, Html, Selector};
use serde::{de::DeserializeOwned, Deserialize};

pub fn get_html(url: &str) -> Html {
    let html_str = reqwest::blocking::get(url).unwrap().text().unwrap();
    Html::parse_document(html_str.as_str())
}

fn get_element<'a, 'b>(elm: ElementRef<'a>, selector_str: &'b str) -> ElementRef<'a> {
    let selector = Selector::parse(selector_str).unwrap();
    elm.select(&selector).next().unwrap()
}

pub fn get_json<T>(url: &str) -> T
where
    T: DeserializeOwned,
{
    reqwest::blocking::get(url).unwrap().json().unwrap()
}

pub fn get_weather(local: &str) -> Weather {
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

    Weather {
        city: get_element(root_html, "#search .search_default em")
            .inner_html()
            .to_string(),
        temp: get_element(wrap_dom, ".wea_weather em")
            .inner_html()
            .to_string(),
        weather: get_element(wrap_dom, ".wea_weather b")
            .inner_html()
            .to_string(),
        air: get_element(wrap_dom, ".wea_alert em")
            .inner_html()
            .to_string(),
        humidity,
        wind: get_element(wrap_dom, ".wea_about em")
            .inner_html()
            .to_string(),
        limit: "".to_string(),
        note: get_element(wrap_dom, ".wea_about em")
            .inner_html()
            .to_string()
            .replace("。", ""),
    }
}

pub fn get_one() -> One {
    let html = get_html("http://wufazhuce.com/");
    let root_html = html.root_element();
    let wrap_dom = get_element(root_html, ".fp-one .carousel .item.active");
    let day = get_element(wrap_dom, ".dom").inner_html();
    let month_year = get_element(wrap_dom, ".may").inner_html();
    let img = get_element(wrap_dom, ".fp-one-imagen")
        .value()
        .attr("src")
        .unwrap()
        .to_string();
    let sentence = get_element(wrap_dom, ".fp-one-cita a").inner_html();

    One {
        date: format!("{} {}", day, month_year),
        img_url: img,
        sentence: sentence,
    }
}

pub fn get_english() -> English {
    let html = get_html("http://dict.eudic.net/home/dailysentence/");
    let root_html = html.root_element();
    let wrap_dom = get_element(root_html, ".containter .head-img");
    let img = get_element(wrap_dom, ".himg")
        .value()
        .attr("src")
        .unwrap()
        .to_string();
    let sentence = get_element(wrap_dom, ".sentence .sect_en").inner_html();

    English {
        img_url: img,
        sentence: sentence,
    }
}

#[derive(Deserialize, Debug, Clone)]
struct PoemResData {
    origin: PoemJson,
}

#[derive(Deserialize, Debug, Clone)]
struct PoemJson {
    title: String,
    dynasty: String,
    author: String,
    content: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct PoemRes {
    status: String,
    data: PoemResData,
}

pub fn get_poem() -> Poem {
    let poem_json: PoemJson = get_json::<PoemRes>("https://v2.jinrishici.com/one.json")
        .data
        .origin;
    Poem {
        title: poem_json.title,
        content: poem_json.content.join("\r\n"),
        dynasty: poem_json.dynasty,
        author: poem_json.author,
    }
}

pub fn get_wallpaper() -> Wallpaper {
    let url = "https://cn.bing.com/";
    let html = get_html(url);
    let root_html = html.root_element();
    let img_path = get_element(root_html, "#bgLink")
        .value()
        .attr("href")
        .unwrap();
    let title = get_element(root_html, "#sh_cp")
        .value()
        .attr("title")
        .unwrap()
        .to_string();

    Wallpaper {
        title,
        img_url: url.to_owned() + img_path,
    }
}

pub fn get_trivia() -> Trivia {
    let html = get_html("http://www.lengdou.net/random");
    let root_html = html.root_element();
    let wrap_dom = get_element(root_html, ".container .media .media-body");
    let img = get_element(wrap_dom, ".topic-img img")
        .value()
        .attr("src")
        .unwrap()
        .to_string();
    let description = get_element(wrap_dom, ".topic-content")
        .inner_html()
        .split("#")
        .collect::<Vec<&str>>()[0]
        .to_string();

    Trivia {
        description,
        img_url: img,
    }
}

pub fn get_data(config: &Config) -> CombinedData {
    let weather = get_weather(&config.to.local);
    let one = get_one();
    let english = get_english();
    let poem = get_poem();
    let wallpaper = get_wallpaper();
    // let trivia = get_trivia();
    CombinedData {
        weather,
        one,
        english,
        poem,
        wallpaper,
    }
}
