
use std::io::{Cursor, Read};
use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
use multipart::server::Multipart;
use rocket::request::FromParam;
use rocket::http::RawStr;

use std::str;
use std::str::FromStr;
use std::result::Result;
use rocket::request::FromFormValue;

use regex::Regex;

use colorhsl::*;

struct Filter {
    
}

#[derive(Debug)]
pub enum Layout {
    Grid,
    Table,
    DoubleTable,
    Error,
}

pub const DEFAULT_LAYOUT: Layout = Layout::Grid;

impl Layout {
    pub fn new(s: &str) -> Layout {
        match s.trim().to_lowercase().as_str() {
            "grid" => Layout::Grid,
            "table" => Layout::Table,
            "doubletable" => Layout::DoubleTable,
            "" => DEFAULT_LAYOUT,
            _ => Layout::Error,
        }
    }
    pub fn create(s: &str) -> Layout {
        match s.trim().to_lowercase().as_str() {
            "grid" => Layout::Grid,
            "table" => Layout::Table,
            "doubletable" => Layout::DoubleTable,
            _ => DEFAULT_LAYOUT,
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
    
}

impl Page {
    pub fn default() -> Page {
        Page {
            sort: DEFAULT_SORT,
            layout: DEFAULT_LAYOUT,
        }
    }
}

impl FromData for Page {
    type Error = ();
    
    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        // boring setup variables
        
        /*
        let contype = request.headers().get_one("Content-Type").expect("No content type specified.");
        let cidx = contype.find("boundary=").unwrap_or(0);
        let boundary = &contype[(cidx + "boundary=".len())..];
        let mut dat = Vec::new();
        data.stream_to(&mut dat).expect("Could not read data stream");
        println!(" --- Content type: {:?}", contype);
        println!(" --- Dat: {:#?}", dat);
        let mut parts = Multipart::with_body(Cursor::new(dat), boundary);
        */
        
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
        // the potatoes
        // let mut sort: Option<SortMethod> = None;
        // let mut layout: Option<Layout> = None;
        let mut sort = DEFAULT_SORT;
        let mut layout = DEFAULT_LAYOUT;
        
        // parts.foreach_entry(|mut entry| {
            // match entry.name.to_lowercase().as_str() {
        
        // let re = Regex::new("&").unwrap();
        // let re = Regex::new("=").unwrap();
        // for part in re.split(fdata) {
        let parts: Vec<&str> = fdata.split('&').collect();
        
        for part in parts {
            // if part.contains("=") && !part.ends_with("=") {
            if part != "" && !part.ends_with('=') {
                // let pos = part.find('=').unwrap_or(0);
                // let (key, val) = part.split_at(pos);
                
                let pieces: Vec<&str> = part.splitn(2, '=').collect();
                if pieces.len() != 2 { continue; }
                let key = pieces[0];
                let val = pieces[1];
                // match part.find('=') {
                //     Some
                // }
                
                
                match key {
                    "sort" => {
                        // let t = entry.data.as_text().unwrap_or("");
                        // sort = Some(t.into());
                        // sort = SortMethod::create(t);
                        sort = SortMethod::create(val);
                        println!("Set Sort to {:?}", sort);
                    },
                    "layout" => {
                        // let t = entry.data.as_text().unwrap_or("");
                        // layout = Layout::create(t);
                        layout = Layout::create(val);
                        println!("Set Layout to {:?}", layout);
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
        // }).expect("Failed to iterate parts.");
        
        let o = Page {
            sort,
            layout,
            
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
    // let selhsl = if ops.Sort == SortMethod::Hsl { "selected" } else { "" };
    let selhsl = match ops.sort { SortMethod::Hsl => "selected", _ =>  "" };
    let selhls = match ops.sort { SortMethod::Hls => "selected", _ =>  "" };
    let sellsh = match ops.sort { SortMethod::Lsh => "selected", _ =>  "" };
    let sellhs = match ops.sort { SortMethod::Lhs => "selected", _ =>  "" };
    let selslh = match ops.sort { SortMethod::Slh => "selected", _ =>  "" };
    let selshl = match ops.sort { SortMethod::Shl => "selected", _ =>  "" };
    
    // let selgrid = if ops.Layout == Layout::Grid { "selected" } else { "" };
    let selgrid = match ops.layout { Layout::Grid => "selected", _ =>  "" };
    let seltable = match ops.layout { Layout::Table => "selected", _ =>  "" };
    let seldoubletable = match ops.layout { Layout::DoubleTable => "selected", _ =>  "" };
    
    
    format!(r###"
      <form method="post" action="http://localhost:8000/" class="sticky-top">
        <div class="row v-form">
          <div class="col-md-3">
            <!-- <input type="text" class="form-control" placeholder=""> -->
            <div class="input-group">
              <div class="input-group">
                <input type="text" class="form-control" placeholder="Add Color" aria-label="Add Color">
                <span class="input-group-btn">
                  <button class="btn btn-secondary" type="button">
                    <i class="fa fa-tint" aria-hidden="true"></i>
                  </button>
                </span>
              </div>
              <!--
              <input type="text" class="form-control" placeholder="Color" aria-label="HexColor" aria-describedby="basic-addon1">
              <span class="input-group-addon" id="basic-addon1">
                <i class="fa fa-tint" aria-hidden="true"></i>
              </span>
              -->
            </div>
          </div>
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
          <div class="col-md-3">
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
                <!-- <option selected value="Grid">Layout</option> -->
                <option {gridsel} value="Grid">Grid</option>
                <option {tablesel} value="Table">List</option>
                <option {doubletablesel} value="DoubleTable">Double List</option>
              </select>
            </div>
            <div class="">
            <div class="row">
              <div class="col col-lg-5">
                <button type="button" class="btn">
                  <i class="fa fa-refresh" aria-hidden="true"></i>
                </button>&nbsp;
              </div>
              <div class="col-md-auto">
              </div>
              <div class="col col-lg-5">
                <button type="submit" class="btn">Submit</button>
              </div>
            </div>
            </div>
          </div>
        </div>
      </form>
      <br>
      Options: {opts:#?}
      <div class="v-collection">
"###, 
        hslsel=selhsl, hlssel=selhls, lshsel=sellsh, lhssel=sellhs, slhsel=selslh, shlsel=selshl,
        gridsel=selgrid, tablesel=seltable, doubletablesel=seldoubletable,
        opts=ops
    )
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
              <div class=\"\"><div class=\"v-table-hsl\" data-clipboard-text=\"{hue:.4}, {sat:.4}, {lum:.4}\">{hue:.4}, {sat:.4}, {lum:.4}</div></div>
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



