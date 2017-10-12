
// to supress warnings use:
// set RUSTFLAGS=-Awarnings

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
// use page::DEFAULT_SORT;
use colorhsl::*;

use regex::Regex;
use std::str;
use std::io;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use multipart::server::Multipart;

// contains content::Html<String> which can be used as a return type
use rocket::response::{content, NamedFile};
use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
// use rocket::Data;
// use rocket::response::NamedFile;

#[derive(Debug)]
pub struct ColorFile {
    pub colr_upld: Vec<u8>,
}

#[derive(Debug)]
pub struct FileColors {
    pub colors: Vec<ColorHsl>,
}

impl FromData for FileColors {
    type Error = ();
    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let ct = request.headers().get_one("Content-Type").expect("no content-type");
        let idx = ct.find("boundary=").expect("no boundary");
        let boundary = &ct[(idx + "boundary=".len())..];

        let mut dat: Vec<u8> = Vec::new();
        data.stream_to(&mut dat).expect("Unable to read");
        
        let mut mp = Multipart::with_body(Cursor::new(dat), boundary);
        
        // Custom implementation parts
        
        let mut alldata: Vec<u8> = Vec::new();
        let mut file: Option<Vec<u8>> = None;
        
        // for part in mp.into_entry() {
        // while let Ok(partraw) = mp.read_entry() {
        //     if let Some(mut part) = partraw {
        //         // let mut part = partraw.expect("Could not unwrap raw part");
                
        //         let mut d: Vec<u8> = Vec::new();
        //         let f = part.data.as_file().expect("Could not read multipart form file as data");
        //         f.read_to_end(&mut d).expect("Could not read upload file data");
        //         // alldata.extend(part.data);
        //         alldata.extend(d);
                
        //         // if let MultipartData::File(filedata) = part.data {
        //     }
        // }
        
        
        mp.foreach_entry(|mut entry| {
            if entry.name.as_str() == "colr_upld" {
                let mut d: Vec<u8> = Vec::new();
                let f = entry.data.as_file().expect("Could not open file!");
                f.read_to_end(&mut d).expect("Could not read all form data!");
                if d.len() != 0 {
                    alldata.append(&mut d);
                    println!("File {} was parsed.", entry.name);
                } else {
                    println!("File {} could not be parsed.", entry.name);
                }
            } else {
                println!("Unexpected entry: {}", entry.name.as_str());
            }
            
        });//.expect("Could not parse form.");
        
        lazy_static! {
            static ref HEXES: Regex = Regex::new(r#"#([A-Fa-f0-9]){6}"#).unwrap();
        }
        
        let s = str::from_utf8(&alldata).expect("Could not convert form data to valid utf8");
        let mut list: Vec<ColorHsl> = Vec::new();
        
        for colr in HEXES.captures_iter(s) {
            let col = &colr[0];
            if let Some(color) = ColorHsl::from_hex(col, col) {
                list.push(color);
            }
        }
        
        let v: FileColors = FileColors {
            colors: list,
        };
        Outcome::Success(v)
        
    }
}

/*
impl FromData for ColorFile {
    type Error = ();
    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let ct = request.headers().get_one("Content-Type").expect("no content-type");
        let idx = ct.find("boundary=").expect("no boundary");
        let boundary = &ct[(idx + "boundary=".len())..];

        let mut dat: Vec<u8> = Vec::new();
        data.stream_to(&mut dat).expect("Unable to read");
        
        let mut mp = Multipart::with_body(Cursor::new(dat), boundary);
        
        // Custom implementation parts
        
        let mut alldata: Vec<u8> = Vec::new();
        let mut file: Option<Vec<u8>> = None;
        
        // for part in mp.into_entry() {
        while let Some(part) = mp.read_entry() {
            let mut d: Vec<u8> = Vec::new();
            let f = entry.data.as_file().expect("Could not read multipart form file as data");
            f.read_to_end(&mut d).expect("Could not read upload file data");
            alldata.extend(part.data);
            
            // if let MultipartData::File(filedata) = part.data {
        }

        
        // mp.foreach_entry(|mut entry| {
        //     match entry.name.as_str() {
        //         "colr_upld" => {
        //             let mut d = Vec::new();
        //             let f = entry.data.as_file().expect("Data is not a file.");
        //             f.read_to_end(&mut d).expect("Could not read upload file data");
        //             if file == None {
        //                 file = Some(d);
        //             } else {
        //                 let mut cur = file.unwrap();
        //                 cur.extend(d);
        //             }
        //         },
        //         o => { println!("Encountered foreign form field: {}", o); },
        //     }
        // }).expect("Could not parse form.");
        
        
        let v = ColorFile {
            colr_upld: file.expect("No File found."),
        };
        Outcome::Success(v)
    }
}
*/

