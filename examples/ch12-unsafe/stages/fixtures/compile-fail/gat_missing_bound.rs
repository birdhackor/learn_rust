trait BorrowView {
    type View<'a>;

    fn view<'a>(&'a self) -> Self::View<'a>;
}

fn main() {}
