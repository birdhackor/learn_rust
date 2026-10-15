use ch14_api::BatchLimit;

fn main() {
    let invalid = BatchLimit(0);
    println!("試圖繞過驗證={}", invalid.get());
}
