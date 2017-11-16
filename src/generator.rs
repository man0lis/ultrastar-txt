use std::fmt;
use structs::*;

#[derive(PartialEq, Clone, Debug)]
pub enum GeneratorError {
    InvalidPath{tag: &'static str},
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_msg = match *self {
            GeneratorError::InvalidPath{ref tag} => format!("invalid path in tag {}", tag),
        };
        write!(f, "{}", error_msg)
    }
}

pub fn generate_song_txt(header: &Header, lines: &Vec<Line>) -> Result<String, GeneratorError> {
    // generate header
    let mp3_str = match header.audio_path.to_str() {
        Some(x) => x,
        None => return Err(GeneratorError::InvalidPath{tag: "MP3"}),
    };
    let mut song_txt_str = String::from(
        format!("#TITLE:{}\n#ARTIST:{}\n#MP3:{}\n#BPM:{}\n",
        header.title,
        header.artist,
        mp3_str,
        header.bpm
    ));
    if let Some(gap) = header.gap {
        song_txt_str.push_str(&format!("#GAP:{}\n",gap));
    }
    if let Some(cover_path) = header.cover_path.clone() {
        let cover_str = match cover_path.to_str() {
            Some(x) => x,
            None => return Err(GeneratorError::InvalidPath{tag: "COVER"}),
        };
        song_txt_str.push_str(&format!("#COVER:{}\n",cover_str));
    }
    if let Some(background_path) = header.background_path.clone() {
        let background_str = match background_path.to_str() {
            Some(x) => x,
            None => return Err(GeneratorError::InvalidPath{tag: "BACKGROUND"}),
        };
        song_txt_str.push_str(&format!("#BACKGROUND:{}\n",background_str));
    }
    if let Some(video_path) = header.video_path.clone() {
        let video_str = match video_path.to_str() {
            Some(x) => x,
            None => return Err(GeneratorError::InvalidPath{tag: "VIDEO"}),
        };
        song_txt_str.push_str(&format!("#VIDEO:{}\n",video_str));
    }
    if let Some(videogap) = header.video_gap {
        song_txt_str.push_str(&format!("#VIDEOGAP:{}\n",videogap));
    }
    if let Some(genre) = header.genre.clone() {
        song_txt_str.push_str(&format!("#GENRE:{}\n",genre));
    }
    if let Some(edition) = header.edition.clone() {
        song_txt_str.push_str(&format!("#EDITION:{}\n",edition));
    }
    if let Some(language) = header.language.clone() {
        song_txt_str.push_str(&format!("#LANGUAGE:{}\n",language));
    }
    if let Some(year) = header.year {
        song_txt_str.push_str(&format!("#YEAR:{}\n", year));
    }
    if let Some(relative) = header.relative {
        if relative {
            song_txt_str.push_str("#RELATIVE:YES\n");
        }
        else {
            song_txt_str.push_str("#RELATIVE:NO\n");
        }
    }
    if let Some(unknown) = header.unknown.clone() {
        for (key, value) in unknown.iter() {
            song_txt_str.push_str(&format!("#{}:{}\n", key, value));
        }
    }

    // generate lines
    for line in lines.iter() {
        if line.start != 0 {
            song_txt_str.push_str(format!("- {}\n", line.start).as_ref());
        }
        for note in line.notes.iter() {
            let notetype_symbol = match note.notetype {
                NoteType::Regular   => ":",
                NoteType::Golden    => "*",
                NoteType::Freestyle => "F",
            };
            song_txt_str.push_str(format!("{} {} {} {} {}\n", notetype_symbol, note.start, note.duration, note.pitch, note.text).as_ref());
        }
    }
    song_txt_str.push_str("E");
    Ok(song_txt_str)
}
