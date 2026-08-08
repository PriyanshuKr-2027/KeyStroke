use criterion::{black_box, criterion_group, criterion_main, Criterion};
use keymind_autocorrect::AutocorrectEngine;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

fn bench_autocorrect_pipeline(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let engine = rt.block_on(async {
        let tmp_path = std::env::temp_dir().join(format!("test_autocorrect_bench_{}.json", std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos()));
        let engine = AutocorrectEngine::new(tmp_path);
        engine.initialize().await.unwrap();
        engine.add_to_personal_dict("customkeyword");
        engine.record_user_correction_in_memory("teh", "the", 3);
        engine
    });

    let mut group = c.benchmark_group("Autocorrect Engine Latency");

    group.bench_function("check_personal_dictionary_l1", |b| {
        b.iter(|| {
            black_box(engine.check(black_box("customkeyword"), black_box("testing ")))
        });
    });

    group.bench_function("check_learned_correction_l1_5", |b| {
        b.iter(|| {
            black_box(engine.check(black_box("teh"), black_box("in ")))
        });
    });

    group.bench_function("check_homophone_pattern_l3", |b| {
        b.iter(|| {
            black_box(engine.check(black_box("their"), black_box("going over ")))
        });
    });

    group.bench_function("check_symspell_typo_l2", |b| {
        b.iter(|| {
            black_box(engine.check(black_box("taht"), black_box("it is ")))
        });
    });

    group.finish();
}

criterion_group!(benches, bench_autocorrect_pipeline);
criterion_main!(benches);
