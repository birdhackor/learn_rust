use ch14_performance::count_borrowed;
use ch14_performance::make_workload;
use ch14_profiling::repeat_counts;

#[global_allocator]
static ALLOCATOR: dhat::Alloc = dhat::Alloc;

fn main() {
    let lines = make_workload(64);
    assert_eq!(count_borrowed("Rust", lines.as_slice()), Ok(16));

    let profiler = dhat::Profiler::builder()
        .file_name("target/dhat-solution.json")
        .build();
    let total_hits = repeat_counts(lines.as_slice(), true, 100);
    let stats = dhat::HeapStats::get();
    drop(profiler);

    assert_eq!(total_hits, 1600);
    println!("累計符合行數={total_hits}");
    println!(
        "累計配置={} blocks，{} bytes",
        stats.total_blocks, stats.total_bytes
    );
    println!("受追蹤bytes峰值={}", stats.max_bytes);
    println!("snapshot時仍活著的受追蹤bytes={}", stats.curr_bytes);
}
