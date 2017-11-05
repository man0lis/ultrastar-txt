#[macro_use] extern crate lazy_static;
extern crate regex;

use std::path::PathBuf;
use regex::Regex;
use std::fmt;

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
    pub relative: Option<bool>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Song {
    pub header: Header,
    pub lines: Vec<Line>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ParserError {
    DuplicateHeader{line: u32, tag: String},
    MissingEssential,
    ValueError{line: u32, field: String},
    UnknownNoteType{line: u32, note: String},
    ParserFailure{line: u32}
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_msg = match *self {
            ParserError::DuplicateHeader{ref line, ref tag} => format!("additional {} tag found in line: {}", tag, line),
            ParserError::MissingEssential => format!("one or more essential headers are missing"),
            ParserError::ValueError{ref line, ref field} => format!("could not parse {} in line: {}", field, line),
            ParserError::UnknownNoteType{ref line, ref note} => format!("unknown note type '{}' in line: {}", note, line),
            ParserError::ParserFailure{line} => format!("could not parse line: {}", line),
        };
        write!(f, "{}", error_msg)
    }
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

pub fn parse_txt_header_str(txt_str: &str) -> Result<Header, ParserError> {

    let mut opt_title = None;
    let mut opt_artist = None;
    let mut opt_bpm = None;
    let mut opt_gap = None;
    let mut opt_audio_path = None;

    let mut opt_cover_path = None;
    let mut opt_background_path = None;
    let mut opt_video_path = None;
    let mut opt_video_gap = None;
    let mut opt_genre = None;
    let mut opt_edition = None;
    let mut opt_language = None;
    let mut opt_year = None;
    let mut opt_relative = None;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"#([A-Z3]*):(.*)").unwrap();
    }

    let mut line_count = 0;
    for line in txt_str.lines() {
        line_count += 1;
        let cap = match RE.captures(line) {
            Some(x) => x,
            None => break,
        };
        let key = cap.get(1).unwrap().as_str();
        let value = cap.get(2).unwrap().as_str();
        match key {
            "TITLE" => {
                if opt_title.is_none() {
                    opt_title = Some(String::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("TITLE")});
                }
            },
            "ARTIST" => {
                if opt_artist.is_none() {
                    opt_artist = Some(String::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("ARTIST")});
                }
            },
            "MP3" => {
                if opt_audio_path.is_none() {
                    opt_audio_path = Some(PathBuf::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("MP3")});
               }
            },
            "GAP" => {
                if opt_gap.is_none() {
                    opt_gap = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => return Err(ParserError::ValueError{line: line_count, field: String::from("GAP")}),
                    };
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("GAP")});
                }
            },
            "BPM" => {
                if opt_bpm.is_none() {
                    opt_bpm = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => return Err(ParserError::ValueError{line: line_count, field: String::from("BPM")}),
                    };
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("BPM")});
                }
            },

            // Optional Header fields
            "COVER" => {
                if opt_cover_path.is_none() {
                    opt_cover_path = Some(PathBuf::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("COVER")});
                }
            },
            "BACKGROUND" => {
                if opt_background_path.is_none() {
                    opt_background_path = Some(PathBuf::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("BACKGROUND")});
                }
            },
            "VIDEO" => {
                if opt_video_path.is_none() {
                    opt_video_path = Some(PathBuf::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("VIDEO")});
                }
            },
            "VIDEOGAP" => {
                if opt_video_gap.is_none() {
                    opt_video_gap = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => return Err(ParserError::ValueError{line: line_count, field: String::from("VIDEOGAP")}),
                    };
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("VIDEOGAP")});
                }
            },
            "GENRE" => {
                if opt_genre.is_none() {
                    opt_genre = Some(String::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("GENRE")});
                }
            },
            "EDITION" => {
                if opt_edition.is_none() {
                    opt_edition = Some(String::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("EDITION")});
                }
            },
            "LANGUAGE" => {
                if opt_language.is_none() {
                    opt_language = Some(String::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("LANGUAGE")});
                }
            },
            "YEAR" => {
                if opt_year.is_none() {
                    opt_year = match value.parse() {
                        Ok(x) => Some(x),
                        Err(_) => return Err(ParserError::ValueError{line: line_count, field: String::from("YEAR")}),
                    };
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("YEAR")});
                }
            },
            "RELATIVE" => {
                if opt_relative.is_none() {
                    opt_relative = match value {
                        "YES" => Some(true),
                        "NO" => Some(false),
                        _ => return Err(ParserError::ValueError{line: line_count, field: String::from("RELATIVE")}),
                    }
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: String::from("RELATIVE")});
                }
            },
            // TODO: use hashmap to store unknown tags
            _ => println!("Warning: unknown tag {} found in line: {}", key, line_count),
        };

    }

    // build header from Options
    if let (Some(title), Some(artist), Some(bpm), Some(gap), Some(auto_path)) = (opt_title, opt_artist, opt_bpm, opt_gap, opt_audio_path) {
        let header = Header {
            title: title,
            artist: artist,
            bpm: bpm,
            gap: gap,
            audio_path: auto_path,

            cover_path: opt_cover_path,
            background_path: opt_background_path,
            video_path: opt_video_path,
            video_gap: opt_video_gap,
            genre: opt_genre,
            edition: opt_edition,
            language: opt_language,
            year: opt_year,
            relative: opt_relative,
        };
        // header complete
        Ok(header)
    }
    else {
        // essential field is missing
        Err(ParserError::MissingEssential)
    }
}

