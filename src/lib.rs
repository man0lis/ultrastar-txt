#[macro_use] extern crate lazy_static;
extern crate regex;

use std::path::PathBuf;
use regex::Regex;

//use std::io::{BufRead, BufReader, Read};

#[derive(PartialEq, Clone, Debug)]
pub struct Header {
    // mandatory data from headers
    pub artist: String,
    pub title: String,
    pub bpm: f32,
    pub gap: f32,
    pub audio_path: PathBuf,

    // optional data from headers
    pub cover_path: Option<PathBuf>,
    pub background_path: Option<PathBuf>,
    pub video_path: Option<PathBuf>,
    pub video_gap: Option<f32>,
    pub genre: Option<String>,
    pub edition: Option<String>,
    pub language: Option<String>,
    pub year: Option<u32>,
    pub relative: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Song {
    pub header: Header,
    pub lines: Vec<Line>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum NoteType {
    Regular,
    Golden,
    Freestyle,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Note {
    pub notetype: NoteType,
    pub start: i32,
    pub duration: i32,
    pub pitch: i32,
    pub text: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Line {
    pub start: i32,
    pub notes: Vec<Note>,
}

fn get_empty_header() -> Header {
    Header {
        artist: String::from(""),
        title: String::from(""),
        bpm: std::f32::NAN,
        gap: std::f32::NAN,
        audio_path: PathBuf::from(""),
        cover_path: None,
        background_path: None,
        video_path: None,
        video_gap: None,
        genre: None,
        edition: None,
        language: None,
        year: None,
        relative: false,
    }
}

pub fn parse_txt_header_str(txt_str: &str) -> Result<Header, &str> {
    let mut found_artist = false;
    let mut found_title = false;
    let mut found_bpm = false;
    let mut found_gap = false;
    let mut found_audio_path = false;

    let mut header = get_empty_header();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"#([A-Z3]*):(.*)").unwrap();
    }

    for line in txt_str.lines() {
        let cap = match RE.captures(line) {
            Some(x) => x,
            None => break,
        };
        let key = cap.get(1).unwrap().as_str();
        let value = cap.get(2).unwrap().as_str();
        match key {
            "TITLE" => {
                header.title = String::from(value);
                if found_title {
                    return Err("multiple TITLE tags found")
                }
                found_title = true;
            },
            "ARTIST" => {
                header.artist = String::from(value);
                if found_artist {
                    return Err("multiple ARTIST tags found")
                }
                found_artist = true;
            },
            "MP3" => {
                header.audio_path = PathBuf::from(value);
                if found_audio_path {
                    return Err("multiple MP3 tags found")
                }
                found_audio_path = true;
            },
            "GAP" => {
                header.gap = match value.replace(",", ".").parse() {
                    Ok(x) => x,
                    Err(_) => return Err("invalid GAP"),
                };
                if found_gap {
                    return Err("multiple GAP tags found")
                }
                found_gap = true;
            },
            "BPM" => {
                header.bpm = match value.replace(",", ".").parse() {
                    Ok(x) => x,
                    Err(_) => return Err("invalid BPM"),
                };
                if found_bpm {
                    return Err("multiple BPM tags found")
                }
                found_bpm = true;
            },

            // Optional Header fields
            "COVER" => {
                header.cover_path = Some(PathBuf::from(value));
            },
            "BACKGROUND" => {
                header.background_path = Some(PathBuf::from(value));
            },
            "VIDEO" => {
                header.video_path = Some(PathBuf::from(value));
            },
            "VIDEOGAP" => {
                header.video_gap = match value.replace(",", ".").parse() {
                    Ok(x) => Some(x),
                    Err(_) => {
                        println!("Warning: Invalid video gap");
                        None
                    },
                };
            },
            "GENRE" => {
                header.genre = Some(String::from(value));
                println!("Set genre to: {:?}", header.genre);
            },
            "EDITION" => {
                header.edition = Some(String::from(value));
                println!("Set edition to: {:?}", header.edition);
            },
            "LANGUAGE" => {
                header.language = Some(String::from(value));
                println!("Set language to: {:?}", header.language);
            },
            "YEAR" => {
                header.year = match value.parse() {
                    Ok(x) => Some(x),
                    Err(_) => {
                        println!("Warning: Invalid year");
                        None
                    },
                };
            },
            "RELATIVE" => {
                header.relative = match value {
                    "YES" => true,
                    "NO" => false,
                    _ => { println!("Warning: Invalid relative tag");
                    false
                },
            };
        },
        _ => println!("{}",key),
    };

}
Ok(header)
}

pub fn parse_txt_lines_str(txt_str: &str) -> Result<Vec<Line>, &str> {

    let re = Regex::new(r"([A-Z3]*):(.*)").unwrap();

    for line in txt_str.lines() {
        let cap = match re.captures(line) {
            Some(x) => x,
            None => break,
        };
        let key = cap.get(1).unwrap().as_str();
        let value = cap.get(2).unwrap().as_str();
    }
    Ok(Vec::new())

}
