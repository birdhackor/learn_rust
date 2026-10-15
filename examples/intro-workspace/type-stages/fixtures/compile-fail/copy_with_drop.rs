#[derive(Clone, Copy)]
struct Tracked {
    id: i32,
}

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("清理 id={}", self.id);
    }
}

fn main() {}
