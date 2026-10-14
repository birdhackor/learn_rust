use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

fn save_note(filename: &str, text: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    write!(file, "{text}")?;
    Ok(())
}

fn main() {
    match save_note("note-output.txt", "Rust\n中\n") {
        Ok(()) => match fs::read_to_string("note-output.txt") {
            Ok(saved) => {
                assert_eq!(saved.as_str(), "Rust\n中\n");
                println!("重新讀取確認={:?}", saved.as_str());
            }
            Err(error) => println!("寫入完成，重新讀取卻失敗：{error}"),
        },
        Err(error) => println!("寫入失敗：{error}"),
    }
}
