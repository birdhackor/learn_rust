trait Extract {
    type Output;
    fn extract(self) -> Self::Output;
}

struct SwitchState {
    enabled: bool,
}

impl Extract for SwitchState {
    type Output = bool;

    fn extract(self) -> Self::Output {
        self.enabled
    }
}

fn double_number<S: Extract<Output = i32>>(source: S) -> i32 {
    source.extract() * 2
}

fn main() {
    println!("{}", double_number(SwitchState { enabled: true }));
}
