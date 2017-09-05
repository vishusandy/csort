use colorhsl::*;
use std::str::FromStr;
use std::result::Result;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub enum SortBy {
    Hsl,
    Hls,
    Lsh,
    Slh,
    Lhs,
    Error,
}

pub const SORT_DEFAULT: SortBy = SortBy::Hls;

impl SortBy {
    pub fn new(s: &str) -> SortBy {
        // println!("Converting {} to SortBy", s.trim().to_lowercase());
        match s.trim().to_lowercase().as_str() {
            "hls"| "h" | "hue"  => SortBy::Hls,
            "hsl"| "hl" | "h2" | "hue2" => SortBy::Hsl,
            "lsh" | "l" | "ls" | "light" | "lightness" | "lum" | "luminance" => SortBy::Lsh,
            "slh" | "s" | "sl" | "sat" | "saturation" => SortBy::Slh,
            "lhs" | "lh" | "light2" | "lightness2" | "lum2" | "luminance2" => SortBy::Lhs,
            "" => SORT_DEFAULT,
            _ => SortBy::Error,
        }
    }
}

impl FromStr for SortBy {
    type Err = ();
    fn from_str(s: &str) -> Result<SortBy, Self::Err> {
        match SortBy::new(s) {
            SortBy::Error => Err(()),
            a => {
                // println!("Converted SortBy to {:?}", a);
                Ok(a)
            },
        }
    }
}

fn cmp_h(a: &ColorHsl, b: &ColorHsl) -> Ordering {
    a.h.partial_cmp(&b.h).unwrap_or(Ordering::Equal)
}
fn cmp_s(a: &ColorHsl, b: &ColorHsl) -> Ordering {
    a.s.partial_cmp(&b.s).unwrap_or(Ordering::Equal)
}
fn cmp_l(a: &ColorHsl, b: &ColorHsl) -> Ordering {
    a.l.partial_cmp(&b.l).unwrap_or(Ordering::Equal)
}


// pub fn sort_method(by: SortBy) -> Box<Fn(&ColorHsl, &ColorHsl) -> Ordering {
    // Box::new()

pub fn sort_method(by: SortBy) -> Box<FnOnce(&ColorHsl, &ColorHsl) -> Ordering> {
    match by {
        SortBy::Hsl => Box::new(sort_by_hsl),
        SortBy::Hls => Box::new(sort_by_hls),
        SortBy::Lsh => Box::new(sort_by_lsh),
        SortBy::Slh => Box::new(sort_by_slh),
        SortBy::Lhs => Box::new(sort_by_lhs),
        SortBy::Error | _ => 
            match SORT_DEFAULT {
                SortBy::Error => Box::new(sort_by_hls),
                _ => sort_method(SORT_DEFAULT)
            },
            /*if SORT_DEFAULT != SortBy::Error {
                sort_method(SORT_DEFAULT)
            } else {
                Box::new(sort_by_hls)
            },*/
    }
}


/*
pub fn sort_method(by: SortBy) -> Box<FnOnce(&ColorHsl, &ColorHsl) -> Ordering> {
    let sorting_hsl = |&a, &b| -> Ordering {
        match cmp_h(a, b) { // == Ordering::Equal {
            Ordering::Equal => match cmp_s(a, b) {
                Ordering::Equal => cmp_l(a, b),
                y => y,
            },
            x => x,
        }
    };
    let sorting_hls = |&a, &b| -> Ordering {
        match cmp_h(a, b) { // == Ordering::Equal {
            Ordering::Equal => match cmp_l(a, b) {
                Ordering::Equal => cmp_s(a, b),
                y => y,
            },
            x => x,
        }
    };
    let sorting_lsh = |&a, &b| -> Ordering {
        match cmp_l(a, b) { // == Ordering::Equal {
            Ordering::Equal => match cmp_s(a, b) {
                Ordering::Equal => cmp_h(a, b),
                y => y,
            },
            x => x,
        }
    };
    let sorting_slh = |&a, &b| -> Ordering {
        match cmp_s(a, b) { // == Ordering::Equal {
            Ordering::Equal => match cmp_l(a, b) {
                Ordering::Equal => cmp_h(a, b),
                y => y,
            },
            x => x,
        }
    };
    let sorting_lhs = |&a, &b| -> Ordering {
        match cmp_l(a, b) { // == Ordering::Equal {
            Ordering::Equal => match cmp_h(a, b) {
                Ordering::Equal => cmp_s(a, b),
                y => y,
            },
            x => x,
        }
    };

    match by {
        SortBy::Hsl => Box::new(sorting_hsl),
        SortBy::Hls => Box::new(sorting_hls),
        SortBy::Lsh => Box::new(sorting_lsh),
        SortBy::Slh => Box::new(sorting_slh),
        SortBy::Lhs => Box::new(sorting_lhs),
        SortBy::Error | _ => 
            match SORT_DEFAULT {
                SortBy::Error => Box::new(sorting_hls),
                _ => sort_method(SORT_DEFAULT)
            },
            /*if SORT_DEFAULT != SortBy::Error {
                sort_method(SORT_DEFAULT)
            } else {
                Box::new(sort_by_hls)
            },*/
    }

}
*/


pub fn sort_by_hsl(a: &ColorHsl, b: &ColorHsl) -> Ordering {
    match cmp_h(a, b) { // == Ordering::Equal {
        Ordering::Equal => match cmp_s(a, b) {
            Ordering::Equal => cmp_l(a, b),
            y => y,
        },
        x => x,
    }
}

pub fn sort_by_hls(a: &ColorHsl, b: &ColorHsl) -> Ordering {
    match cmp_h(a, b) { // == Ordering::Equal {
        Ordering::Equal => match cmp_l(a, b) {
            Ordering::Equal => cmp_s(a, b),
            y => y,
        },
        x => x,
    }
}

pub fn sort_by_lsh(a: &ColorHsl, b: &ColorHsl) -> Ordering {
    match cmp_l(a, b) { // == Ordering::Equal {
        Ordering::Equal => match cmp_s(a, b) {
            Ordering::Equal => cmp_h(a, b),
            y => y,
        },
        x => x,
    }
}

pub fn sort_by_slh(a: &ColorHsl, b: &ColorHsl) -> Ordering {
    match cmp_s(a, b) { // == Ordering::Equal {
        Ordering::Equal => match cmp_l(a, b) {
            Ordering::Equal => cmp_h(a, b),
            y => y,
        },
        x => x,
    }
}

pub fn sort_by_lhs(a: &ColorHsl, b: &ColorHsl) -> Ordering {
    match cmp_l(a, b) { // == Ordering::Equal {
        Ordering::Equal => match cmp_h(a, b) {
            Ordering::Equal => cmp_s(a, b),
            y => y,
        },
        x => x,
    }
}