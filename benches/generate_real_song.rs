#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate ultrastar_txt;

use ultrastar_txt::*;

fn bench_generate_real_song_str(c: &mut Criterion) {
    let txt = get_real_txt_str();
    let header = parse_txt_header_str(txt).unwrap();
    let lines = parse_txt_lines_str(txt).unwrap();
    c.bench_function("generate_real_song", move |b| {
        b.iter(|| generate_song_txt(&header, &lines).unwrap())
    });
}

criterion_group!(benches, bench_generate_real_song_str);
criterion_main!(benches);

fn get_real_txt_str() -> &'static str {
    include_str!("Pornophonique - Space Invaders.txt")
}
