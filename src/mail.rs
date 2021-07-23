use handlebars::Handlebars;
use serde::{ Serialize, Deserialize };
#[derive(Serialize, Deserialize, Debug)]
pub struct Weather{
  City     :String,
  // Temp     :String,
  // Weather  :String,
  // Air      :String,
  // Humidity :String,
  // Wind     :String,
  // Limit    :String,
  // Note     :String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct One{
  Date     :String,
  ImgURL   :String,
  Sentence :String,
}
#[derive(Serialize, Deserialize, Debug)]
struct English {
  ImgURL   :String,
  Sentence :String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Poem {
  Title   :String,
  Dynasty :String,
  Author  :String,
  Content :Vec<String>
}
#[derive(Serialize, Deserialize, Debug)]
struct Wallpaper{
  Title   :String,
  ImgURL  :String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Trivia  {
	ImgURL      :String,
	Description :String,
}


#[derive(Serialize, Deserialize, Debug)]
struct CombinedData{
  Weather: Weather,
  // One: One,
  // English: English,
  // Poem: Poem,
  // Wallpaper: Wallpaper,
  // Trivia: Trivia,
}

pub struct Render{
  data: String
}

impl Render {
  pub fn new(data: String) -> Self{
    Self{
      data: data
    }
  }
  pub fn get_content(&self) -> String{
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("mail_tpl", &self.get_tpl());
    let data = serde_json::from_str::<CombinedData>(self.data.as_str()).unwrap();
    println!("data:   {:?}", data);
    handlebars.render("mail_tpl", &data).unwrap()
    // mail
  }

  pub fn get_tpl(&self)->String{
    "<!DOCTYPE html>
    <html lang='en'>
    <head>
      <meta charset='UTF-8'>
      <meta name='viewport' content='width=device-width, initial-scale=1.0'>
      <meta http-equiv='X-UA-Compatible' content='ie=edge'>
      <title>每日一暖, 温情一生</title>
    </head>
    <body>
      <div style='max-width: 375px; margin: 20px auto;color:#444; font-size: 16px;'>
{{#with One}}
        <h3 >{{Date}}</h3>
{{/with}}
{{#with Weather}}
        <h3 style='text-align: center'>{{City}}</h3>
        <div style='text-align: center;font-size: 30px;'>❣️</div>
        <br>
        <div style='padding: 0;width: 100%;'>
          <div><span style='color: #6e6e6e'>天气：</span>{{Weather}}</div>
          <div><span style='color: #6e6e6e'>温度：</span>{{Temp}}</div>
          <div><span style='color: #6e6e6e'>湿度：</span>{{Humidity}}</div>
          <div><span style='color: #6e6e6e'>风向：</span>{{Wind}}</div>
          <div><span style='color: #6e6e6e'>空气：</span>{{Air}}</div>
          <div><span style='color: #6e6e6e'>限行：</span>{{Limit}}</div>
          <div><span style='color: #6e6e6e'>提示：</span>{{Note}}</div>
        </div>
{{/with}}
        <br>
        <div style='text-align: center;font-size: 30px;'>📝</div>
        <br>
{{#with English}}
        <div> 
          <div><img width='100%' src='{{ImgURL}}'></div>
          <div style='margin-top: 10px;line-height: 1.5'>&emsp;&emsp;{{Sentence}}</div>
          </div>
{{/with}}
        <br>
        <div style='text-align: center;font-size: 30px;'>📖</div>
        <br>
{{#with Poem}}
        <div style='text-align: center'>
          <div>{{Title}}</div>
          <div style='font-size: 12px'>{{Dynasty}} {{Author}}</div>
          <br>
          <div>{{Content}}</div>
        </div>
{{/with}}
        <br>
        <div style='text-align: center;font-size: 30px;'>🔔</div>
        <br>
{{#with One}}
        <div>
          <div><img width='100%' src='{{ImgURL}}'></div>
          <div style='margin-top: 10px;line-height: 1.5'>&emsp;&emsp;{{Sentence}}</div>
        </div>
{{/with}}
        <br>
        <div style='text-align: center;font-size: 30px;'>🏞</div>
        <br>
{{#with Wallpaper}}
        <div>
          <div><img width='100%' src='{{ImgURL}}'></div>
          <div style='margin-top: 10px;line-height: 1.5;text-align: center;'>{{Title}}</div>
        </div>
{{/with}}
        <br>
        <div style='text-align: center;font-size: 30px;'>📚</div>
        <br>
{{#with Trivia}}
        <div>
          <div><img width='100%' src='{{ImgURL}}'></div>
          <div style='margin-top: 10px;line-height: 1.5'>&emsp;&emsp;{{Description}}</div>
        </div>
{{/with}}
      </div>
      <br><br>
    </body>
    </html>
    ".to_string()
  }
}
