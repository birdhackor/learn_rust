trait Record {
    fn record(&self, amount: i32);
}

struct Recorder {
    values: Vec<i32>,
}

impl Record for Recorder {
    fn record(&self, amount: i32) {
        self.values.push(amount);
    }
}

fn main() {
    let recorder = Recorder { values: Vec::new() };
    recorder.record(80);
}
