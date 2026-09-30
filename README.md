<img src="https://docs.rustclamp.com/assets/rustclamp-logo.png" alt="RustClamp logo" width="160">

# rustclamp-core

Core component of [RustClamp](https://github.com/rustclamp/rustclamp): minimal,
domain-neutral contracts shared by the other components. It defines contracts and
identities; resolution and runtime behavior live in
[kernel](https://github.com/rustclamp/kernel) and
[runtime](https://github.com/rustclamp/runtime). Zero external dependencies, no
Tokio or `async` required. Companion crate, not a standalone framework.

## Install

Not published to crates.io yet (`publish = false`). Depend on it from git, Rust 1.96.1+:

```toml
[dependencies]
rustclamp-core = { git = "https://github.com/rustclamp/core" }
```

## Example

```rust
use std::time::{Duration, UNIX_EPOCH};
use rustclamp_core::{Clock, ManualClock};

let clock = ManualClock::new(UNIX_EPOCH);
clock.advance(Duration::from_secs(60));
let now = (&clock as &dyn Clock).now(); // deterministic time for tests
```

## Main API

- **Time:** `Clock` (`Send + Sync`, so `&dyn Clock` moves into threads and tasks),
  `SystemClock`, `ManualClock` (`set`, `advance`), `ClockCapability`.
- **Identity:** `Reference` (UUIDv7 naming a request, command or action;
  `created_at`, `Reference::range`), `ModuleId`, `CapabilityId`, `QualifierId`,
  `ApplicationId`, `ProcessId`, `ExecutionId`, `ContributionId`, `ContributionTargetId`.
- **Modules:** `Module`, `Requires<C>`, `Provides<C>`, `Capability`, `Qualifier`.
- **Contributions:** `Contribution`, `ContributionTarget`; each target owns
  validation and the runtime representation it builds.
- **Lifecycle:** `LifecycleContext` plus opt-in `Initialize`, `Start`, `Ready`,
  `Drain`, `Stop`; a module implements any subset.

Feature flags: none. See [CHANGELOG.md](CHANGELOG.md).

## Documentation

<https://docs.rustclamp.com>

## Development

```sh
cargo fmt --all -- --check
cargo clippy --offline --locked --all-targets --all-features -- -D warnings
cargo test --offline --locked --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --offline --locked --no-deps --all-features
```

Coordinated checkout, architecture checks and release policy: see the
[facade contributor guide](https://github.com/rustclamp/rustclamp/blob/main/CONTRIBUTING.md).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option. Unless you state otherwise, any
contribution you submit for inclusion is dual licensed as above, without
additional terms or conditions.
