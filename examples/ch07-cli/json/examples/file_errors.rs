use ch07_json::files::NotebookError;
use ch07_json::files::read_notebook;

fn main() {
    for filename in [
        "fixtures/notebook.json",
        "fixtures/missing.json",
        "fixtures/broken-syntax.json",
        "fixtures/not-utf8.bin",
    ] {
        match read_notebook(filename) {
            Ok(notebook) => println!("{filename}：標題={}", notebook.title.as_str()),
            Err(NotebookError::Io(error)) => {
                println!("{filename}：I/O {:?}，{error}", error.kind());
            }
            Err(NotebookError::Json(error)) => println!("{filename}：JSON 格式，{error}"),
        }
    }
}
