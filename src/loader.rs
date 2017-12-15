extern crate chardet;
extern crate encoding;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use structs::TXTSong;
use parser::{parse_txt_header_str, parse_txt_lines_str};

// TODO proper error handling
fn read_file_to_string<P: AsRef<Path>>(p: P) -> Result<String, String> {
    let p = p.as_ref();
    let mut f = match File::open(p) {
        Ok(x) => x,
        Err(e) => {
            let error = format!("error while opening file: {:?} ({:?})", e, p);
            return Err(error);
        }
    };
    let mut reader: Vec<u8> = Vec::new();
    if let Err(_) = f.read_to_end(&mut reader) {
        return Err(format!("ER: {:?}", f));
    }

    // detect encoding and decode to String
    let chardet_result = chardet::detect(&reader);
    let whtwg_label = chardet::charset2encoding(&chardet_result.0);
    let coder = encoding::label::encoding_from_whatwg_label(whtwg_label);
    let file_content = match coder {
        Some(c) => match c.decode(&reader, encoding::DecoderTrap::Ignore) {
            Ok(x) => x,
            Err(e) => {
                return Err(format!("EE: {:?} {:?}", e, p));
            }
        },
        None => return Err(format!("ED: {:?}", p)),
    };

    Ok(file_content)
}

fn canonicalize_path<P: AsRef<Path>, B: AsRef<Path>>(
    path: Option<P>,
    base_path: B,
) -> Option<PathBuf> {
    if let Some(ref path) = path {
        let mut tmp_path = PathBuf::from(base_path.as_ref());
        tmp_path.push(path);
        match tmp_path.canonicalize() {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    } else {
        None
    }
}

// TODO: return result with error
pub fn parse_txt_song<P: AsRef<Path>>(path: P) -> Option<TXTSong> {
    let path = path.as_ref();
    if let Ok(txt) = read_file_to_string(path) {
        let mut txt_song = TXTSong {
            header: match parse_txt_header_str(txt.as_ref()) {
                Ok(x) => x,
                Err(_) => return None,
            },
            lines: match parse_txt_lines_str(txt.as_ref()) {
                Ok(x) => x,
                Err(_) => return None,
            },
        };
        // canonicalize paths
        if let Some(base_path) = path.parent() {
            // canonicalize audio path
            txt_song.header.audio_path =
                match canonicalize_path(Some(txt_song.header.audio_path), base_path) {
                    Some(x) => x,
                    None => return None,
                };

            // canonicalize other path
            txt_song.header.video_path = canonicalize_path(txt_song.header.video_path, base_path);
            txt_song.header.cover_path = canonicalize_path(txt_song.header.cover_path, base_path);
            txt_song.header.background_path =
                canonicalize_path(txt_song.header.background_path, base_path);
        }
        Some(txt_song)
    } else {
        None
    }
}
