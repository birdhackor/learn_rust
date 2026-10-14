use std::mem::drop;

struct Tracked {
    id: i32,
}

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("清理 id={}", self.id);
    }
}

fn main() {
    let first = Tracked { id: 1 };
    println!("建立資料={}", first.id);
    let second = first;
    println!("接手資料={}", second.id);
    drop(second);
    println!("清理完成後，繼續工作");
}
