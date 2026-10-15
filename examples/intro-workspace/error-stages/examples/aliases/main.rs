pub mod aliases;
pub mod policy;

use crate::aliases::CountResult;
use crate::aliases::count_with_alias;
use crate::policy::CountError;

fn main() {
    let short_name: CountResult = count_with_alias(3);
    let original_name: Result<i32, CountError> = short_name;
    assert_eq!(original_name, Ok(3));
    println!("兩種型別名稱都表示={original_name:?}");
}
