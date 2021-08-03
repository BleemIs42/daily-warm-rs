use handlebars::Handlebars;
use serde::{ Serialize, Deserialize };
#[derive(Serialize, Deserialize, Debug)]
pub struct Weather{
  pub city     :String,
  pub temp     :String,
  pub weather  :String,
  pub air      :String,
  pub humidity :String,
  pub wind     :String,
  pub limit    :String,
  pub note     :String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct One{
  pub date     :String,
  pub img_url   :String,
  pub sentence :String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct English {
  pub img_url   :String,
  pub sentence :String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Poem {
  pub title   :String,
  pub dynasty :String,
  pub author  :String,
  pub content :Vec<String>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Wallpaper{
  pub title   :String,
  pub img_url  :String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Trivia  {
	pub img_url      :String,
	pub description :String,
}


#[derive(Serialize, Deserialize, Debug)]
struct CombinedData{
  pub weather: Weather,
  // pub one: One,
  // pub english: English,
  // pub poem: Poem,
  // pub wallpaper: Wallpaper,
  // pub trivia: Trivia,
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
{{#with one}}
        <h3 >{{date}}</h3>
{{/with}}
{{#with weather}}
        <h3 style='text-align: center'>{{city}}</h3>
        <div style='text-align: center;font-size: 30px;'>❣️</div>
        <br>
        <div style='padding: 0;width: 100%;'>
          <div><span style='color: #6e6e6e'>天气：</span>{{weather}}</div>
          <div><span style='color: #6e6e6e'>温度：</span>{{temp}}</div>
          <div><span style='color: #6e6e6e'>湿度：</span>{{humidity}}</div>
          <div><span style='color: #6e6e6e'>风向：</span>{{wind}}</div>
          <div><span style='color: #6e6e6e'>空气：</span>{{air}}</div>
          <div><span style='color: #6e6e6e'>限行：</span>{{limit}}</div>
          <div><span style='color: #6e6e6e'>提示：</span>{{note}}</div>
        </div>
{{/with}}
        <br>
        <div style='text-align: center;font-size: 30px;'>📝</div>
        <br>
{{#with english}}
        <div> 
          <div><img width='100%' src='{{img_url}}'></div>
          <div style='margin-top: 10px;line-height: 1.5'>&emsp;&emsp;{{sentence}}</div>
          </div>
{{/with}}
        <br>
        <div style='text-align: center;font-size: 30px;'>📖</div>
        <br>
{{#with Poem}}
        <div style='text-align: center'>
          <div>{{title}}</div>
          <div style='font-size: 12px'>{{dynasty}} {{author}}</div>
          <br>
          <div>{{content}}</div>
        </div>
{{/with}}
        <br>
        <div style='text-align: center;font-size: 30px;'>🔔</div>
        <br>
{{#with one}}
        <div>
          <div><img width='100%' src='{{img_url}}'></div>
          <div style='margin-top: 10px;line-height: 1.5'>&emsp;&emsp;{{sentence}}</div>
        </div>
{{/with}}
        <br>
        <div style='text-align: center;font-size: 30px;'>🏞</div>
        <br>
{{#with wallpaper}}
        <div>
          <div><img width='100%' src='{{img_url}}'></div>
          <div style='margin-top: 10px;line-height: 1.5;text-align: center;'>{{title}}</div>
        </div>
{{/with}}
        <br>
        <div style='text-align: center;font-size: 30px;'>📚</div>
        <br>
{{#with trivia}}
        <div>
          <div><img width='100%' src='{{img_url}}'></div>
          <div style='margin-top: 10px;line-height: 1.5'>&emsp;&emsp;{{description}}</div>
        </div>
{{/with}}
      </div>
      <br><br>
    </body>
    </html>
    ".to_string()
  }
}
