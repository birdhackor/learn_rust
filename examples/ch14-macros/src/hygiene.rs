use crate::Reading;

// --8<-- [start:ordinary-pair]
fn read_internal<T: Reading>(value: &T) -> i32 {
    value.reading()
}

pub fn read_pair_values<T: Reading, U: Reading>(left: &T, right: &U) -> (i32, i32) {
    (read_internal(left), read_internal(right))
}
// --8<-- [end:ordinary-pair]

// --8<-- [start:unqualified]
#[macro_export]
macro_rules! reading_type_unqualified {
    ($name:ident) => {
        pub struct $name(pub i32);

        impl Reading for $name {
            fn reading(&self) -> i32 {
                self.0
            }
        }
    };
}
// --8<-- [end:unqualified]

// --8<-- [start:exported]
#[macro_export]
macro_rules! reading_type {
    ($name:ident) => {
        #[derive(Debug, PartialEq)]
        pub struct $name(pub i32);

        impl $crate::Reading for $name {
            fn reading(&self) -> i32 {
                self.0
            }
        }
    };
}
// --8<-- [end:exported]

// --8<-- [start:pair-rule]
#[macro_export]
macro_rules! read_pair {
    ($left:expr, $right:expr $(,)?) => {{
        let value = $left;
        ($crate::read_value(value), $crate::read_value($right))
    }};
}
// --8<-- [end:pair-rule]

// --8<-- [start:private-rule]
#[macro_export]
macro_rules! private_reading {
    ($value:expr) => {
        $crate::hygiene::read_internal($value)
    };
}
// --8<-- [end:private-rule]

// --8<-- [start:checked-rule]
#[macro_export]
macro_rules! reading_type_checked {
    ($name:ident) => {
        $crate::reading_type!($name);
    };
    ($($other:tt)*) => {
        compile_error!("reading_type_checked! 需要一個型別名字");
    };
}
// --8<-- [end:checked-rule]
