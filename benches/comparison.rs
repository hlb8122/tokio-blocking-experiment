use criterion::*;
use tokio::runtime::{self, Runtime};
use tokio_blocking::*;

const SAMPLE_SIZE: usize = 10;

const REQUESTS: [usize; 3] = [1, 4, 8];

const MAX_LATENCY: u64 = 1_000;
const LATENCY_STEP: usize = 250;

const MAX_CORES: usize = 4;

fn new_runtime(n_cores: usize) -> Runtime {
    runtime::Builder::new()
        .threaded_scheduler()
        .core_threads(n_cores)
        .build()
        .unwrap()
}

fn bench_blocking(c: &mut Criterion) {
    let mut group = c.benchmark_group("Comparison");
    group.sample_size(SAMPLE_SIZE);

    for n_cores in 1..MAX_CORES + 1 {
        for n_requests in REQUESTS.iter() {
            for latency in (0..MAX_LATENCY + 1).step_by(LATENCY_STEP) {
                let parameters = format!(
                    "{} core, {} par, {} micros",
                    n_cores, n_requests, latency
                );
                // Bench blocking
                group.bench_with_input(
                    BenchmarkId::new("blocking", &parameters),
                    &(n_cores, n_requests, latency),
                    |b, (n_cores, n_requests, latency)| {
                        let mut rt = new_runtime(*n_cores);
                        b.iter(|| {
                            let tasks = (0..**n_requests).map(|_| rt.spawn(blocking(*latency)));
                            let joined_task = futures::future::join_all(tasks);
                            rt.block_on(joined_task);
                        });
                    },
                );

                // Bench non-blocking
                group.bench_with_input(
                    BenchmarkId::new("Non-blocking", &parameters),
                    &(n_cores, n_requests, latency),
                    |b, (n_cores, n_requests, latency)| {
                        let mut rt = new_runtime(*n_cores);
                        b.iter(|| {
                            let tasks = (0..**n_requests).map(|_| rt.spawn(non_blocking(*latency)));
                            let joined_task = futures::future::join_all(tasks);
                            rt.block_on(joined_task);
                        });
                    },
                );
            }
        }
    }

    group.finish();
}

criterion_group!(benches, bench_blocking);
criterion_main!(benches);
