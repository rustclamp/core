//! Minimal shared contracts for Clamp applications.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::time::SystemTime;

/// A stable identifier for a declared application module.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModuleId(&'static str);

impl ModuleId {
    /// Creates an identifier from a stable, source-defined name.
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the stable module name.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// A stable identifier for a capability contract.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CapabilityId(&'static str);

impl CapabilityId {
    /// Creates an identifier from a stable, source-defined name.
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the stable capability name.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// Associates a capability marker with its value type and stable identity.
pub trait Capability {
    /// The typed value that satisfies this capability.
    type Value: ?Sized;

    /// Stable semantic identifier; not a display label.
    const ID: CapabilityId;
}

/// A source of wall-clock time required by application behavior.
///
/// Implementations may be production clocks, deterministic test clocks, or
/// adapters around an application-owned time source. The contract does not
/// prescribe ownership, synchronization, or a runtime.
pub trait Clock {
    /// Returns the current wall-clock time.
    fn now(&self) -> SystemTime;
}

/// Marker type that identifies the [`Clock`] capability to the Kernel.
pub struct ClockCapability;

impl Capability for ClockCapability {
    type Value = dyn Clock;

    const ID: CapabilityId = CapabilityId::new("rustclamp.core.clock");
}
