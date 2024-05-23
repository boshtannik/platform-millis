use super::ms;

/// Interface for platform specific millis timer implementation.
pub trait PlatformMillis {
    /// Call of this mthod will return the current time in milliseconds.
    fn millis() -> ms;
}
