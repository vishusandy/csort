#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate multipart;

#[post("/", data = "<upload>")]
fn index(upload: DummyMultipart) -> String {
    format!("I read this: {:?}", upload)
}

#[derive(Debug)]
struct DummyMultipart {
    alpha: String,
    one: i32,
    file: Vec<u8>,
}

use std::io::{Cursor, Read};
use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
use multipart::server::Multipart;

impl FromData for DummyMultipart {
    type Error = ();

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        // All of these errors should be reported
        let ct = request.headers().get_one("Content-Type").expect("no content-type");
        let idx = ct.find("boundary=").expect("no boundary");
        let boundary = &ct[(idx + "boundary=".len())..];

        let mut d = Vec::new();
        data.stream_to(&mut d).expect("Unable to read");

        let mut mp = Multipart::with_body(Cursor::new(d), boundary);

        // Custom implementation parts

        let mut alpha = None;
        let mut one = None;
        let mut file = None;

        mp.foreach_entry(|mut entry| {
            match entry.name.as_str() {
                "alpha" => {
                    let t = entry.data.as_text().expect("not text");
                    alpha = Some(t.into());
                },
                "one" => {
                    let t = entry.data.as_text().expect("not text");
                    let n = t.parse().expect("not number");
                    one = Some(n);
                },
                "file" => {
                    let mut d = Vec::new();
                    let f = entry.data.as_file().expect("not file");
                    f.read_to_end(&mut d).expect("cant read");
                    file = Some(d);
                },
                other => panic!("No known key {}", other),
            }
        }).expect("Unable to iterate");

        let v = DummyMultipart {
            alpha: alpha.expect("alpha not set"),
            one: one.expect("one not set"),
            file: file.expect("file not set"),
        };

        // End custom

        Outcome::Success(v)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}