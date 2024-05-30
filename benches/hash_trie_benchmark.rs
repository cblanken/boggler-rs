use boggler_rs::tries::hash_trie::HashTrie;
use boggler_rs::tries::traits::WordTree;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{fs, time::Duration};

pub fn hash_trie_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("load_dictionary");
    group.bench_function("large", |b| {
        let dict_file =
            fs::read_to_string("dictionaries/scrabble_2019.txt").expect("Could not read file");
        let dict = dict_file.split_whitespace().map(|w| w.chars());
        b.iter(|| {
            HashTrie::build(black_box(dict.clone()));
        })
    });

    group
        .significance_level(0.5)
        .sample_size(50)
        .measurement_time(Duration::new(30, 0));
    group.finish();
}

criterion_group!(benches, hash_trie_benchmarks);
criterion_main!(benches);