type Html = content::Html<String>;

fn sort_list(v: &Vec<ColorHsl>) -> Vec<ColorHsl> {
    Vec::new()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// #[post("/upload", format = "", data = "<data>")]

// #[post("/upload", data = "<data>")]
// fn upload(data: Data) -> Html {
    
// #[post("/upload", format = "multipart/form-data", data = "<data>")]
#[post("/upload", data = "<data>")]
fn upload(data: FileColors) -> Html {
    let list = data.colors;
    let params = Page::default();
    // let mut sorted_list = po.sort.sort(&list, po.reverse);
    let mut sorted_list = DEFAULT_SORT.sort(&list, false);
    sorted_list.dedup();
    
    let mut output: String = String::new();
    
    output.push_str(&header());
    output.push_str(&form(&params));
    output.push_str(&body(&sorted_list));
    output.push_str(&footer());
    
    content::Html(output)
    
}

/*
#[post("/upload", format = "multipart/form-data", data = "<data>")]
fn upload(data: FileColors) -> Html {
    
    // let s = match str::from_utf8(buf) {
    //     Ok(v) => v,
    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };
    
    // println!("\nData:\n{:?}", data.colr_upld);
    // content::Html(str::from_utf8(&data.colr_upld).expect("Could not read form data as valid utf8").to_string())
    
    let s = str::from_utf8(&data.colr_upld).expect("Could not read form data as valid utf8");
    
    lazy_static! {
        static ref HEXES: Regex = Regex::new(r#"#([A-Fa-f0-9]){6}|([A-Fa-f0-9]){3}[^A-Fa-f0-9]"#).unwrap();
    }
    let mut output = String::new();
    let mut list: Vec<ColorHsl> = Vec::new();
    let params: Page = Page::default();
    
    // for colr in HEXES.find_iter(s) {
    for colr in HEXES.captures_iter(s) {
        // let col = format!("{}", colr);
        let col = &colr[0];
        if let Some(color) = ColorHsl::from_hex(col, col) {
            list.push(color);
        }
        // match ColorHsl::from_hex(colr, colr) {
        //     Some(color) => {},
        //     None => {}
        // }
    }
    
    output.push_str(&header());
    output.push_str(&form(&params));
    output.push_str(&body(&list));
    output.push_str(&footer());
    
    content::Html(output)
    
    
    // let person_ct = ContentType::new("application", "x-person");
    // if req.content_type() != Some(&person_ct) {
    //     return Outcome::Forward(data);
    // }

    // data.stream_to_file("upload_data.txt");
    
    
    // Read the data into a String.
    // let mut string = String::new();
    // if let Err(e) = data.open().read_to_string(&mut string) {
    //     // return Failure((Status::InternalServerError, format!("{:?}", e))a);
    //     return content::Html(String::from("Could not open uploaded file."));
    // }
    // content::Html(string)
    
    
    // println!("\nData:\n{:?}\n", data);
    // data.stream_to_file("upload_data.txt").map(|n| n.to_string());
    // content::Html(string)
    // content::Html(String::from("Uploaded file successfully."))
    
}
*/



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
    
    let mut sorted_list = po.sort.sort(&list, po.reverse);
    
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
    rocket::ignite().mount("/", routes![index, files, findex, upload]).launch();
}
