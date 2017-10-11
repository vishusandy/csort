
use std::io::{Cursor, Read};
use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
use rocket::http::uri::URI;
use multipart::server::Multipart;
use rocket::request::FromParam;
use rocket::http::RawStr;

use std::str;
use std::str::FromStr;
use std::result::Result;
use rocket::request::FromFormValue;

use regex::Regex;

use colorhsl::*;
use sort_hsl::*;

struct Filter {
    
}

#[derive(Debug)]
pub enum Layout {
    Grid,
    Table,
    DoubleTable,
    Block,
    Error,
}

pub const DEFAULT_LAYOUT: Layout = Layout::Grid;

impl Layout {
    pub fn new(s: &str) -> Layout {
        match s.trim().to_lowercase().as_str() {
            "grid" => Layout::Grid,
            "table" => Layout::Table,
            "doubletable" => Layout::DoubleTable,
            "block" => Layout::Block,
            "" => DEFAULT_LAYOUT,
            _ => Layout::Error,
        }
    }
    pub fn create(s: &str) -> Layout {
        match s.trim().to_lowercase().as_str() {
            "grid" => Layout::Grid,
            "table" => Layout::Table,
            "doubletable" => Layout::DoubleTable,
            "block" => Layout::Block,
            _ => DEFAULT_LAYOUT,
        }
    } 
}

impl Clone for Layout {
    fn clone(&self) -> Layout {
        match *self {
            Layout::Grid => Layout::Grid,
            Layout::Table => Layout::Table,
            Layout::DoubleTable => Layout::DoubleTable,
            Layout::Block => Layout::Block,
            Layout::Error => Layout::Error, 
        }
    }
}

impl FromStr for Layout {
    type Err = ();
    fn from_str(s: &str) -> Result<Layout, Self::Err> {
        match Layout::new(s) {
            Layout::Error => Err(()),
            a => Ok(a)
        }
    }
}

#[derive(Debug)]
pub enum SortMethod {
    Hsl,
    Hls,
    Lsh,
    Lhs,
    Slh,
    Shl,
    Error,
}


impl Clone for SortMethod {
    fn clone(&self) -> SortMethod {
        match *self {
            SortMethod::Error => SortMethod::Error, 
            SortMethod::Hsl => SortMethod::Hsl,
            SortMethod::Hls => SortMethod::Hls,
            SortMethod::Lsh => SortMethod::Lsh,
            SortMethod::Lhs => SortMethod::Lhs,
            SortMethod::Slh => SortMethod::Slh,
            SortMethod::Shl => SortMethod::Shl,
        }
    }
}

pub const DEFAULT_SORT: SortMethod = SortMethod::Hls;

impl SortMethod {
    pub fn new(s: &str) -> SortMethod {
        match s.trim().to_lowercase().as_str() {
            "hsl" => SortMethod::Hsl,
            "hls" => SortMethod::Hls,
            "lsh" => SortMethod::Lsh,
            "lhs" => SortMethod::Lhs,
            "slh" => SortMethod::Slh,
            "shl" => SortMethod::Shl,
            "" => DEFAULT_SORT,
            _ => SortMethod::Error,
        }
    }
    pub fn create(s: &str) -> SortMethod {
        match s.trim().to_lowercase().as_str() {
            "hsl" => SortMethod::Hsl,
            "hls" => SortMethod::Hls,
            "lsh" => SortMethod::Lsh,
            "lhs" => SortMethod::Lhs,
            "slh" => SortMethod::Slh,
            "shl" => SortMethod::Shl,
            _ => DEFAULT_SORT,
        }
    }
    pub fn sort(&self, list: &Vec<ColorHsl>, reverse: bool) -> Vec<ColorHsl> {
        // let new: Vec<ColorHsl> = list.clone();
        if !reverse {
            match self {
                &SortMethod::Hsl => ColorHsl::sort_vector(list, sort_by_hsl),
                &SortMethod::Hls => ColorHsl::sort_vector(list, sort_by_hls),
                &SortMethod::Lsh => ColorHsl::sort_vector(list, sort_by_lsh),
                &SortMethod::Lhs => ColorHsl::sort_vector(list, sort_by_lhs),
                &SortMethod::Slh => ColorHsl::sort_vector(list, sort_by_slh),
                &SortMethod::Shl => ColorHsl::sort_vector(list, sort_by_shl),
                              _ => ColorHsl::sort_vector(list, sort_by_hls),
            }
        } else {
            match self {
                &SortMethod::Hsl => ColorHsl::sort_vector(list, sort_rev_hsl),
                &SortMethod::Hls => ColorHsl::sort_vector(list, sort_rev_hls),
                &SortMethod::Lsh => ColorHsl::sort_vector(list, sort_rev_lsh),
                &SortMethod::Lhs => ColorHsl::sort_vector(list, sort_rev_lhs),
                &SortMethod::Slh => ColorHsl::sort_vector(list, sort_rev_slh),
                &SortMethod::Shl => ColorHsl::sort_vector(list, sort_rev_shl),
                              _ => ColorHsl::sort_vector(list, sort_rev_hls),
            }
        }
        
    }
}

