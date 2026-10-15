use ch15_ffi_buffer::count_bytes;

fn main() {
    let bytes = [10u8, 0, 20];
    let counts = count_bytes(&bytes).expect("三項長度可由u32表示");
    println!("buffer：零={}，非零={}", counts.zeros, counts.nonzero);
    let empty = count_bytes(&[]).expect("零長度可表示");
    println!("空buffer：零={}，非零={}", empty.zeros, empty.nonzero);
    println!("呼叫後原buffer仍有三項={}", bytes.len());
}
