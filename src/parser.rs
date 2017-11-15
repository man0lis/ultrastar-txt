use std::path::PathBuf;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use structs::*;

#[derive(PartialEq, Clone, Debug)]
pub enum ParserError {
    DuplicateHeader{line: u32, tag: &'static str},
    MissingEssential,
    ValueError{line: u32, field: &'static str},
    UnknownNoteType{line: u32},
    ParserFailure{line: u32}
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_msg = match *self {
            ParserError::DuplicateHeader{ref line, ref tag} => format!("additional {} tag found in line: {}", tag, line),
            ParserError::MissingEssential => format!("one or more essential headers are missing"),
            ParserError::ValueError{ref line, ref field} => format!("could not parse {} in line: {}", field, line),
            ParserError::UnknownNoteType{ref line} => format!("unknown note type in line: {}", line),
            ParserError::ParserFailure{line} => format!("could not parse line: {}", line),
        };
        write!(f, "{}", error_msg)
    }
}


pub fn parse_txt_header_str(txt_str: &str) -> Result<Header, ParserError> {

    let mut opt_title = None;
    let mut opt_artist = None;
    let mut opt_bpm = None;
    let mut opt_audio_path = None;

    let mut opt_gap = None;
    let mut opt_cover_path = None;
    let mut opt_background_path = None;
    let mut opt_video_path = None;
    let mut opt_video_gap = None;
    let mut opt_genre = None;
    let mut opt_edition = None;
    let mut opt_language = None;
    let mut opt_year = None;
    let mut opt_relative = None;
    let mut opt_unknown: Option<HashMap<String, String>> = None;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"#([A-Z3a-z]*):(.*)").unwrap();
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


        if value == "" {
            //TODO: somehow warn about this
            continue;
        }

        match key {
            "TITLE" => {
                if opt_title.is_none() {
                    opt_title = Some(String::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "TITLE"});
                }
            },
            "ARTIST" => {
                if opt_artist.is_none() {
                    opt_artist = Some(String::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "ARTIST"});
                }
            },
            "MP3" => {
                if opt_audio_path.is_none() {
                    opt_audio_path = Some(PathBuf::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "MP3"});
               }
            },
            "BPM" => {
                if opt_bpm.is_none() {
                    opt_bpm = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => return Err(ParserError::ValueError{line: line_count, field: "BPM"}),
                    };
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "BPM"});
                }
            },

            // Optional Header fields
            "GAP" => {
                if opt_gap.is_none() {
                    opt_gap = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => return Err(ParserError::ValueError{line: line_count, field: "GAP"}),
                    };
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "GAP"});
                }
            },
            "COVER" => {
                if opt_cover_path.is_none() {
                    opt_cover_path = Some(PathBuf::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "COVER"});
                }
            },
            "BACKGROUND" => {
                if opt_background_path.is_none() {
                    opt_background_path = Some(PathBuf::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "BACKGROUND"});
                }
            },
            "VIDEO" => {
                if opt_video_path.is_none() {
                    opt_video_path = Some(PathBuf::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "VIDEO"});
                }
            },
            "VIDEOGAP" => {
                if opt_video_gap.is_none() {
                    opt_video_gap = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => return Err(ParserError::ValueError{line: line_count, field: "VIDEOGAP"}),
                    };
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "VIDEOGAP"});
                }
            },
            "GENRE" => {
                if opt_genre.is_none() {
                    opt_genre = Some(String::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "GENRE"});
                }
            },
            "EDITION" => {
                if opt_edition.is_none() {
                    opt_edition = Some(String::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "EDITION"});
                }
            },
            "LANGUAGE" => {
                if opt_language.is_none() {
                    opt_language = Some(String::from(value));
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "LANGUAGE"});
                }
            },
            "YEAR" => {
                if opt_year.is_none() {
                    opt_year = match value.parse() {
                        Ok(x) => Some(x),
                        Err(_) => return Err(ParserError::ValueError{line: line_count, field: "YEAR"}),
                    };
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "YEAR"});
                }
            },
            //TODO: check if relative changes line breaks
            "RELATIVE" => {
                if opt_relative.is_none() {
                    opt_relative = match value {
                        "YES" | "yes" => Some(true),
                        "NO" | "no" => Some(false),
                        _ => return Err(ParserError::ValueError{line: line_count, field: "RELATIVE"}),
                    }
                }
                else {
                    return Err(ParserError::DuplicateHeader{line: line_count, tag: "RELATIVE"});
                }
            },
            // use hashmap to store unknown tags
            k => {
                opt_unknown = match opt_unknown {
                    Some(mut x) => if !x.contains_key(k) {
                        x.insert(String::from(k), String::from(value));
                        Some(x)
                    }
                    else {
                        return Err(ParserError::DuplicateHeader{line: line_count, tag: "UNKNOWN"});
                    },
                    None => {
                        let mut unknown = HashMap::new();
                        unknown.insert(String::from(k), String::from(value));
                        Some(unknown)
                    },
                };
            },
        };

    }

    // build header from Options
    if let (Some(title), Some(artist), Some(bpm), Some(auto_path)) = (opt_title, opt_artist, opt_bpm, opt_audio_path) {
        let header = Header {
            title: title,
            artist: artist,
            bpm: bpm,
            audio_path: auto_path,

            gap: opt_gap,
            cover_path: opt_cover_path,
            background_path: opt_background_path,
            video_path: opt_video_path,
            video_gap: opt_video_gap,
            genre: opt_genre,
            edition: opt_edition,
            language: opt_language,
            year: opt_year,
            relative: opt_relative,
            unknown: opt_unknown,
        };
        // header complete
        Ok(header)
    }
    else {
        // essential field is missing
        Err(ParserError::MissingEssential)
    }
}

pub fn parse_txt_lines_str(txt_str: &str) -> Result<Vec<Line>, ParserError> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new("^- ?(-?[0-9]+)").unwrap();
        //TODO: figure out if some of these numbers can be negative (should not, but there might be strange txts)
        static ref NOTE_RE: Regex = Regex::new("(.) *(-?[0-9]+) *([0-9]+) *(-?[0-9]+) (.*)").unwrap();
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
            None => return Err(ParserError::ParserFailure{line: line_count}),
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

        // ignore duett tags for now
        // TODO: implement duett
        if first_char == 'P' {
            continue;
        }

        // current line is a line break
        if LINE_RE.is_match(line) {
            // push old line to the Line vector and prepare new line
            lines_vec.push(current_line);
            let cap = LINE_RE.captures(line).unwrap();
            let line_start = match cap.get(1).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => return Err(ParserError::ValueError{line: line_count, field: "line start"}),
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
                _ => return Err(ParserError::UnknownNoteType{line: line_count}),
            };
            let note_start = match cap.get(2).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => return Err(ParserError::ValueError{line: line_count, field: "note start"}),
            };
            let note_duration = match cap.get(3).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => return Err(ParserError::ValueError{line: line_count, field: "note duration"}),
            };
            let note_pitch = match cap.get(4).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => return Err(ParserError::ValueError{line: line_count, field: "note pitch"}),
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
            return Err(ParserError::ParserFailure{line: line_count});
        }

    }
    Ok(lines_vec)

}