impl FromStr for SortMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<SortMethod, Self::Err> {
        match SortMethod::new(s) {
            SortMethod::Error => Err(()),
            a => Ok(a)
        }
    }
}

#[derive(Debug)]
pub struct Page {
    pub sort: SortMethod,
    pub layout: Layout,
    pub add: Option<ColorHsl>,
    pub adds: Option<Vec<ColorHsl>>,
    pub persist: Vec<String>,
    pub reverse: bool,
    
}

impl Clone for Page {
    fn clone(&self) -> Page {
        Page {
            sort: self.sort.clone(),
            layout: self.layout.clone(),
            add: self.add.clone(),
            adds: self.adds.clone(),
            persist: self.persist.clone(),
            reverse: if self.reverse { true } else { false },
        }
    }
}

impl Page {
    pub fn default() -> Page {
        Page {
            sort: DEFAULT_SORT,
            layout: DEFAULT_LAYOUT,
            add: None,
            adds: None,
            persist: Vec::new(),
            reverse: false,
        }
    }
}

impl FromData for Page {
    type Error = ();
    
    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        
        let mut dat = Vec::new();
        match data.stream_to(&mut dat) {
            Ok(_) => {},
            // _ => { return Outcome<Self, Self::Error>::Error },
            _ => { 
                panic!("Could not stream form data");
                // println!("Could not stream form data.");
                // return Outcome::Success(Page::default()) 
            },
        }
        let fdata = str::from_utf8(&dat).unwrap_or("");
        println!("{:?}", fdata);
        let mut sort = DEFAULT_SORT;
        let mut layout = DEFAULT_LAYOUT;
        let mut add = None;
        let mut adds = None;
        let mut persist: Vec<String> = Vec::new();
        let mut reverse: bool = false;

        let parts: Vec<&str> = fdata.split('&').collect();
        
