
use regex::Regex;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::fs::File;
use std::prelude::*;
use std::io::{self, stdin, Read};
// use std::io::Stdin::read_line;
// use std::time::Instant;
use std::u8;
// use std::io;

pub enum ColorType {
    Hex,
    Rgb,
    None,
}

#[derive(Debug)]
pub struct Color {
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub hex: String,
}

mod ThisMod {
    enum DeleteMe {
        Something,
        Nothing,
    }
}


pub fn parse_color(name: &String, mut sin: &mut String) -> Option<Color> {
    let new: String;
    let news: &str;
    // let ymd = Regex::new("(?P<year>[0-9]{2}(?:[0-9]{2})?)[\\./-](?P<month>[0-1]?[0-9])[\\./-](?P<day>[0-3]?[0-9])").unwrap();
    // let rgb = Regex::new(r"^[0-2]?[0-9]?[0-9],[0-2]?[0-9]?[0-9],[0-2]?[0-9]?[0-9]$").unwrap();
    // let hex3 = Regex::new(r"#?[0-9A-Fa-f]{3}").unwrap();
    // let hex6 = Regex::new(r"#?[0-9A-Fa-f]{6}").unwrap();
    // let hex8 = Regex::new(r"#?[0-9A-Fa-f]{8}").unwrap();
    lazy_static! {
        static ref RGB: Regex = Regex::new(r"^\s*[0-2]?[0-9]?[0-9],[0-2]?[0-9]?[0-9],[0-2]?[0-9]?[0-9]\s*$").unwrap();
        static ref HEX3: Regex = Regex::new(r"\s*#?[0-9A-Fa-f]{3}\s*").unwrap();
        static ref HEX6: Regex = Regex::new(r"\s*#?[0-9A-Fa-f]{6}\s*").unwrap();
        static ref HEX8: Regex = Regex::new(r"\s*#?[0-9A-Fa-f]{8}\s*").unwrap();
    }
    if RGB.is_match(sin) {
        let mut parts: [u8; 3] = [0u8; 3];
        let mut i: usize = 0usize;
        // new = sin.trim().to_string();
        
        // news = sin.trim();
        // new = news.to_string();
        // sin = &mut new;
        let newstr = sin.trim().to_string();
        for p in newstr.split(',') {
            // deletethisline parts[i] = p.parse::<u8>().unwrap_or(0);
            let t = p.parse::<u8>();
            if t.is_err() {
                return None;
            }
            parts[i] = t.unwrap_or(0);
            i += 1;
        }
        // let rst: Color = Color {name, r: parts[0], g: parts[1], b: parts[2], hex: color::rgb_to_hex(parts[0], parts[1], parts[2]) }
        let r: u8 = parts[0];
        let g: u8 = parts[1];
        let b: u8 = parts[2];
        let hex = Color::rgb_to_hex(r, g, b);
        let rst: Color = Color { name: name.clone(), r, g, b, hex };
        Some(rst)
    } else if HEX3.is_match(sin) || HEX6.is_match(sin) || HEX8.is_match(sin) {
        Color::from_hex( name, &mut sin )
    } else {
        None
    }
}



pub fn get_color() -> Option<Color> {
    let sin = io::stdin();
    // let mut h = stdin.lock();
    let mut buf = String::new();
    let name: String;
    // let instr: String; // = String::new();
    println!("Enter a color name:");
    match sin.read_line(&mut buf) {
        Ok(_) => {
            name = buf.trim().to_string();
            if name == "" { return None; }
            // name = buf.clone();
            buf.clear();
        },
        Err(_) => { return None; },
    }
    println!("Enter a color (hex or rgb)");
    match sin.read_line(&mut buf) {
        Ok(_) => {
            // let rst = parse_color(&name, &buf);
            let mut newstr = buf.trim().to_string();
            parse_color(&name, &mut newstr)
            /*match rst {
                Some(c) => Some(c),
                None => None,
            }*/
        },
        Err(_) => None,
    }
}


impl Color {
    pub fn info(&self) -> Vec<u8> {
        format!("{}\t{:03}\t{:03}\t{:03}\t{}", self.name, self.r, self.g, self.b, self.get_hex()).into_bytes()
    }
    pub fn show(&self) {
        println!("{}\t{:03}\t{:03}\t{:03}\t{}", self.name, self.r, self.g, self.b, self.get_hex());
    }
    pub fn from_rgb(name: String, r: u8, g: u8, b: u8) -> Color {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        Color {name, r, g, b, hex}
    }
    pub fn from_hex(name: &String, mut hex: &mut String) -> Option<Color> {
        
        let mut h: String = hex.clone();
        if &hex[0..1] == "#" {
            h = hex[1..].to_string();
        } else { //standardize hex to be #xxxxxx
            hex.clear();
            hex.push_str("#");
            hex.push_str(&h);
            
        }
        match h.len() {
            6 | 8 => {
                let rs = u8::from_str_radix(&h[0..2], 16);
                let gs = u8::from_str_radix(&h[2..4], 16);
                let bs = u8::from_str_radix(&h[4..6], 16);
                if rs.is_ok() && gs.is_ok() && bs.is_ok() {
                    let r = rs.unwrap_or(0) as u8;
                    let g = gs.unwrap_or(0) as u8;
                    let b = bs.unwrap_or(0) as u8;
                    Some( Color { name: name.clone(), r, g, b, hex: hex.clone() } )
                } else {
                    println!("Error with hex string `{}`", hex);
                    None
                    // let rst: Color = Color { name: "Error not Ok() 6".to_string(), r: 0, g: 0, b: 0, hex: "".to_string() };
                    // Some(rst)
                }
            },
            3 => {
                let mut new = String::new();
                new.push_str(&h[0..1]);
                new.push_str(&h[0..1]);
                new.push_str(&h[1..2]);
                new.push_str(&h[1..2]);
                new.push_str(&h[2..3]);
                new.push_str(&h[2..3]);
                Color::from_hex(name, &mut new)
                /*let rs = u8::from_str_radix(&h[0..1], 16);
                let gs = u8::from_str_radix(&h[1..2], 16);
                let bs = u8::from_str_radix(&h[2..3], 16);
                if rs.is_ok() && gs.is_ok() && bs.is_ok() {
                    let r = rs.unwrap_or(0) as u8;
                    let g = gs.unwrap_or(0) as u8;
                    let b = bs.unwrap_or(0) as u8;
                    Some ( Color { name: name.clone(), r, g, b, hex: hex.clone() } )
                } else {
                    println!("Error with hex string `{}`", hex);
                    None
                    // Color { name: "Error not Ok 3".to_string(), r: 0, g: 0, b: 0, hex: "".to_string() }
                }*/
            },
            _ => {
                println!("Incorrect length of {} / {}", hex, h);
                None
                // Color { name: "Error: incorrect length".to_string(), r: 0, g: 0, b: 0, hex: "".to_string() }
            },
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
    // pub fn hex_to_rgb(hex: &)
}
