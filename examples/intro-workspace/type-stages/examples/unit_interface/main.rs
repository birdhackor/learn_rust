pub mod newtypes;

use newtypes::BatchSize;
use newtypes::MeterBatch;
use newtypes::Meters;

fn main() {
    let route = MeterBatch::from_values(vec![Meters(80), Meters(120)]);
    println!("路程報告：{route}");
    println!("再次借讀總路程={}公尺", route.total().0);

    let values: Vec<i32> = vec![80, 120];
    println!(
        "本例定義的trait能實作在Vec上，項目數={}",
        values.batch_size()
    );
}
