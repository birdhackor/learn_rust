use ch14_performance::count_borrowed;
use ch14_performance::count_cloned;
use ch14_performance::make_workload;

fn main() {
    for rows in [64usize, 256] {
        let lines = make_workload(rows);
        let borrowed = count_borrowed("Rust", lines.as_slice()).expect("搜尋文字非空");
        let cloned = count_cloned("Rust", lines.as_slice()).expect("搜尋文字非空");
        assert_eq!(borrowed, rows / 4);
        assert_eq!(cloned, borrowed);
        println!("{rows}行：借讀={borrowed}，owned複本={cloned}");
    }
}
