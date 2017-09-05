
// use regex::Regex;
use std::collections::HashMap;
use std::io::{BufReader, BufRead, BufWriter, Write, Read};
use std::fs::File;
use std::prelude::*;
// use std::io::{self, stdin, Read};
// use std::io::Stdin::read_line;
// use std::time::Instant;
// use std::u8;
use hsl::HSL;
use color::*;
use std::cmp::Ordering;

use std::fmt;
use std::str::FromStr;
use std::result::Result;
use rocket::request::FromFormValue;
use rocket::request::FromParam;
use rocket::http::RawStr;
use page::color_template;

extern crate serde;
extern crate serde_json;
use self::serde_json::Error;
// use std::io;

// mod hsl_soert;
// use hsl_sort::*;

/*pub enum ColorType {
    Hex,
    Rgb,
    Hsl,
    None,
}*/

// #[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorHsl {
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub hex: String,
}

pub fn sort_h(lhs: ColorHsl, rhs: ColorHsl) -> Ordering {
    lhs.h.partial_cmp(&rhs.h).unwrap_or(Ordering::Equal)
}
pub fn sort_s(lhs: ColorHsl, rhs: ColorHsl) -> Ordering {
    lhs.s.partial_cmp(&rhs.s).unwrap_or(Ordering::Equal)
}
pub fn sort_l(lhs: ColorHsl, rhs: ColorHsl) -> Ordering {
    lhs.l.partial_cmp(&rhs.l).unwrap_or(Ordering::Equal)
}

impl PartialEq for ColorHsl {
    fn eq(&self, other: &ColorHsl) -> bool {
           self.hex == other.hex
        
           // self.r == other.r 
        // && self.g == other.g 
        // && self.b == other.b
         
        // && self.h == other.h 
        // && self.s == other.s 
        // && self.l == other.l 
    }
}

impl <'v> FromFormValue<'v> for ColorHsl {
    type Error = &'v RawStr;
    fn from_form_value(form_value: &'v RawStr) -> Result<ColorHsl, &'v RawStr> {
        match ColorHsl::from_hex(form_value, form_value) {
            None => Err(form_value),
            Some(c) => Ok(c),
        }
    }
}

impl fmt::Display for ColorHsl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", color_template(self))
    }
}

impl ColorHsl {
    pub fn clone_sort(colors: &HashMap<String, ColorHsl>) -> Vec<ColorHsl> {
        // let mut v: Vec<ColorHsl> = colors.values().collect();
        
        let mut v: Vec<ColorHsl> = Vec::new();
        for (key, value) in colors {
            v.push(value.clone());
        }
        
        v.sort_by(|a, b| a.h.partial_cmp(&b.h).unwrap_or(Ordering::Equal));
        v
    }
    
    pub fn sort_by<F>(colors: &HashMap<String, ColorHsl>, sorter: F) -> Vec<ColorHsl>
        where F: Fn(&ColorHsl, &ColorHsl) -> Ordering
    {
        let mut v: Vec<ColorHsl> = Vec::new();
        for (key, value) in colors {
            v.push(value.clone());
        }
        // maybe take out the closure and replace with just the function?
        v.sort_by(|ref a, ref b| sorter(&a, &b));
        v
    }
    
    pub fn sort_by2<F>(colors: &HashMap<String, ColorHsl>, mut sorter: F) -> Vec<ColorHsl>
        where F: FnMut(&ColorHsl, &ColorHsl) -> Ordering
    {
        let mut v: Vec<ColorHsl> = Vec::new();
        for (key, value) in colors {
            v.push(value.clone());
        }
        // maybe take out the closure and replace with just the function?
        
        v.sort_by(|ref a, ref b| sorter(&a, &b));
        v
    }
    
    pub fn info_str(&self) -> String {
        // format!("{}\t{:03}\t{:03}\t{:03}\t{}\t{}\t{}\t{}"
        format!("{}\t{:03}\t{:03}\t{:03}\t{:.6}\t{:.6}\t{:.6}\t{}"
            , self.name, self.r, self.g, self.b, self.h, self.s, self.l, self.hex)
    }
    pub fn info(&self) -> Vec<u8> {
        // format!("{}\t{:03}\t{:03}\t{:03}\t{}\t{}\t{}\t{}"
        format!("{}\t{:03}\t{:03}\t{:03}\t{:.6}\t{:.6}\t{:.6}\t{}"
            , self.name, self.r, self.g, self.b, self.h, self.s, self.l, self.hex).into_bytes()
    }
    pub fn show(&self) {
        println!("{}\t{:03}\t{:03}\t{:03}\t{:<8}\t{:.9}\t{:.9}\t{}"
            , self.name, self.r, self.g, self.b, self.h, self.s, self.l, self.hex);
    }
    pub fn from_rgb(name: &str, r: u8, g: u8, b: u8) -> ColorHsl {
        let c = HSL::from_rgb(&[r, g, b]);
        let h = c.h;
        let s = c.s;
        let l = c.l;
        let hex = Color::rgb_to_hex(r, g, b);
        
        ColorHsl { name: name.to_string(), r, g, b, h, s, l, hex }
    }
    pub fn from_color(c: &Color) -> ColorHsl {
        let color = HSL::from_rgb(&[c.r, c.g, c.b]);
        /*let h = color.h;
        let s = color.s;
        let l = color.l;
        // let hex = c.hex.clone();
        let hex = c.hex;
        let name = c.name;*/
        ColorHsl {
            name: c.name.clone(), r: c.r, g: c.g, b: c.b, h: color.h, s: color.s, l: color.l, hex: c.hex.clone()
            // name, r: c.r, g: c.g, b: c.b, h, s, l, hex
            // maybe try h, s, l ..c
        }
    }
    pub fn from_hex(name: &str, mut hex: &str) -> Option<ColorHsl> {
        let c = Color::from_hex(&name.to_string(), &mut hex.to_string());
        match c {
            Some(ref a) =>
                Some(Self::from_color(a)),
            None => None,
        }
    }
    pub fn show_hex(&self) {
        print!("{}", self.get_hex());
    }
    pub fn get_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
    pub fn get_rgb(&self) -> String {
        format!("{:03},{:03},{:03}", self.r, self.g, self.b)
    }
    pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }
    
    pub fn write_json(jsonfile: &str, list: &Vec<ColorHsl>) {
        let mut j = BufWriter::new(File::create(jsonfile).expect("Could not create json file."));
        let json_colors = serde_json::to_vec(list).unwrap();
        
        j.write(&json_colors);
    }
    pub fn read_json(jsonfile: &str) -> Vec<ColorHsl> {
        let mut f = File::open(jsonfile).expect("Could not open json file.");
        let mut contents: Vec<u8> = Vec::new(); 
        f.read_to_end(&mut contents);
        // let list: Vec<ColorHsl> = serde_json::from_slice(&contents).unwrap();
        let list: Vec<ColorHsl> = serde_json::from_slice(&contents).unwrap();
        list
        // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    }
    pub fn read_json_str(jsonstr: &str) -> Vec<ColorHsl> {
        // let mut contents: Vec<u8> = Vec::new();
        // f.read_to_end(&mut contents);
        
        
        let contents = jsonstr.as_bytes();
        // let mut list: Vec<ColorHsl> = Vec::new();
        // let list: Vec<ColorHsl> = serde_json::from_slice(&contents).unwrap();
        let list: Vec<ColorHsl> = serde_json::from_slice(&contents).unwrap_or(Vec::<ColorHsl>::new());
        list
        // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    }
}

