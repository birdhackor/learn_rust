struct ReadOnlyI32<'a> {
    source: &'a i32,
    pointer: *const i32,
}

impl<'a> ReadOnlyI32<'a> {
    fn new(source: &'a i32) -> Self {
        Self {
            source,
            pointer: &raw const *source,
        }
    }

    fn original(&self) -> &'a i32 {
        self.source
    }

    fn read(&self) -> i32 {
        // SAFETY: private constructor讓pointer與source指向同一有效i32；
        // source的shared borrow維持有效性，沒有其他寫入介面。
        unsafe { *self.pointer }
    }
}

fn main() {
    let saved = {
        let amount = 80;
        ReadOnlyI32::new(&amount)
    };
    println!("來源已結束，仍想讀={}、{}", saved.read(), saved.original());
}
