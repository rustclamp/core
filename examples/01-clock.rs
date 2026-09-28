//! Demonstrates a typed Clock requirement with direct Rust construction.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rustclamp_core::Clock;

struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

struct TestClock {
    now: SystemTime,
}

impl TestClock {
    fn at_unix_seconds(seconds: u64) -> Self {
        Self {
            now: UNIX_EPOCH + Duration::from_secs(seconds),
        }
    }
}

impl Clock for TestClock {
    fn now(&self) -> SystemTime {
        self.now
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
        let unix_seconds = self
            .clock
            .now()
            .duration_since(UNIX_EPOCH)
            .expect("clock must not precede the Unix epoch")
            .as_secs();
        format!("Hello, {name}! (unix second {unix_seconds})")
    }
}

fn main() {
    let production_greeter = Greeter::new(SystemClock);
    println!("{}", production_greeter.greet("world"));

    let deterministic_greeter = Greeter::new(TestClock::at_unix_seconds(42));
    println!("{}", deterministic_greeter.greet("Ada"));
}