        for part in parts {
            if part != "" && !part.ends_with('=') {
                let pieces: Vec<&str> = part.splitn(2, '=').collect();
                if pieces.len() != 2 { continue; }
                let key = pieces[0];
                let val = pieces[1];
                
                
                match key {
                    "persistence" => {
                        let de = URI::percent_decode(val.as_bytes());
                        let decoded: String = match de {
                            Ok(d2) => d2.to_string().replace("+", " "),
                            _ => "".to_string(),
                        };
                        // for raw in decoded.split(',') {
                        //     let d = raw.trim();
                        // }
                        let colors: Vec<_> = decoded.split(',').map(|ref c| c.trim().to_string()).collect();
                        persist = colors;
                    },
                    "reverse" => {
                        if val == "true" {
                            reverse = true;
                        }
                    },
                    "sort" => {
                        sort = SortMethod::create(val);
                        println!("Set Sort to {:?}", sort);
                    },
                    "layout" => {
                        layout = Layout::create(val);
                        println!("Set Layout to {:?}", layout);
                    },
                    "add" => { 
                        let de = URI::percent_decode(val.as_bytes());
                        match de {
                            Ok(d) => {
                                let dec = d.to_string();
                                add = ColorHsl::from_hex(&dec, &dec); 
                                let atmp = ColorHsl::from_hex(&dec, &dec); 
                                if let Some(a) = atmp {
                                    println!("Set add to {}", a.hex);
                                }
                            },
                            _ => { println!("Could not decode string: `{}`", val); },
                        }
                        // add = ColorHsl::from_hex(val, val); 
                        // let atmp = ColorHsl::from_hex(val, val); 
                        // if let Some(a) = atmp {
                        //     println!("Set add to {}", a.hex);
                        // }
                    },
                    "adds" => {
                        let de = URI::percent_decode(val.as_bytes());
                        let dstr: String = match de {
                            Ok(d2) => d2.to_string().replace("+", " "),
                            _ => "".to_string(),
                        };
                        println!("Decoded string:\n{}", dstr);
                        let mut tlen: usize = 0;
                        if dstr.starts_with("[{") && dstr.ends_with("}]") {
                            // parse as json
                            /*match de {
                                Ok(d) => {
                                    let dec = d.to_string();
                                    let t = ColorHsl::read_json_str(&dec);
                                    adds = match t.len() {
                                        0 => None,
                                        l => {
                                                tlen = l;
                                                Some(t)
                                            },
                                    };
                                }
                                _ => {},
                            }*/
                            if dstr != "" {
                                let t = ColorHsl::read_json_str(&dstr);
                                adds = match t.len() {
                                    0 => None,
                                    l => {
                                            tlen = l;
                                            Some(t)
                                        },
                                };
                            }
                            if adds.is_some() {
                                println!("Set adds to {} items", tlen);
                            } else {
                                println!("Set adds to None");
                            }
                        } else if dstr.contains(',') {
                            // parse as list of comma separated hex codes
                            let mut alist: Vec<ColorHsl> = Vec::new();
                            for col in dstr.split(',') {
                                let co = col.trim();
                                match ColorHsl::from_hex(co, co) {
                                    Some(c) => alist.push(c),
                                    _ => {},
                                }
                            }
                            adds = Some(alist);
                        } else {
                            // try parsing as a single hex code
                            let tad = ColorHsl::from_hex(&dstr, &dstr);
                            if let Some(ta) = tad {
                                adds = Some(vec![ta]);
                            }
                        }
                    },
                    // To be implemented later to upload color files
                    // "file" => {
                    //     let mut d = Vec::new();
                    //     let f = entry.data.as_file().expect("not file");
                    //     f.read_to_end(&mut d).expect("cant read");
                    //     file = Some(d);
                    // },
                    _ => {},
                }
            }
        }
        
        let o = Page {
            sort,
            layout,
            add,
            adds,
            persist,
            reverse,
        };
        
        Outcome::Success(o)
    }
    
}

pub fn header() -> String {
    let head = include_str!("template_header.html");
    head.to_string()
}

