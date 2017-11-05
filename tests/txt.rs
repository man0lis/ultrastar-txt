extern crate ultrastar_txt;

use ultrastar_txt::*;
use std::path::PathBuf;
use std::collections::HashMap;

#[test]
fn simple_txt_header() {
    let txt = get_simple_txt_str();
    let header = get_simple_txt_header();
    assert_eq!(header, parse_txt_header_str(txt).unwrap());
}

#[test]
fn simple_txt_lines() {
    let txt = get_simple_txt_str();
    let lines = get_simple_txt_lines();
    assert_eq!(lines, parse_txt_lines_str(txt).unwrap());
}

#[test]
fn komma_in_float_number() {
    let txt = include_str!("txts/komma_in_float.txt");
    assert!(parse_txt_header_str(txt).is_ok())
}

#[test]
fn missing_essential_header() {
    let txt = include_str!("txts/missing_essential_header.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::MissingEssential));
}

#[test]
fn value_error_in_header_bpm() {
    let txt = include_str!("txts/value_error_in_header_bpm.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::ValueError { line: 5, field: "BPM" }));
}

#[test]
fn value_error_in_header_gap() {
    let txt = include_str!("txts/value_error_in_header_gap.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::ValueError { line: 4, field: "GAP" }));
}

#[test]
fn value_error_in_header_videogap() {
    let txt = include_str!("txts/value_error_in_header_videogap.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::ValueError { line: 6, field: "VIDEOGAP" }));
}

#[test]
fn value_error_in_header_year() {
    let txt = include_str!("txts/value_error_in_header_year.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::ValueError { line: 7, field: "YEAR" }));
}

#[test]
fn survive_nonstandard_tags() {
    let txt = include_str!("txts/survive_nonstandard_tags.txt");
    let mut header = get_simple_txt_header();
    let mut unknown = HashMap::new();
    unknown.insert(String::from("MedleyStartBeat"), String::from("550"));
    unknown.insert(String::from("MedleyEndBeat"), String::from("863"));
    header.unknown = Some(unknown);
    assert_eq!(parse_txt_header_str(txt).unwrap(), header);
}

#[test]
fn dash_in_lyrics() {
    let txt = include_str!("txts/dash_in_lyrics.txt");
    assert!(parse_txt_lines_str(txt).is_ok());
}

#[test]
fn empty_note_text() {
    let txt = include_str!("txts/empty_note_text.txt");
    assert!(parse_txt_lines_str(txt).is_ok());
}

#[test]
fn survive_duett_tags() {
    let txt = include_str!("txts/survive_duett_tags.txt");
    assert!(parse_txt_lines_str(txt).is_ok());
}

#[test]
fn negative_pitch_in_lines() {
    let txt = include_str!("txts/negative_pitch_in_lines.txt");
    assert!(parse_txt_lines_str(txt).is_ok());
}

#[test]
fn negative_start_in_lines() {
    let txt = include_str!("txts/negative_start_in_lines.txt");
    assert!(parse_txt_lines_str(txt).is_ok());
}

#[test]
fn missing_space_in_line_break() {
    let txt = include_str!("txts/missing_space_in_line_break.txt");
    assert!(parse_txt_lines_str(txt).is_ok());
}

#[test]
fn lower_case_relative() {
    let txt = include_str!("txts/lower_case_relative.txt");
    assert!(parse_txt_header_str(txt).is_ok());
}

#[test]
fn unknown_note_type() {
    let txt = include_str!("txts/unknown_note_type.txt");
    assert_eq!(parse_txt_lines_str(txt), Err(ParserError::UnknownNoteType { line: 7 }));
}

#[test]
fn unknown_tags() {
    let txt = include_str!("txts/unknown_tags.txt");
    let mut header = get_simple_txt_header();
    let mut unknown = HashMap::new();
    unknown.insert(String::from("UNKNOWN"), String::from("tag"));
    unknown.insert(String::from("WHAT"), String::from("is this"));
    header.unknown = Some(unknown);
    assert_eq!(parse_txt_header_str(txt).unwrap(), header);
}

#[test]
fn garbage_line() {
    let txt = include_str!("txts/garbage_line.txt");
    assert_eq!(parse_txt_lines_str(txt), Err(ParserError::ParserFailure { line: 7 }));
}

#[test]
fn empty_optional_tags() {
    let txt = include_str!("txts/empty_optional_tags.txt");
    let mut header = get_simple_txt_header();
    header.relative = None;
    header.video_path = None;
    header.cover_path = None;
    header.background_path = None;
    header.video_gap = None;
    header.genre = None;
    header.edition = None;
    header.language = None;
    header.year = None;
    header.unknown = None;
    assert_eq!(parse_txt_header_str(txt).unwrap(), header);
}

#[test]
fn duplicate_header_artist() {
    let txt = include_str!("txts/duplicate_header_artist.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 3, tag: "ARTIST" }));
}

#[test]
fn duplicate_header_background() {
    let txt = include_str!("txts/duplicate_header_background.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 10, tag: "BACKGROUND" }));
}

#[test]
fn duplicate_header_bpm() {
    let txt = include_str!("txts/duplicate_header_bpm.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 6, tag: "BPM" }));
}

#[test]
fn duplicate_header_cover() {
    let txt = include_str!("txts/duplicate_header_cover.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 9, tag: "COVER" }));
}

#[test]
fn duplicate_header_edition() {
    let txt = include_str!("txts/duplicate_header_edition.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 8, tag: "EDITION" }));
}

#[test]
fn duplicate_header_gap() {
    let txt = include_str!("txts/duplicate_header_gap.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 5, tag: "GAP" }));
}

#[test]
fn duplicate_header_genre() {
    let txt = include_str!("txts/duplicate_header_genre.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 7, tag: "GENRE" }));
}

#[test]
fn duplicate_header_language() {
    let txt = include_str!("txts/duplicate_header_language.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 14, tag: "LANGUAGE" }));
}

#[test]
fn duplicate_header_mp3() {
    let txt = include_str!("txts/duplicate_header_mp3.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 4, tag: "MP3" }));
}

#[test]
fn duplicate_header_relative() {
    let txt = include_str!("txts/duplicate_header_relative.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 13, tag: "RELATIVE" }));
}

#[test]
fn duplicate_header_title() {
    let txt = include_str!("txts/duplicate_header_title.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 2, tag: "TITLE" }));
}

#[test]
fn duplicate_header_unknown() {
    let txt = include_str!("txts/duplicate_header_unknown.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 16, tag: "UNKNOWN" }));
}

#[test]
fn duplicate_header_video() {
    let txt = include_str!("txts/duplicate_header_video.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 11, tag: "VIDEO" }));
}

#[test]
fn duplicate_header_videogap() {
    let txt = include_str!("txts/duplicate_header_videogap.txt");
    assert_eq!(parse_txt_header_str(txt), Err(ParserError::DuplicateHeader { line: 12, tag: "VIDEOGAP" }));
}

fn get_simple_txt_str() -> &'static str {
    include_str!("txts/simple_txt_with_all_features.txt")
}

fn get_simple_txt_header() -> Header {
    Header {
        artist: String::from("Testartist"),
        title: String::from("Testsong"),
        bpm: 123.0,
        gap: 666.0,
        audio_path: PathBuf::from("Testfile.mp3"),
        relative: Some(false),
        video_path: Some(PathBuf::from("DLzxrzFCyOs.mp4")),
        cover_path: Some(PathBuf::from("Cover.jpg")),
        background_path: Some(PathBuf::from("BG.jpg")),
        video_gap: Some(777.0),
        genre: Some(String::from("Music")),
        edition: Some(String::from("Testmusic")),
        language: Some(String::from("en")),
        year: Some(1337),
        unknown: None,
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
