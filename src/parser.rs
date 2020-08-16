use crate::structs::{Header, Line, Note, Source};
use regex::Regex;
use std::collections::HashMap;

error_chain! {
    errors {
        #[doc="duplicate header tag was found"]
        DuplicateHeader(line: u32, tag: &'static str) {
            description("duplicate header")
            display("additional {} tag found in line: {}", line, tag)
        }
        #[doc="an essential header is missing"]
        MissingEssential(fields: Vec<String>) {
            description("essential header is missing")
            display("essential header is missing. Missing headers: {:?}", fields)
        }

        #[doc="value could not be parsed"]
        ValueError(line: u32, field: &'static str) {
            description("could not parse value")
            display("could not parse {} in line: {}", field, line)
        }
        #[doc="an unknown note type was found"]
        UnknownNoteType(line: u32) {
            description("unknown note type")
            display("unknown note type in line: {}", line)
        }
        #[doc="could not parse the line at all"]
        ParserFailure(line: u32) {
            description("could not parse line")
            display("could not parse line: {}", line)
        }
        #[doc="song is missing the end terminator"]
        MissingEndIndicator {
            description("missing end indicator")
        }
        #[doc="song file uses a feature that is not implemented"]
        NotImplemented(line: u32, feature: &'static str) {
            description("not implemented")
            display("the feature {} in line {} is not implemented", line, feature)
        }
    }
}

/// Parses the Header of a given Ultrastar Song and returns a Header struct
///
/// # Arguments
/// * txt_str  - a &str that contains the song to parse
///
pub fn parse_txt_header_str(txt_str: &str) -> Result<Header> {
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
        static ref RE: Regex = Regex::new(r"#([A-Z1-3a-z]*):(.*)").unwrap();
    }

    for (line, line_count) in txt_str.lines().zip(1..) {
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
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "TITLE"));
                }
            }
            "ARTIST" => {
                if opt_artist.is_none() {
                    opt_artist = Some(String::from(value));
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "ARTIST"));
                }
            }
            "MP3" => {
                if opt_audio_path.is_none() {
                    opt_audio_path = Some(Source::parse(value));
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "MP3"));
                }
            }
            "BPM" => {
                if opt_bpm.is_none() {
                    opt_bpm = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => {
                            bail!(ErrorKind::ValueError(line_count, "BPM"));
                        }
                    };
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "BPM"));
                }
            }

            // Optional Header fields
            "GAP" => {
                if opt_gap.is_none() {
                    opt_gap = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => {
                            bail!(ErrorKind::ValueError(line_count, "GAP"));
                        }
                    };
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "GAP"));
                }
            }
            "COVER" => {
                if opt_cover_path.is_none() {
                    opt_cover_path = Some(Source::parse(value));
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "COVER"));
                }
            }
            "BACKGROUND" => {
                if opt_background_path.is_none() {
                    opt_background_path = Some(Source::parse(value));
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "BACKGROUND"));
                }
            }
            "VIDEO" => {
                if opt_video_path.is_none() {
                    opt_video_path = Some(Source::parse(value));
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "VIDEO"));
                }
            }
            "VIDEOGAP" => {
                if opt_video_gap.is_none() {
                    opt_video_gap = match value.replace(",", ".").parse() {
                        Ok(x) => Some(x),
                        Err(_) => {
                            bail!(ErrorKind::ValueError(line_count, "VIDEOGAP"));
                        }
                    };
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "VIDEOGAP"));
                }
            }
            "GENRE" => {
                if opt_genre.is_none() {
                    opt_genre = Some(String::from(value));
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "GENRE"));
                }
            }
            "EDITION" => {
                if opt_edition.is_none() {
                    opt_edition = Some(String::from(value));
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "EDITION"));
                }
            }
            "LANGUAGE" => {
                if opt_language.is_none() {
                    opt_language = Some(String::from(value));
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "LANGUAGE"));
                }
            }
            "YEAR" => {
                if opt_year.is_none() {
                    opt_year = match value.parse() {
                        Ok(x) => Some(x),
                        Err(_) => {
                            bail!(ErrorKind::ValueError(line_count, "YEAR"));
                        }
                    };
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "YEAR"));
                }
            }
            //TODO: check if relative changes line breaks
            "RELATIVE" => {
                if opt_relative.is_none() {
                    opt_relative = match value {
                        "YES" | "yes" => Some(true),
                        "NO" | "no" => Some(false),
                        _ => {
                            bail!(ErrorKind::ValueError(line_count, "RELATIVE"));
                        }
                    }
                } else {
                    bail!(ErrorKind::DuplicateHeader(line_count, "RELATIVE"));
                }
            }
            // use hashmap to store unknown tags
            k => {
                opt_unknown = match opt_unknown {
                    Some(mut x) => {
                        if !x.contains_key(k) {
                            x.insert(String::from(k), String::from(value));
                            Some(x)
                        } else {
                            bail!(ErrorKind::DuplicateHeader(line_count, "UNKNOWN"));
                        }
                    }
                    None => {
                        let mut unknown = HashMap::new();
                        unknown.insert(String::from(k), String::from(value));
                        Some(unknown)
                    }
                };
            }
        };
    }

    // build header from Options
    if let (Some(title), Some(artist), Some(bpm), Some(audio_path)) =
        (opt_title.clone(), opt_artist.clone(), opt_bpm.clone(), opt_audio_path.clone())
    {
        let header = Header {
            title,
            artist,
            bpm,
            audio_path,

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
    } else {
        let mut fields = Vec::new();
        if opt_title.is_none() { fields.push("Title".to_string()) }
        if opt_artist.is_none() { fields.push("Artist".to_string()) }
        if opt_bpm.is_none() { fields.push("BPM".to_string()) }
        if opt_audio_path.is_none() { fields.push("Audio Path".to_string()) }
        // essential field is missing
        bail!(ErrorKind::MissingEssential(fields))
    }
}

