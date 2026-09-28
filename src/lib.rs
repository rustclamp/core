//! Minimal shared contracts for Clamp applications.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::time::SystemTime;

/// A source of wall-clock time required by application behavior.
///
/// Implementations may be production clocks, deterministic test clocks, or
/// adapters around an application-owned time source. The contract does not
/// prescribe ownership, synchronization, or a runtime.
pub trait Clock {
    /// Returns the current wall-clock time.
    fn now(&self) -> SystemTime;
}
