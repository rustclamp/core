# Changelog: rustclamp-core

## Unreleased

### Changed

- Breaking: `Clock: Send + Sync`, so `&dyn Clock` can move into threads, tasks and
  scheduled jobs. `Cell`-based clocks switch to `ManualClock` or atomics.

### Added

- `ManualClock`: a settable, advanceable `Clock` for tests and simulations.
- `Reference`, a UUIDv7 naming one request, command or action (ADR 0021):
  time-ordered, `created_at()`, and `Reference::range(from, to)` for
  time-window filters. Random bytes are read from the OS 4 KiB at a time.
- `SystemClock`, the OS wall clock as a `Clock` implementation.
- The initial `Clock` capability contract for system and deterministic time sources.
- Additive module identity, capability requirement, and capability provision contracts.
- Stable contribution/target identities and a domain-owned target extension contract.
- Stable application, process, and execution-root identities for projection models.
- Added an identity-only lifecycle context and independent synchronous lifecycle
  participation traits for Initialize, Start, Ready, Drain, and Stop.
- A typed `Greeter` capability example using direct constructor injection.
- Phase 0 package scaffold and development checks.
- Expanded the package README with the Core dependency boundary, current scaffold
  status, and planned verification criteria.
