//! Verifies deterministic Clock injection without a composition registry.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rustclamp_core::Clock;

struct FixedClock(SystemTime);

impl Clock for FixedClock {
    fn now(&self) -> SystemTime {
        self.0
    }
}

struct Greeter<C> {
    clock: C,
}

impl<C: Clock> Greeter<C> {
    fn new(clock: C) -> Self {
        Self { clock }
    }

    fn greet(&self, name: &str) -> String {
        let seconds = self
            .clock
            .now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("Hello, {name}! (unix second {seconds})")
    }
}

#[test]
fn greeter_accepts_a_deterministic_clock_through_an_ordinary_constructor() {
    let clock = FixedClock(UNIX_EPOCH + Duration::from_secs(42));
    let greeter = Greeter::new(clock);

    assert_eq!(greeter.greet("Ada"), "Hello, Ada! (unix second 42)");
}
