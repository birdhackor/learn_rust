use blake3::Hasher;
use lesson_content_streams::equal_readers;
use lesson_content_streams::hash_reader;
use lesson_content_streams::readers::short_reader;

fn main() {
    let data = "ABCDE".as_bytes();
    let mut by_three = short_reader(data, 3);
    let mut by_one = short_reader(data, 1);
    let equal = equal_readers(&mut by_three, &mut by_one).expect("固定的短讀取來源不會失敗");
    assert!(equal);
    println!("切分不同，內容相同={equal}");

    let mut reader = short_reader(data, 3);
    let digest = hash_reader(&mut reader).expect("固定來源只有成功的短讀取");
    let mut one_update = Hasher::new();
    one_update.update(data);
    assert_eq!(digest, one_update.finalize());
    println!("分批與一次 update 得到同一 digest={digest}");
}
