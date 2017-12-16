extern crate chardet;
extern crate encoding;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use structs::TXTSong;
use parser::{parse_txt_header_str, parse_txt_lines_str};

error_chain!{
    errors {
        IOError {
            description("io error")
        }
        EncodingDetectionError {
            description("encoding detection error")
        }
        DecodingError(msg: String) {
            description("decoding error")
            display("decoding error: {}", msg)
        }
        CanonicalizationError {
            description("canonicalization error")
        }
        HeaderParsingError {
            description("header parsing error")
        }
        LinesParsingError {
            description("lines parsing error")
        }
    }
}

fn read_file_to_string<P: AsRef<Path>>(p: P) -> Result<String> {
    let p = p.as_ref();
    let mut f = File::open(p).chain_err(|| ErrorKind::IOError)?;
    let mut reader: Vec<u8> = Vec::new();
    f.read_to_end(&mut reader).chain_err(|| ErrorKind::IOError)?;

    // detect encoding and decode to String
    let chardet_result = chardet::detect(&reader);
    let whtwg_label = chardet::charset2encoding(&chardet_result.0);
    let coder = encoding::label::encoding_from_whatwg_label(whtwg_label);
    let file_content = match coder {
        Some(c) => match c.decode(&reader, encoding::DecoderTrap::Ignore) {
            Ok(x) => x,
            Err(e) => bail!(ErrorKind::DecodingError(e.into_owned())),
        },
        None => bail!(ErrorKind::EncodingDetectionError),
    };

    Ok(file_content)
}

fn canonicalize_path<P: AsRef<Path>, B: AsRef<Path>>(
    path: Option<P>,
    base_path: B,
) -> Result<Option<PathBuf>> {
    Ok(if let Some(ref path) = path {
        let mut tmp_path = PathBuf::from(base_path.as_ref());
        tmp_path.push(path);
        let result = tmp_path
            .canonicalize()
            .chain_err(|| ErrorKind::CanonicalizationError)?;
        Some(result)
    } else {
        None
    })
}

pub fn parse_txt_song<P: AsRef<Path>>(path: P) -> Result<TXTSong> {
    let path = path.as_ref();
    let txt = read_file_to_string(path)?;

    let mut txt_song = TXTSong {
        header: parse_txt_header_str(txt.as_ref()).chain_err(|| ErrorKind::HeaderParsingError)?,
        lines: parse_txt_lines_str(txt.as_ref()).chain_err(|| ErrorKind::LinesParsingError)?,
    };

    // canonicalize paths
    if let Some(base_path) = path.parent() {
        // canonicalize audio path
        txt_song.header.audio_path =
            canonicalize_path(Some(txt_song.header.audio_path), base_path)?.unwrap();

        // canonicalize other path
        txt_song.header.video_path = canonicalize_path(txt_song.header.video_path, base_path)?;
        txt_song.header.cover_path = canonicalize_path(txt_song.header.cover_path, base_path)?;
        txt_song.header.background_path =
            canonicalize_path(txt_song.header.background_path, base_path)?;
    }

    Ok(txt_song)
}
