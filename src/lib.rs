//! Minimal shared contracts for Clamp applications.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::time::SystemTime;

/// A stable identifier for an application architecture blueprint.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ApplicationId(&'static str);

impl ApplicationId {
    /// Creates an identifier from a stable, source-defined name.
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the stable application name.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// A stable identifier for one runnable application projection.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProcessId(&'static str);

impl ProcessId {
    /// Creates an identifier from a stable, source-defined name.
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the stable process name.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// A stable identifier for a process execution root.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExecutionId(&'static str);

impl ExecutionId {
    /// Creates an identifier from a stable, source-defined name.
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the stable execution name.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

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

/// A stable identifier for a qualifier that distinguishes capability instances.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QualifierId(&'static str);

impl QualifierId {
    /// Creates an identifier from a stable, source-defined name.
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the stable qualifier name.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// A stable identifier for a composition-time contribution kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContributionId(&'static str);

impl ContributionId {
    /// Creates an identifier from a stable, source-defined name.
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the stable contribution name.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// A stable identifier for a domain-specific contribution target.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContributionTargetId(&'static str);

impl ContributionTargetId {
    /// Creates an identifier from a stable, source-defined name.
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    /// Returns the stable target name.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// Gives a typed qualifier a stable semantic identity.
pub trait Qualifier {
    /// Stable semantic identifier; not a display label.
    const ID: QualifierId;
}

/// Associates a capability marker with its value type and stable identity.
pub trait Capability {
    /// The typed value that satisfies this capability.
    type Value: ?Sized;

    /// Stable semantic identifier; not a display label.
    const ID: CapabilityId;
}

/// Identifies a module that can participate in additive architecture contracts.
///
/// This contract describes module identity only. It does not impose a
/// constructor, lifecycle, health, or shutdown API.
pub trait Module {
    /// Stable semantic identifier; not a display label.
    const ID: ModuleId;
}

/// Identity available to a lifecycle participant without exposing a runtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LifecycleContext {
    application: ApplicationId,
    process: ProcessId,
}

impl LifecycleContext {
    /// Creates a context for one application's process lifecycle.
    pub const fn new(application: ApplicationId, process: ProcessId) -> Self {
        Self {
            application,
            process,
        }
    }

    /// Returns the application whose lifecycle is being coordinated.
    pub const fn application(self) -> ApplicationId {
        self.application
    }

    /// Returns the process whose lifecycle is being coordinated.
    pub const fn process(self) -> ProcessId {
        self.process
    }
}

/// Opts a module into resource preparation before its process starts.
pub trait Initialize: Module {
    /// An implementation-defined preparation error.
    type Error;

    /// Prepares resources; this does not mean the module is accepting work.
    fn initialize(&mut self, context: &LifecycleContext) -> Result<(), Self::Error>;
}

/// Opts a module into activation after initialization succeeds.
pub trait Start: Module {
    /// An implementation-defined activation error.
    type Error;

    /// Activates the module for its selected process.
    fn start(&mut self, context: &LifecycleContext) -> Result<(), Self::Error>;
}

/// Opts a module into the readiness check after activation.
pub trait Ready: Module {
    /// An implementation-defined readiness error.
    type Error;

    /// Confirms this participant is ready; process policy decides global readiness.
    fn ready(&mut self, context: &LifecycleContext) -> Result<(), Self::Error>;
}

/// Opts a module into graceful shutdown before it is stopped.
pub trait Drain: Module {
    /// An implementation-defined drain error.
    type Error;

    /// Finishes accepted work after process admission has closed.
    fn drain(&mut self, context: &LifecycleContext) -> Result<(), Self::Error>;
}

/// Opts a module into teardown after draining.
pub trait Stop: Module {
    /// An implementation-defined teardown error.
    type Error;

    /// Releases resources or execution mechanisms owned by this participant.
    fn stop(&mut self, context: &LifecycleContext) -> Result<(), Self::Error>;
}

/// Declares that a module requires one capability.
///
/// The default leaves provider selection to composition validation. A module
/// may name a preferred provider explicitly when its architecture requires it.
pub trait Requires<C: Capability>: Module {
    /// Explicit provider identity, or `None` when resolution should be unique.
    const SELECTED_PROVIDER: Option<ModuleId> = None;
}

/// Declares and exposes one capability provided by a module.
pub trait Provides<C: Capability>: Module {
    /// Returns the value satisfying this capability requirement.
    fn provided_value(&self) -> &C::Value;
}

/// Describes a composition-time declaration consumed by a domain target.
pub trait Contribution {
    /// Stable semantic identity; not a display label.
    const ID: ContributionId;

    /// Whether dropping this declaration without a consumer is an error.
    const REQUIRED: bool = true;
}

/// A domain-owned compiler from declarations to a runtime representation.
///
/// The target owns validation, conflict rules, ordering, and its empty-input
/// behavior. Core does not prescribe any of those semantics.
pub trait ContributionTarget {
    /// The declaration type accepted by this target.
    type Contribution: Contribution;

    /// The representation retained for runtime execution.
    type Runtime;

    /// A structured, target-specific assembly error.
    type Error;

    /// Stable semantic identity; not a display label.
    const ID: ContributionTargetId;

    /// Validates, orders, and compiles declarations into the runtime form.
    fn build(
        &self,
        contributions: &[(ModuleId, Self::Contribution)],
    ) -> Result<Self::Runtime, Self::Error>;
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
