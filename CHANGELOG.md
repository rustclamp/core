# Changelog: rustclamp-core

## Unreleased

## 1.0.0 - 2026-10-19

### Fixed

- `Reference::new` works on Windows: it reads the OS random source through
  `getrandom` there, a Windows-only dependency. Unix still reads
  `/dev/urandom` and has no dependencies.

### Changed

- `Reference::new` uses `AtomicU64::try_update`: `fetch_update` is deprecated
  on current stable, so every dependent crate warned (and `-D warnings` failed).
- Ahead of 1.0: `ParseReferenceError` is `#[non_exhaustive]`, so a reason can be
  added later without a breaking change (it can no longer be built outside core);
  `ClockCapability` derives `Clone`, `Copy`, `Debug` and `Default`.
- Ready for crates.io: `publish = false` is gone and the manifest carries
  homepage, documentation, keywords and categories. Nothing is uploaded until
  rustclamp's `tools/publish.py --execute` runs.
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
