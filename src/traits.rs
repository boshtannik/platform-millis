use super::ms;

/// Interface for platform specific time.
pub trait PlatformTime {
    /// Call of this mthod will return the current time in milliseconds.
    fn millis() -> ms;
}
