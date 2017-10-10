
// #![feature(const_unsafe_cell_new)]
// #![feature(const_atomic_bool_new)]

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

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::content;
use rocket::response::NamedFile;

type Html = content::Html<String>;

fn sort_list(v: &Vec<ColorHsl>) -> Vec<ColorHsl> {
    Vec::new()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
fn index() -> Html {
    let params = Page::default();
    let mut output = String::new();
    let list: Vec<ColorHsl> = vec![
        // ColorHsl::from_hex("#65d1fa", "#65d1fa").unwrap(),
        // ColorHsl::from_hex("#ad5bff", "#ad5bff").unwrap(),
        // ColorHsl::from_hex("#fc66d2", "#fc66d2").unwrap(),
        // ColorHsl::from_hex("#df0028", "#df0028").unwrap(),
        // ColorHsl::from_hex("#ff7700", "#ff7700").unwrap(),
        // ColorHsl::from_hex("#ac6c00", "#ac6c00").unwrap(),
        // ColorHsl::from_hex("#ffe467", "#ffe467").unwrap(),
        // ColorHsl::from_hex("#e3fe00", "#e3fe00").unwrap(),
        // ColorHsl::from_hex("#a2fb00", "#a2fb00").unwrap(),
        // ColorHsl::from_hex("#65faf4", "#65faf4").unwrap(),
        // ColorHsl::from_hex("#0b6aff", "#0b6aff").unwrap(),
    ];
    output.push_str(&header());
    output.push_str(&form(&params));
    output.push_str(&body(&list));
    output.push_str(&footer());
    
    content::Html(output)
}

#[post("/", data = "<params>")]
fn findex(params: Page) -> Html {
    let mut output = String::new();
    
    let mut list: Vec<ColorHsl> = vec![
        // ColorHsl::from_hex("#65d1fa", "#65d1fa").unwrap(),
        // ColorHsl::from_hex("#ad5bff", "#ad5bff").unwrap(),
        // ColorHsl::from_hex("#fc66d2", "#fc66d2").unwrap(),
        // ColorHsl::from_hex("#df0028", "#df0028").unwrap(),
        // ColorHsl::from_hex("#ff7700", "#ff7700").unwrap(),
        // ColorHsl::from_hex("#ac6c00", "#ac6c00").unwrap(),
        // ColorHsl::from_hex("#ffe467", "#ffe467").unwrap(),
        // ColorHsl::from_hex("#e3fe00", "#e3fe00").unwrap(),
        // ColorHsl::from_hex("#a2fb00", "#a2fb00").unwrap(),
        // ColorHsl::from_hex("#65faf4", "#65faf4").unwrap(),
        // ColorHsl::from_hex("#0b6aff", "#0b6aff").unwrap(),
    ];
    // if let Some(addcolr) = params.add {
        // list.push(addcolr);
    
    
    let po = params.to_owned();
    if po.persist.len() != 0 {
        // list.extend_from_slice(po.persist.as_slice());
        let new: Vec<ColorHsl> = po.persist.iter().filter_map(|ref x| ColorHsl::from_hex(&x, &x)).collect();
        list.extend_from_slice(new.as_slice());
    }
    if po.add.is_some() {
        list.push(po.add.unwrap());
        // let newadd = (&params).add.unwrap().clone();
        // list.push(newadd);
    }
    if po.adds.is_some() {
        list.extend_from_slice(po.adds.unwrap().as_slice());
    }
    // list.sort();
    
    let mut sorted_list = po.sort.sort(&list);
    sorted_list.dedup();
    
    
    // if pc.add.is_some() {
    //     let orig 
    //     let Some(pc) = params.add;
    //     list.push(pc);
    // }
    
    // TODO: Sort by specified sort method
    
    output.push_str(&header());
    output.push_str(&form(&params));
    output.push_str(&body(&sorted_list));
    output.push_str(&footer());

    content::Html(output)
}


fn main() {
    rocket::ignite().mount("/", routes![index, files, findex]).launch();
}
