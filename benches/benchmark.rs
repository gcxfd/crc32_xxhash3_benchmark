use std::hash::Hasher;

use crc::{Crc, CRC_32_ISCSI};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use twox_hash::XxHash32;

fn crc32fast(c: &mut Criterion) {
    let bin = rand::thread_rng().gen::<[u8; 64]>();
    c.bench_function("crc32fast", |b| b.iter(|| crc32fast::hash(black_box(&bin))));
}

const CRC32_CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

fn crc32(c: &mut Criterion) {
    let bin = rand::thread_rng().gen::<[u8; 64]>();
    c.bench_function("crc32", |b| {
        b.iter(|| CRC32_CASTAGNOLI.checksum(black_box(&bin)))
    });
}

fn xxhash32(c: &mut Criterion) {
    let bin = rand::thread_rng().gen::<[u8; 64]>();
    c.bench_function("xxhash32", |b| {
        b.iter(|| {
            let mut xxhash32: XxHash32 = XxHash32::with_seed(0);
            xxhash32.write(black_box(&bin));
            black_box(xxhash32.finish() as u32);
        })
    });
}

criterion_group!(benches, crc32fast, crc32, xxhash32);
criterion_main!(benches);
