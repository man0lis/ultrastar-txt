#![feature(test)]

extern crate test;
extern crate ultrastar_txt;

use ultrastar_txt::*;
use test::{Bencher, black_box};

#[bench]
fn bench_parse_simple_song_str(b: &mut Bencher) {
    let txt = get_simple_txt_str();
    b.iter(|| {
        let header = parse_txt_header_str(txt).unwrap();
        let lines = parse_txt_lines_str(txt).unwrap();
        black_box((header,lines));
    });
}

#[bench]
fn bench_parse_real_song_str(b: &mut Bencher) {
    let txt = get_real_txt_str();
    b.iter(|| {
        let header = parse_txt_header_str(txt).unwrap();
        let lines = parse_txt_lines_str(txt).unwrap();
        black_box((header,lines));
    });
}

fn get_simple_txt_str() -> &'static str {
    include_str!("simple_txt_with_all_features.txt")
}

fn get_real_txt_str() -> &'static str {
    include_str!("Pornophonique - Space Invaders.txt")
}
