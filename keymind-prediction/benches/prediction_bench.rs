use criterion::{black_box, criterion_group, criterion_main, Criterion};
use keymind_prediction::{init_trigram_table, load_bundled_trigrams, query_trigrams, BUNDLED_TRIGRAMS};
use sqlx::sqlite::SqlitePoolOptions;
use tokio::runtime::Runtime;

fn bench_trigram_lookup(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let db = rt.block_on(async {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        init_trigram_table(&pool).await.unwrap();
        load_bundled_trigrams(&pool, BUNDLED_TRIGRAMS).await.unwrap();
        pool
    });

    c.bench_function("trigram_lookup_sub_1ms", |b| {
        b.to_async(&rt).iter(|| async {
            let (suggestions, conf) = query_trigrams(&db, black_box("in"), black_box("the")).await.unwrap();
            black_box((suggestions, conf));
        })
    });
}

criterion_group!(benches, bench_trigram_lookup);
criterion_main!(benches);
