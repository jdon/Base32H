use base32h::{decode_string, decode_string_to_binary, encode_binary_to_string, encode_to_string};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn encode(input: u128) -> String {
    encode_to_string(input).unwrap()
}

fn encode_bin(input: &[u8]) -> String {
    encode_binary_to_string(input).unwrap()
}

fn decode(input: &str) -> u128 {
    decode_string(input)
}

fn decode_bin(input: &str) -> Vec<u8> {
    decode_string_to_binary(input)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Encode digit", |b| b.iter(|| encode(black_box(4294967295))));
    c.bench_function("decode digit", |b| b.iter(|| decode(black_box("3zZzZzZ"))));

    c.bench_function("encode binary", |b| {
        b.iter(|| {
            encode_bin(black_box(&[
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            ]))
        })
    });
    c.bench_function("decode binary", |b| {
        b.iter(|| decode_bin(black_box("zZzZzZzZzZzZzZzZ")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
