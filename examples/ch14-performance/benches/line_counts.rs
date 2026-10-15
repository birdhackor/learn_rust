use ch14_performance::count_borrowed;
use ch14_performance::count_cloned;
use ch14_performance::make_workload;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use std::hint::black_box;
use std::time::Duration;

// --8<-- [start:bench]
fn compare_counts(criterion: &mut Criterion) {
    for rows in [64usize, 256] {
        let lines = make_workload(rows);
        assert_eq!(count_borrowed("Rust", lines.as_slice()), Ok(rows / 4));
        assert_eq!(count_cloned("Rust", lines.as_slice()), Ok(rows / 4));

        let borrowed_id = format!("borrowed/{rows}");
        criterion.bench_function(borrowed_id.as_str(), |bencher| {
            bencher.iter(|| {
                black_box(count_borrowed(
                    black_box("Rust"),
                    black_box(lines.as_slice()),
                ))
            });
        });

        let cloned_id = format!("cloned/{rows}");
        criterion.bench_function(cloned_id.as_str(), |bencher| {
            bencher
                .iter(|| black_box(count_cloned(black_box("Rust"), black_box(lines.as_slice()))));
        });
    }
}
// --8<-- [end:bench]

// --8<-- [start:runner]
criterion_group! {
    name = count_benches;
    config = Criterion::default()
        .sample_size(30)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2))
        .without_plots();
    targets = compare_counts
}
criterion_main!(count_benches);
// --8<-- [end:runner]
