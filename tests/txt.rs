extern crate ultrastar_txt;

use ultrastar_txt::*;
use std::path::PathBuf;

#[test]
fn parse_simple_txt_header_str() {
    let txt = get_simple_txt_str();
    let header = get_simple_txt_header();
    assert_eq!(header, parse_txt_header_str(txt).unwrap());
}

#[test]
fn parse_simple_txt_lines_str() {
    let txt = get_simple_txt_str();
    let lines = get_simple_txt_lines();
    assert_eq!(lines, parse_txt_lines_str(txt).unwrap());
}

#[test]
fn komma_in_float_number() {
    let txt = include_str!("komma_in_float.txt");
    assert!(parse_txt_header_str(txt).is_ok())
}

fn get_simple_txt_str() -> &'static str {
    include_str!("simple_txt_with_all_features.txt")
}

fn get_simple_txt_header() -> Header {
    Header {
        artist: String::from("Testartist"),
        title: String::from("Testsong"),
        bpm: 123.0,
        gap: 666.0,
        audio_path: PathBuf::from("Testfile.mp3"),
        relative: false,
        video_path: Some(PathBuf::from("DLzxrzFCyOs.mp4")),
        cover_path: Some(PathBuf::from("Cover.jpg")),
        background_path: Some(PathBuf::from("BG.jpg")),
        video_gap: Some(777.0),
        genre: Some(String::from("Music")),
        edition: Some(String::from("Testmusic")),
        language: Some(String::from("en")),
        year: Some(1337),
    }
}

fn get_simple_txt_lines() -> Vec<Line> {
    vec![
    Line {
        start: 0,
        notes: vec![
        Note {
            notetype: NoteType::Regular,
            start: 0,
            duration: 4,
            pitch: 59,
            text: String::from("Test "),
        },
        Note {
            notetype: NoteType::Regular,
            start: 4,
            duration: 4,
            pitch: 59,
            text: String::from("I"),
        },
        Note {
            notetype: NoteType::Regular,
            start: 8,
            duration: 4,
            pitch: 59,
            text: String::from("'m "),
        },
        Note {
            notetype: NoteType::Golden,
            start: 12,
            duration: 4,
            pitch: 59,
            text: String::from("test"),
        },
        Note {
            notetype: NoteType::Regular,
            start: 16,
            duration: 4,
            pitch: 59,
            text: String::from("ing."),
        },
        ],
    },
    Line {
        start: 20,
        notes: vec![
        Note {
            notetype: NoteType::Regular,
            start: 24,
            duration: 4,
            pitch: 59,
            text: String::from("Test "),
        },
        Note {
            notetype: NoteType::Regular,
            start: 28,
            duration: 4,
            pitch: 59,
            text: String::from("I"),
        },
        Note {
            notetype: NoteType::Regular,
            start: 32,
            duration: 4,
            pitch: 59,
            text: String::from("'m "),
        },
        Note {
            notetype: NoteType::Freestyle,
            start: 36,
            duration: 4,
            pitch: 59,
            text: String::from("test"),
        },
        Note {
            notetype: NoteType::Freestyle,
            start: 40,
            duration: 4,
            pitch: 59,
            text: String::from("ing."),
        },
        ],
    },
    ]
}
