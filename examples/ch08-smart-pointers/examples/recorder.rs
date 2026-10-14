use ch08_smart_pointers::cells::Record;
use ch08_smart_pointers::cells::Recorder;
use std::mem::drop;

fn main() {
    let recorder = Recorder::empty();
    recorder.record(80);
    recorder.record(120);
    let view = recorder.view();
    println!(
        "shared介面記錄{}項，總額={}",
        view.len(),
        view.as_slice()[0] + view.as_slice()[1]
    );
    drop(view);
    recorder.record(40);
    let view = recorder.view();
    println!("釋放view後新增，第3項={}", view.as_slice()[2]);
}
