// --8<-- [start:send-check]
pub fn require_send<T: Send>(value: T) -> T {
    value
}
// --8<-- [end:send-check]
