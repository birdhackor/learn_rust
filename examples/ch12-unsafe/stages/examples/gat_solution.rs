trait LendMut {
    type View<'a>
    where
        Self: 'a;

    fn lend<'a>(&'a mut self) -> Self::View<'a>;
}

struct Numbers {
    values: Vec<i32>,
}

impl LendMut for Numbers {
    type View<'a>
        = &'a mut [i32]
    where
        Self: 'a;

    fn lend<'a>(&'a mut self) -> Self::View<'a> {
        self.values.as_mut_slice()
    }
}

fn main() {
    let mut source = Numbers {
        values: vec![80, 120],
    };
    {
        let view = source.lend();
        view[0] = 100;
        println!("透過借出視圖修改={}", view[0]);
    }
    source.values.push(40);
    let next = source.lend();
    assert_eq!(next.len(), 3);
    assert_eq!(next[0] + next[1] + next[2], 260);
    println!("下一次借出保留修改，總額={}", next[0] + next[1] + next[2]);
}
