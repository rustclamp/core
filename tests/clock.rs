//! Verifies deterministic Clock injection without a composition registry.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rustclamp_core::{Clock, ManualClock, SystemClock};

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

#[test]
fn system_clock_reads_the_os_wall_clock() {
    let before = SystemTime::now();
    let now = SystemClock.now();
    assert!(now >= before && now <= SystemTime::now());
}

#[test]
fn manual_clock_moves_only_when_told_and_is_shareable() {
    let clock = std::sync::Arc::new(ManualClock::new(UNIX_EPOCH));
    clock.advance(Duration::from_secs(90));
    let reader = std::sync::Arc::clone(&clock);
    let seen = std::thread::spawn(move || (reader.as_ref() as &dyn Clock).now())
        .join()
        .unwrap();
    assert_eq!(seen, UNIX_EPOCH + Duration::from_secs(90));
    clock.set(UNIX_EPOCH + Duration::from_secs(5));
    assert_eq!(clock.now(), UNIX_EPOCH + Duration::from_secs(5));
}

#[test]
fn a_resolved_clock_reference_crosses_threads() {
    fn assert_shareable<T: Send + Sync + ?Sized>() {}
    assert_shareable::<dyn Clock>();
}
