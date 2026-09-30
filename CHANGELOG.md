# Changelog: rustclamp-core

## Unreleased

### Added

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