/// Parses the lyric lines of a given Ultarstar song and returns a vector of Line structs
///
/// # Arguments
/// * txt_str  - a &str that contains the song to parse
///
pub fn parse_txt_lines_str(txt_str: &str) -> Result<Vec<Line>> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new("^-\\s?(-?[0-9]+)\\s*$").unwrap();
        static ref LREL_RE: Regex = Regex::new("^-\\s?(-?[0-9]+)\\s+(-?[0-9]+)").unwrap();
        static ref NOTE_RE: Regex =
            Regex::new("^(.)\\s*(-?[0-9]+)\\s+(-?[0-9]+)\\s+(-?[0-9]+)\\s?(.*)").unwrap();
        static ref DUET_RE: Regex = Regex::new("^P\\s?(-?[0-9]+)").unwrap();
    }

    let mut lines_vec = Vec::new();
    let mut current_line = Line {
        start: 0,
        rel: None,
        notes: Vec::new(),
    };

    let mut found_end_indicator = false;
    for (line, line_count) in txt_str.lines().zip(1..) {
        let first_char = match line.chars().next() {
            Some(x) => x,
            None => bail!(ErrorKind::ParserFailure(line_count)),
        };

        // ignore header
        if first_char == '#' {
            continue;
        }

        // not implemented
        if first_char == 'B' {
            bail!(ErrorKind::NotImplemented(line_count, "variable bpm"));
        }

        // stop parsing after end symbol
        if first_char == 'E' {
            lines_vec.push(current_line);
            found_end_indicator = true;
            break;
        }

        // current line is a note
        if NOTE_RE.is_match(line) {
            let cap = NOTE_RE.captures(line).unwrap();

            let note_start = match cap.get(2).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => {
                    bail!(ErrorKind::ValueError(line_count, "note start"));
                }
            };
            let note_duration = match cap.get(3).unwrap().as_str().parse() {
                Ok(x) => {
                    if x >= 0 {
                        x
                    } else {
                        bail!(ErrorKind::ValueError(line_count, "note duration"));
                    }
                }
                Err(_) => {
                    bail!(ErrorKind::ValueError(line_count, "note duration"));
                }
            };
            let note_pitch = match cap.get(4).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => {
                    bail!(ErrorKind::ValueError(line_count, "note pitch"));
                }
            };
            let note_text = cap.get(5).unwrap().as_str();

            let note = match cap.get(1).unwrap().as_str() {
                ":" => Note::Regular {
                    start: note_start,
                    duration: note_duration,
                    pitch: note_pitch,
                    text: String::from(note_text),
                },
                "*" => Note::Golden {
                    start: note_start,
                    duration: note_duration,
                    pitch: note_pitch,
                    text: String::from(note_text),
                },
                "F" => Note::Freestyle {
                    start: note_start,
                    duration: note_duration,
                    pitch: note_pitch,
                    text: String::from(note_text),
                },
                _ => bail!(ErrorKind::UnknownNoteType(line_count)),
            };

            current_line.notes.push(note);
            continue;
        }

        // current line is a line break
        if LINE_RE.is_match(line) {
            // push old line to the Line vector and prepare new line
            lines_vec.push(current_line);
            let cap = LINE_RE.captures(line).unwrap();
            let line_start = match cap.get(1).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => {
                    bail!(ErrorKind::ValueError(line_count, "line start"));
                }
            };
            current_line = Line {
                start: line_start,
                rel: None,
                notes: Vec::new(),
            };
            continue;
        }

        // current line is a relative line break
        if LREL_RE.is_match(line) {
            // push old line to the Line vector and prepare new line
            lines_vec.push(current_line);
            let cap = LREL_RE.captures(line).unwrap();
            let line_start = match cap.get(1).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => {
                    bail!(ErrorKind::ValueError(line_count, "line start"));
                }
            };
            let line_rel = match cap.get(2).unwrap().as_str().parse() {
                Ok(x) => x,
                Err(_) => {
                    bail!(ErrorKind::ValueError(line_count, "line rel"));
                }
            };
            current_line = Line {
                start: line_start,
                rel: Some(line_rel),
                notes: Vec::new(),
            };
            continue;
        }

        if DUET_RE.is_match(line) {
            let cap = DUET_RE.captures(line).unwrap();
            let note = match cap.get(1).unwrap().as_str().parse() {
                Ok(x) => {
                    if x >= 1 && x <= 3 {
                        Note::PlayerChange { player: x }
                    } else {
                        bail!(ErrorKind::ValueError(line_count, "player change"));
                    }
                }
                Err(_) => {
                    bail!(ErrorKind::ValueError(line_count, "player change"));
                }
            };
            current_line.notes.push(note);
            continue;
        } else {
            // unknown line
            bail!(ErrorKind::ParserFailure(line_count));
        }
    }
    if found_end_indicator {
        Ok(lines_vec)
    } else {
        bail!(ErrorKind::MissingEndIndicator);
    }
}
