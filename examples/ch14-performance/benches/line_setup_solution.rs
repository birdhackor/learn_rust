use ch14_performance::count_borrowed;
use ch14_performance::make_workload;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use std::hint::black_box;
use std::time::Duration;

fn count_with_setup(criterion: &mut Criterion) {
    for rows in [64usize, 256] {
        let known = make_workload(rows);
        assert_eq!(count_borrowed("Rust", known.as_slice()), Ok(rows / 4));
        let id = format!("setup_and_borrowed/{rows}");
        criterion.bench_function(id.as_str(), |bencher| {
            bencher.iter(|| {
                let lines = make_workload(black_box(rows));
                black_box(count_borrowed(
                    black_box("Rust"),
                    black_box(lines.as_slice()),
                ))
            });
        });
    }
}

criterion_group! {
    name = setup_benches;
    config = Criterion::default()
        .sample_size(30)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2))
        .without_plots();
    targets = count_with_setup
}
criterion_main!(setup_benches);
