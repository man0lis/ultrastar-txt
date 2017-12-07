use std::path::PathBuf;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub struct Header {
    // mandatory data from headers
    pub artist: String,
    pub title: String,
    pub bpm: f32,
    pub audio_path: PathBuf,

    // optional data from headers
    pub gap: Option<f32>,
    pub cover_path: Option<PathBuf>,
    pub background_path: Option<PathBuf>,
    pub video_path: Option<PathBuf>,
    pub video_gap: Option<f32>,
    pub genre: Option<String>,
    pub edition: Option<String>,
    pub language: Option<String>,
    pub year: Option<u32>,
    pub relative: Option<bool>,
    pub unknown: Option<HashMap<String, String>>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Song {
    pub header: Header,
    pub lines: Option<Vec<Line>>,
    pub txt_path: PathBuf,
}


#[derive(PartialEq, Clone, Debug)]
pub enum Note {
    Regular {
        start: i32,
        duration: i32,
        pitch: i32,
        text: String,
    },
    Golden {
        start: i32,
        duration: i32,
        pitch: i32,
        text: String,
    },
    Freestyle {
        start: i32,
        duration: i32,
        pitch: i32, //pitch might not be needed but not including it might lose data from orig file
        text: String,
    }, 
}

#[derive(PartialEq, Clone, Debug)]
pub struct Line {
    pub start: i32,
    pub notes: Vec<Note>,
}