pub fn parse_txt_lines_str(txt_str: &str) -> Result<Vec<Line>, String> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new("- ([0-9]*)").unwrap();
        //TODO: figure out if some of these numbers can be negative (should not, but there might be strange txts)
        static ref NOTE_RE: Regex = Regex::new("([:*F]) ([0-9]*) ([0-9]*) ([0-9]*) (.*)").unwrap();
    }

    let mut lines_vec = Vec::new();
    let mut current_line = Line {
        start: 0,
        notes: Vec::new(),
    };

    let mut line_count = 0;
    for line in txt_str.lines() {
        line_count += 1;

        let first_char = match line.chars().nth(0) {
            Some(x) => x,
            None => return Err(format!("Could not parse line: {}", line_count)),
        };

        // ignore header
        if first_char == '#' {
            continue;
        }

        // stop parsing after end symbol
        if first_char == 'E' {
            lines_vec.push(current_line);
            break;
        }

        // current line is a line break
        if LINE_RE.is_match(line) {
            // push old line to the Line vector and prepare new line
            lines_vec.push(current_line);
            let cap = LINE_RE.captures(line).unwrap();
            let line_start = match cap.get(1).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => return Err(format!("Could not parse line start in line: {}", line_count)),
            };
            current_line = Line {
                start: line_start,
                notes: Vec::new(),
            };
            continue;
        }

        // current line is a note
        if NOTE_RE.is_match(line) {
            let cap = NOTE_RE.captures(line).unwrap();
            let note_type = match cap.get(1).unwrap().as_str() {
                ":" => NoteType::Regular,
                "*" => NoteType::Golden,
                "F" => NoteType::Freestyle,
                _ => return Err(format!("Could not parse note type in line: {}", line_count)),
            };
            let note_start = match cap.get(2).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => return Err(format!("Could not parse note start in line: {}", line_count)),
            };
            let note_duration = match cap.get(3).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => return Err(format!("Could not parse note duration in line: {}", line_count)),
            };
            let note_pitch = match cap.get(4).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => return Err(format!("Could not parse note pitch in line: {}", line_count)),
            };
            let note_text = cap.get(5).unwrap().as_str();

            let note = Note {
                notetype: note_type,
                start: note_start,
                duration: note_duration,
                pitch: note_pitch,
                text: String::from(note_text),
            };
            current_line.notes.push(note);
        }
        // unknown line
        else {
            return Err(format!("Could not parse line: {}", line_count));
        }

    }
    Ok(lines_vec)

}
