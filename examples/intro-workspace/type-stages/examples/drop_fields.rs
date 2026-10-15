struct Tracked {
    id: i32,
}

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("清理欄位 id={}", self.id);
    }
}

struct Pair {
    first: Tracked,
    second: Tracked,
}

impl Drop for Pair {
    fn drop(&mut self) {
        println!("Pair先收尾，欄位仍是{}、{}", self.first.id, self.second.id);
    }
}

fn main() {
    let pair = Pair {
        first: Tracked { id: 10 },
        second: Tracked { id: 20 },
    };
    println!("建立Pair={}、{}", pair.first.id, pair.second.id);
}
