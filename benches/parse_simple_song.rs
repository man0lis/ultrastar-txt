#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate ultrastar_txt;

use ultrastar_txt::*;

fn bench_parse_simple_song_str(c: &mut Criterion) {
    let txt = get_simple_txt_str();
    c.bench_function("parse_simple_song", move |b| {
        b.iter(|| {
            let header = parse_txt_header_str(txt).unwrap();
            let lines = parse_txt_lines_str(txt).unwrap();
            (header, lines)
        })
    });
}

criterion_group!(benches, bench_parse_simple_song_str);
criterion_main!(benches);

fn get_simple_txt_str() -> &'static str {
    include_str!("simple_txt_with_all_features.txt")
}
