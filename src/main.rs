

#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate multipart;

extern crate time;
extern crate regex;
extern crate hsl;
#[macro_use] extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

mod color;
mod colorhsl;
mod sort_hsl;

mod page;
use page::*;
use colorhsl::*;

fn sort_list(v: &Vec<ColorHsl>) -> Vec<ColorHsl> {
    Vec::new()
}

#[get("/")]
fn index() -> String {
    let params = Page::default();
    let mut output = String::new();
    let list: Vec<ColorHsl> = vec![
        ColorHsl::from_hex("#65d1fa", "#65d1fa").unwrap(),
        ColorHsl::from_hex("#ad5bff", "#ad5bff").unwrap(),
        ColorHsl::from_hex("#fc66d2", "#fc66d2").unwrap(),
        ColorHsl::from_hex("#df0028", "#df0028").unwrap(),
        ColorHsl::from_hex("#ff7700", "#ff7700").unwrap(),
        ColorHsl::from_hex("#ac6c00", "#ac6c00").unwrap(),
        ColorHsl::from_hex("#ffe467", "#ffe467").unwrap(),
        ColorHsl::from_hex("#e3fe00", "#e3fe00").unwrap(),
        ColorHsl::from_hex("#a2fb00", "#a2fb00").unwrap(),
        ColorHsl::from_hex("#65faf4", "#65faf4").unwrap(),
        ColorHsl::from_hex("#0b6aff", "#0b6aff").unwrap(),
    ];
    output.push_str(&header());
    output.push_str(&form(&params));
    output.push_str(&body(&list));
    output.push_str(&footer());
    output
}

#[post("/", data = "<params>")]
fn findex(params: Page) -> String {
    let mut output = String::new();
    let list: Vec<ColorHsl> = vec![
        ColorHsl::from_hex("#65d1fa", "#65d1fa").unwrap(),
        ColorHsl::from_hex("#ad5bff", "#ad5bff").unwrap(),
        ColorHsl::from_hex("#fc66d2", "#fc66d2").unwrap(),
        ColorHsl::from_hex("#df0028", "#df0028").unwrap(),
        ColorHsl::from_hex("#ff7700", "#ff7700").unwrap(),
        ColorHsl::from_hex("#ac6c00", "#ac6c00").unwrap(),
        ColorHsl::from_hex("#ffe467", "#ffe467").unwrap(),
        ColorHsl::from_hex("#e3fe00", "#e3fe00").unwrap(),
        ColorHsl::from_hex("#a2fb00", "#a2fb00").unwrap(),
        ColorHsl::from_hex("#65faf4", "#65faf4").unwrap(),
        ColorHsl::from_hex("#0b6aff", "#0b6aff").unwrap(),
    ];
    output.push_str(&header());
    output.push_str(&form(&params));
    output.push_str(&body(&list));
    output.push_str(&footer());
    output
}


fn main() {
    // println!("Hello, world!");
    // rocket::ignite().mount("/", routes![findex, index]).launch();
    
    rocket::ignite().mount("/", routes![index]).launch();
}
