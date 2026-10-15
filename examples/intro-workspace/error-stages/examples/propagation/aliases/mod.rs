use crate::policy::CountError;
use crate::policy::checked_count;

pub type CountResult = Result<i32, CountError>;

pub fn count_with_alias(value: i32) -> CountResult {
    checked_count(value)
}
