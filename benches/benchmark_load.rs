use async_filemanager::AsyncFileLoader;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use futures::stream::futures_unordered::FuturesUnordered;
use futures::stream::StreamExt;
use std::sync::Arc;
use threadpool::Builder;

async fn load_custom(f: &[&str], pool: Arc<threadpool::ThreadPool>) {
    let mut u = FuturesUnordered::new();
    for file in f.iter() {
        let pool = pool.clone();
        let mut path = String::from("benches/benchfiles/");
        path.push_str(file);
        let l = AsyncFileLoader::new(path, pool);
        u.push(l);
    }
    let mut vec = Vec::new();
    while let Some(val) = u.next().await {
        vec.push(val);
    }
}
async fn load_async(f: &[&str]) {
    let mut u = FuturesUnordered::new();
    for file in f.iter() {
        let mut path = String::from("benches/benchfiles/");
        path.push_str(file);
        let l = async_std::fs::read(path);
        u.push(l);
    }
    let mut vec = Vec::new();
    while let Some(val) = u.next().await {
        vec.push(val.unwrap());
    }
}
fn load_sync(f: &[&str]) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for file in f.iter() {
        let mut path = String::from("benches/benchfiles/");
        path.push_str(file);
        let l = std::fs::read(path).unwrap();
        vec.push(l);
    }
    vec
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cpu 1", |b| {
        let pool = Arc::new(Builder::new().num_threads(1).build());
        b.iter(|| {
            let pool = pool.clone();
            async_std::task::block_on(async {
                load_custom(
                    black_box(&[
                        "l01", "l02", "l03", "l04", "l05", "l06", "l07", "l08", "s01", "s02",
                        "s03", "s04", "s05", "s06", "s07", "s08", "s09", "s10", "s11", "s12",
                        "s13", "s14", "s15", "s16",
                    ]),
                    pool,
                )
                .await
            })
        })
    });
    c.bench_function("cpu 4", |b| {
        let pool = Arc::new(Builder::new().num_threads(4).build());
        b.iter(|| {
            let pool = pool.clone();
            async_std::task::block_on(async {
                load_custom(
                    black_box(&[
                        "l01", "l02", "l03", "l04", "l05", "l06", "l07", "l08", "s01", "s02",
                        "s03", "s04", "s05", "s06", "s07", "s08", "s09", "s10", "s11", "s12",
                        "s13", "s14", "s15", "s16",
                    ]),
                    pool,
                )
                .await
            })
        })
    });
    c.bench_function("cpu 8", |b| {
        let pool = Arc::new(Builder::new().num_threads(8).build());
        b.iter(|| {
            let pool = pool.clone();
            async_std::task::block_on(async {
                load_custom(
                    black_box(&[
                        "l01", "l02", "l03", "l04", "l05", "l06", "l07", "l08", "s01", "s02",
                        "s03", "s04", "s05", "s06", "s07", "s08", "s09", "s10", "s11", "s12",
                        "s13", "s14", "s15", "s16",
                    ]),
                    pool,
                )
                .await
            })
        })
    });
    c.bench_function("cpu 16", |b| {
        let pool = Arc::new(Builder::new().num_threads(16).build());
        b.iter(|| {
            let pool = pool.clone();
            async_std::task::block_on(async {
                load_custom(
                    black_box(&[
                        "l01", "l02", "l03", "l04", "l05", "l06", "l07", "l08", "s01", "s02",
                        "s03", "s04", "s05", "s06", "s07", "s08", "s09", "s10", "s11", "s12",
                        "s13", "s14", "s15", "s16",
                    ]),
                    pool,
                )
                .await
            })
        })
    });
    c.bench_function("async", |b| {
        b.iter(|| {
            async_std::task::block_on(async {
                load_async(black_box(&[
                    "l01", "l02", "l03", "l04", "l05", "l06", "l07", "l08", "s01", "s02", "s03",
                    "s04", "s05", "s06", "s07", "s08", "s09", "s10", "s11", "s12", "s13", "s14",
                    "s15", "s16",
                ]))
                .await
            })
        })
    });
    c.bench_function("sync", |b| {
        b.iter(|| {
            load_sync(black_box(&[
                "l01", "l02", "l03", "l04", "l05", "l06", "l07", "l08", "s01", "s02", "s03", "s04",
                "s05", "s06", "s07", "s08", "s09", "s10", "s11", "s12", "s13", "s14", "s15", "s16",
            ]))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
