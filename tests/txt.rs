extern crate ultrastar_txt;

use std::collections::HashMap;
use ultrastar_txt::*;
use url::Url;

// usage:
//    assert_error_kind!(some_err, ErrorKind::MyErrorType)
macro_rules! assert_error_kind {
    ($err:expr, $kind:pat) => {
        match *$err.kind() {
            $kind => assert!(true, "{:?} is of kind {:?}", $err, stringify!($kind)),
            _ => assert!(false, "{:?} is NOT of kind {:?}", $err, stringify!($kind)),
        }
    };
}

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
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::MissingEssential
    );
}

#[test]
fn value_error_in_header_bpm() {
    let txt = include_str!("txts/value_error_in_header_bpm.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::ValueError(5, "BPM")
    );
}

#[test]
fn value_error_in_header_gap() {
    let txt = include_str!("txts/value_error_in_header_gap.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::ValueError(4, "GAP")
    );
}

#[test]
fn value_error_in_header_videogap() {
    let txt = include_str!("txts/value_error_in_header_videogap.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::ValueError(6, "VIDEOGAP")
    );
}

#[test]
fn value_error_in_header_year() {
    let txt = include_str!("txts/value_error_in_header_year.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::ValueError(7, "YEAR")
    );
}

#[test]
fn unknown_note_type() {
    let txt = include_str!("txts/unknown_note_type.txt");
    assert_error_kind!(
        parse_txt_lines_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::UnknownNoteType(7)
    );
}

#[test]
fn garbage_line() {
    let txt = include_str!("txts/garbage_line.txt");
    assert_error_kind!(
        parse_txt_lines_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::ParserFailure(7)
    );
}

#[test]
fn duplicate_header_artist() {
    let txt = include_str!("txts/duplicate_header_artist.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(3, "ARTIST")
    );
}

#[test]
fn duplicate_header_background() {
    let txt = include_str!("txts/duplicate_header_background.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(10, "BACKGROUND")
    );
}

#[test]
fn duplicate_header_bpm() {
    let txt = include_str!("txts/duplicate_header_bpm.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(6, "BPM")
    );
}

#[test]
fn duplicate_header_cover() {
    let txt = include_str!("txts/duplicate_header_cover.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(9, "COVER")
    );
}

#[test]
fn duplicate_header_edition() {
    let txt = include_str!("txts/duplicate_header_edition.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(8, "EDITION")
    );
}

#[test]
fn duplicate_header_gap() {
    let txt = include_str!("txts/duplicate_header_gap.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(5, "GAP")
    );
}

#[test]
fn duplicate_header_genre() {
    let txt = include_str!("txts/duplicate_header_genre.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(7, "GENRE")
    );
}

#[test]
fn duplicate_header_language() {
    let txt = include_str!("txts/duplicate_header_language.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(14, "LANGUAGE")
    );
}

#[test]
fn duplicate_header_mp3() {
    let txt = include_str!("txts/duplicate_header_mp3.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(4, "MP3")
    );
}

#[test]
fn duplicate_header_relative() {
    let txt = include_str!("txts/duplicate_header_relative.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(13, "RELATIVE")
    );
}

#[test]
fn duplicate_header_title() {
    let txt = include_str!("txts/duplicate_header_title.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(2, "TITLE")
    );
}

#[test]
fn duplicate_header_unknown() {
    let txt = include_str!("txts/duplicate_header_unknown.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(16, "UNKNOWN")
    );
}

#[test]
fn duplicate_header_video() {
    let txt = include_str!("txts/duplicate_header_video.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(11, "VIDEO")
    );
}

#[test]
fn duplicate_header_videogap() {
    let txt = include_str!("txts/duplicate_header_videogap.txt");
    assert_error_kind!(
        parse_txt_header_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::DuplicateHeader(12, "VIDEOGAP")
    );
}

