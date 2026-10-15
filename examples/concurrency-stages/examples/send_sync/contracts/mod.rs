// --8<-- [start:send-check]
pub fn require_send<T: Send>(value: T) -> T {
    value
}
// --8<-- [end:send-check]

// --8<-- [start:sync-check]
pub fn require_sync<T: Sync + ?Sized>(value: &T) -> &T {
    value
}
// --8<-- [end:sync-check]