pub fn form(ops: &Page) -> String {
    // let form =include_str!("template_form.html");
    // form.to_string()
    let selhsl = match ops.sort { SortMethod::Hsl => "selected", _ =>  "" };
    let selhls = match ops.sort { SortMethod::Hls => "selected", _ =>  "" };
    let sellsh = match ops.sort { SortMethod::Lsh => "selected", _ =>  "" };
    let sellhs = match ops.sort { SortMethod::Lhs => "selected", _ =>  "" };
    let selslh = match ops.sort { SortMethod::Slh => "selected", _ =>  "" };
    let selshl = match ops.sort { SortMethod::Shl => "selected", _ =>  "" };
    
    let selrev = match ops.reverse { true => "checked", false => "" };
    
    let selgrid = match ops.layout { Layout::Grid => "selected", _ =>  "" };
    let seltable = match ops.layout { Layout::Table => "selected", _ =>  "" };
    let seldoubletable = match ops.layout { Layout::DoubleTable => "selected", _ =>  "" };
    let selblock = match ops.layout { Layout::Block => "selected", _ => "" };
    
    format!(r###"
      <form method="post" name="colrform" action="http://localhost:8000/" class="sticky-top" onsubmit="persist()">
        <div class="row v-form">
          
          <div class="col-md-4">
            <!-- <input type="text" class="form-control" placeholder=""> -->
            <div class="input-group">
              <div class="input-group">
                <!--
                <input type="text" name="add" class="form-control" placeholder="Add Color" aria-label="Add Color">
                <span class="input-group-btn">
                  <button class="btn btn-secondary" type="button">
                    <i class="fa fa-file-code-o" aria-hidden="true"></i>
                  </button>
                </span>
                -->
                
                <input type="text" name="adds" class="form-control" placeholder="Add Colors" aria-label="Add Colors">
                <!-- 
                <span class="input-group-btn">
                  <button class="btn btn-secondary" type="button">
                    <i class="fa fa-tint" aria-hidden="true"></i>
                  </button>
                  -->
                  <span class="input-group-addon" id="basic-addon1">
                    <i class="fa fa-tint" aria-hidden="true"></i>
                  </span>
                <!-- </span> -->
                
              </div>
            </div>
          </div>
          <!--
          <div class="col">
            <div class="input-group">
              <input type="text" name="hmin" class="form-control" placeholder="Hue Min" aria-label="HueMin">
              <input type="text" name="smin" class="form-control" placeholder="Saturation Min" aria-label="SatMin">
              <input type="text" name="lmin" class="form-control" placeholder="Lightness Min" aria-label="LumMin">
            </div>
            <div class="input-group">
              <input type="text" name="hmax" class="form-control" placeholder="Hue Max" aria-label="HueMax">
              <input type="text" name="smax" class="form-control" placeholder="Saturation Max" aria-label="SatMax">
              <input type="text" name="lmax" class="form-control" placeholder="Lightness Max" aria-label="LumMax">
            </div>
          </div>
          -->
          
          
          <div class="col-md-2 revsort">
            <div class="form-check">
                <label class="form-check-label" for="sort-reverse">
                  <input type="checkbox" id="sort-reverse" name="reverse" value="true" class="form-check-input" {revsel}>
                  Reverse Sort
                </label>
            </div>
          </div>
          
          
          <div class="col-md-2">
            <div class="input-group">
              <span class="input-group-addon" id="basic-addon1">
                <i class="fa fa-filter" aria-hidden="true"></i>
              </span>
              <select name="sort" id="SortBy" class="custom-select" style="width: 100%">
                <option>Sort By...</option>
                <option {hslsel} value="hsl">HSL</option>
                <option {hlssel} value="hls">HLS</option>
                <option {lshsel} value="lsh">LSH</option>
                <option {lhssel} value="lhs">LHS</option>
                <option {slhsel} value="slh">SLH</option>
                <option {shlsel} value="shl">SHL</option>
              </select>
            </div>
          </div>
          
          
          <div class="col-md-2">
              <div class="input-group">
                  <span class="input-group-addon" id="basic-addon1">
                    <i class="fa fa-table" aria-hidden="true"></i>
                  </span>
                  <select name="layout" id="Layout" class="custom-select" onchange="set_layout()" style="width: 100%">
                    <option {gridsel} value="Grid">Grid</option>
                    <option {blocksel} value="Block">Block</option>
                    <option {doubletablesel} value="DoubleTable">Double List</option>
                    <option {tablesel} value="Table">List</option>
                  </select>
                </div>
          </div>
          <div class="col-md-1">
            <button type="submit" class="btn">Submit</button>
          </div>
          
          
        </div>
        <input type="hidden" name="persistence" value="">
      </form>
      <br>
      <div class="v-collection">
"###, 
        hslsel=selhsl, hlssel=selhls, lshsel=sellsh, lhssel=sellhs, slhsel=selslh, shlsel=selshl,
        gridsel=selgrid, tablesel=seltable, doubletablesel=seldoubletable
        , blocksel=selblock
        , revsel=selrev
        // ,opts=ops
    )
      // after <br>, before <div class="v-collection">
      // Options: {opts:#?}
}

pub fn color_template(c: &ColorHsl) -> String {
    format!("
        <div class=\"v-cont\">
          <div class=\"v-topbox\" data-clipboard-text=\"{hex}\">
            <div class=\"v-leftbox\">
              <div class=\"v-colorbox\"></div>
            </div>
            <div class=\"v-rightbox\">
              <div class=\"\"><div class=\"v-table-hex\">{hex}</div></div>
              <div class=\"\"><div class=\"v-table-rgb\" data-clipboard-text=\"{red}, {green}, {blue}\">{red}, {green}, {blue}</div></div>
              <div class=\"\"><div class=\"v-table-hsl\" data-clipboard-text=\"{hue:.4}, {sat:.4}, {lum:.4}\">{hue:.2}, {sat:.2}, {lum:.2}</div></div>
            </div>
          </div>
        </div>
", hex=c.hex, hue=c.h, sat=c.s, lum=c.l, red=c.r, green=c.g, blue=c.b)

}

pub fn body(v: &Vec<ColorHsl>) -> String {
    let mut out: String = String::new();
    if v.len() > 0 {
        for i in v {
            out.push_str(&color_template(i));
        }
    }
    out
}

pub fn footer() -> String {
    let foot = include_str!("template_footer.html");
    foot.to_string()
}