#[test]
fn missing_end_indicator() {
    let txt = include_str!("txts/missing_end.txt");
    assert_error_kind!(
        parse_txt_lines_str(txt).err().unwrap(),
        ultrastar_txt::parser::ErrorKind::MissingEndIndicator
    );
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
fn negative_line_break() {
    let txt = include_str!("txts/negative_line_break.txt");
    assert!(parse_txt_lines_str(txt).is_ok());
}

#[test]
fn missing_space_in_line_break() {
    let txt = include_str!("txts/missing_space_in_line_break.txt");
    assert!(parse_txt_lines_str(txt).is_ok());
}

#[test]
fn allow_multiple_spaces_between_line_values() {
    let txt = include_str!("txts/allow_multiple_spaces_between_line_values.txt");
    let lines = get_simple_txt_lines();
    assert_eq!(lines, parse_txt_lines_str(txt).unwrap());
}

#[test]
fn lower_case_relative() {
    let txt = include_str!("txts/lower_case_relative.txt");
    assert!(parse_txt_header_str(txt).is_ok());
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
fn generate_and_reparse_song() {
    let orig_txt_header = get_simple_txt_header();
    let orig_txt_lines = get_simple_txt_lines();

    let generated_txt = generate_song_txt(&orig_txt_header, &orig_txt_lines).unwrap();
    let parsed_header = parse_txt_header_str(generated_txt.as_ref()).unwrap();
    let parsed_lines = parse_txt_lines_str(generated_txt.as_ref()).unwrap();

    assert_eq!(parsed_header, orig_txt_header);
    assert_eq!(parsed_lines, orig_txt_lines);
}

#[test]
fn relative_line_breaks() {
    let txt = include_str!("txts/relative_line_breaks.txt");
    let lines = parse_txt_lines_str(txt).unwrap();
    assert_eq!(lines[1].rel.unwrap(), 24);
}

#[test]
fn remote_url_audio() {
    let txt = include_str!("txts/remote_url_as_path.txt");
    let mut header = get_simple_txt_header();
    header.audio_path = Source::parse("https://www.example.com/Testfile.mp3");
    assert_eq!(parse_txt_header_str(txt).unwrap(), header);
    assert_eq!(Source::parse("https://www.example.com/Testfile.mp3"), 
               Source::Remote(Url::parse("https://www.example.com/Testfile.mp3").unwrap()));
}

fn get_simple_txt_str() -> &'static str {
    include_str!("txts/simple_txt_with_all_features.txt")
}

fn get_simple_txt_header() -> Header {
    Header {
        artist: String::from("Testartist"),
        title: String::from("Testsong"),
        bpm: 123.0,
        audio_path: Source::parse("Testfile.mp3"),
        gap: Some(666.0),
        relative: Some(false),
        video_path: Some(Source::parse("DLzxrzFCyOs.mp4")),
        cover_path: Some(Source::parse("Cover.jpg")),
        background_path: Some(Source::parse("BG.jpg")),
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
            rel: None,
            notes: vec![
                Note::Regular {
                    start: 0,
                    duration: 4,
                    pitch: 59,
                    text: String::from("Test "),
                },
                Note::Regular {
                    start: 4,
                    duration: 4,
                    pitch: 59,
                    text: String::from("I"),
                },
                Note::Regular {
                    start: 8,
                    duration: 4,
                    pitch: 59,
                    text: String::from("'m "),
                },
                Note::Golden {
                    start: 12,
                    duration: 4,
                    pitch: 59,
                    text: String::from("test"),
                },
                Note::Regular {
                    start: 16,
                    duration: 4,
                    pitch: 59,
                    text: String::from("ing."),
                },
            ],
        },
        Line {
            start: 20,
            rel: None,
            notes: vec![
                Note::Regular {
                    start: 24,
                    duration: 4,
                    pitch: 59,
                    text: String::from("Test "),
                },
                Note::Regular {
                    start: 28,
                    duration: 4,
                    pitch: 59,
                    text: String::from("I"),
                },
                Note::Regular {
                    start: 32,
                    duration: 4,
                    pitch: 59,
                    text: String::from("'m "),
                },
                Note::Freestyle {
                    start: 36,
                    duration: 4,
                    pitch: 59,
                    text: String::from("test"),
                },
                Note::Freestyle {
                    start: 40,
                    duration: 4,
                    pitch: 59,
                    text: String::from("ing."),
                },
            ],
        },
    ]
}
