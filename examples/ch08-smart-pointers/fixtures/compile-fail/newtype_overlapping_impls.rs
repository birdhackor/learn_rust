trait Label {
    fn label(&self) -> &str;
}

struct Meters(i32);

impl<T> Label for T {
    fn label(&self) -> &str {
        "共用標籤"
    }
}

impl Label for Meters {
    fn label(&self) -> &str {
        "公尺"
    }
}

fn main() {}
