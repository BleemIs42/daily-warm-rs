// use std::collections::HashMap;
use reqwest;
use scraper::{ElementRef, Html, Selector};
use crate::mail::{One};

pub fn get_html(url: &str) -> Html{
  let html_str = reqwest::blocking::get(url).unwrap().text().unwrap();
  Html::parse_document(html_str.as_str())
}

fn get_element<'a, 'b>(elm: ElementRef<'a>, selector_str: &'b str) -> ElementRef<'a>{
  let selector = Selector::parse(selector_str).unwrap();
  elm.select(&selector).next().unwrap()
}

pub fn get_one() -> One {
  let html = get_html("http://wufazhuce.com/");
  let root_html = html.root_element();
  let wrap_dom = get_element(root_html, ".fp-one .carousel .item.active");
  let day = get_element(wrap_dom, ".dom").inner_html();
  let month_year = get_element(wrap_dom, ".may").inner_html();
  let img = get_element(wrap_dom, ".fp-one-imagen").value().attr("src").unwrap();
  let sentence = get_element(wrap_dom, ".fp-one-cita a").inner_html();
  println!("day {:?} \n month_year {:?} \n img {:?}  \n sentence {:?}", day, month_year, img, sentence);

  One{
    Date: day,
    ImgURL: img.to_string(),
    Sentence: sentence
  }
}