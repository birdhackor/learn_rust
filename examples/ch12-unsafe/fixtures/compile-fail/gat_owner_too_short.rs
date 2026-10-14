trait BorrowView {
    type View<'a>
    where
        Self: 'a;

    fn view<'a>(&'a self) -> Self::View<'a>;
}

struct Numbers {
    values: Vec<i32>,
}

impl BorrowView for Numbers {
    type View<'a>
        = &'a [i32]
    where
        Self: 'a;

    fn view<'a>(&'a self) -> Self::View<'a> {
        self.values.as_slice()
    }
}

fn main() {
    let view = {
        let source = Numbers { values: vec![80] };
        source.view()
    };
    println!("來源結束後={}", view[0]);
}
