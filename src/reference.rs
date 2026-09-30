//! [`Reference`]: a UUIDv7 naming one request, command or action (ADR 0021).

use std::cell::RefCell;
use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// A UUIDv7 that names one request, command or action, so its log lines can
/// be found again.
///
/// It starts with its creation time in Unix milliseconds, so references sort
/// by time, and in text as well, which makes a time window a range filter
/// over references ([`Reference::range`]). The time can be read back from a
/// reference; use a random (v4) ID where it must stay hidden, such as tokens.
///
/// ```
/// use rustclamp_core::Reference;
///
/// let first = Reference::new();
/// let second = Reference::new();
/// assert!(first < second);
/// assert!(first.to_string() < second.to_string(), "text sorts the same way");
/// assert_eq!(first.to_string().parse::<Reference>(), Ok(first));
/// ```
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Reference([u8; 16]);

// A random reference is no sensible default value.
#[allow(clippy::new_without_default)]
impl Reference {
    /// A new reference. References made by this process are strictly
    /// increasing, even within one millisecond or across a clock step back:
    /// the 12 bits after the time count up within a millisecond (RFC 9562,
    /// section 6.2, method 3), and the last 62 bits are random. Past 4096
    /// references in one millisecond the time runs ahead of the clock until
    /// the clock catches up, so ordering holds at any rate.
    ///
    /// # Panics
    ///
    /// Where `/dev/urandom` is missing (Windows): a guessable ID is worse than none.
    pub fn new() -> Self {
        // Time, version and counter: one atomic, so threads never wait on a lock.
        let fresh = (millis(SystemTime::now()) << 16) | 0x7000;
        let next = |last: u64| if fresh > last { fresh } else { increment(last) };
        let high = match LAST.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |last| {
            Some(next(last))
        }) {
            Ok(last) | Err(last) => next(last),
        };
        let mut low = RANDOM.with_borrow_mut(Random::take).to_be_bytes();
        low[0] = (low[0] & 0x3f) | 0x80;
        let mut bytes = [0; 16];
        bytes[..8].copy_from_slice(&high.to_be_bytes());
        bytes[8..].copy_from_slice(&low);
        Self(bytes)
    }

    /// When the reference was made, to the millisecond.
    pub fn created_at(&self) -> SystemTime {
        let mut millis = [0; 8];
        millis[2..].copy_from_slice(&self.0[..6]);
        UNIX_EPOCH + Duration::from_millis(u64::from_be_bytes(millis))
    }

    /// The lowest and highest reference that can be made between `from` and
    /// `to` (both included, to the millisecond): every reference made in that
    /// window lies between the two, in bytes and in text.
    ///
    /// ```
    /// use std::time::{Duration, SystemTime};
    /// use rustclamp_core::Reference;
    ///
    /// let now = SystemTime::now();
    /// let reference = Reference::new();
    /// let (low, high) = Reference::range(now, now + Duration::from_secs(1));
    /// assert!(low <= reference && reference <= high);
    /// ```
    pub fn range(from: SystemTime, to: SystemTime) -> (Self, Self) {
        let mut low = [0; 16];
        low[..6].copy_from_slice(&millis(from).to_be_bytes()[2..]);
        low[6] = 0x70;
        low[8] = 0x80;
        let mut high = [0xff; 16];
        high[..6].copy_from_slice(&millis(to).to_be_bytes()[2..]);
        high[6] = 0x7f;
        high[8] = 0xbf;
        (Self(low), Self(high))
    }

    /// The 16 bytes.
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl fmt::Display for Reference {
    /// The hyphenated lowercase form, `0192f1c2-3b4a-7c5d-8e6f-a1b2c3d4e5f6`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // One write instead of sixteen: it runs on every request.
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut text = [b'-'; 36];
        let mut at = 0;
        for (index, byte) in self.0.iter().enumerate() {
            if matches!(index, 4 | 6 | 8 | 10) {
                at += 1;
            }
            text[at] = HEX[usize::from(byte >> 4)];
            text[at + 1] = HEX[usize::from(byte & 0x0f)];
            at += 2;
        }
        formatter.write_str(std::str::from_utf8(&text).map_err(|_| fmt::Error)?)
    }
}

impl fmt::Debug for Reference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Reference({self})")
    }
}

impl FromStr for Reference {
    type Err = ParseReferenceError;

    /// Parses the hyphenated form, in either case. Anything other than a
    /// UUID of version 7 is refused.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let text = text.as_bytes();
        if text.len() != 36 || [8, 13, 18, 23].iter().any(|&dash| text[dash] != b'-') {
            return Err(ParseReferenceError);
        }
        let mut digits = text.iter().filter(|&&byte| byte != b'-');
        let mut bytes = [0; 16];
        for byte in &mut bytes {
            let (Some(high), Some(low)) = (digits.next(), digits.next()) else {
                return Err(ParseReferenceError);
            };
            *byte = (hex(*high)? << 4) | hex(*low)?;
        }
        if bytes[6] >> 4 != 7 || bytes[8] >> 6 != 0b10 {
            return Err(ParseReferenceError);
        }
        Ok(Self(bytes))
    }
}

/// The text is not a hyphenated UUID of version 7.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseReferenceError;

impl fmt::Display for ParseReferenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("not a UUIDv7 reference")
    }
}

impl std::error::Error for ParseReferenceError {}

fn hex(digit: u8) -> Result<u8, ParseReferenceError> {
    match digit {
        b'0'..=b'9' => Ok(digit - b'0'),
        b'a'..=b'f' => Ok(digit - b'a' + 10),
        b'A'..=b'F' => Ok(digit - b'A' + 10),
        _ => Err(ParseReferenceError),
    }
}

/// Unix milliseconds; times before 1970 count as 0.
fn millis(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_millis() as u64)
}

/// The first 64 bits of the last reference made: 48 bits of milliseconds,
/// the version nibble and the 12-bit counter.
static LAST: AtomicU64 = AtomicU64::new(0);

/// `high` with its counter one up; a full counter moves into the next
/// millisecond with the counter back at zero.
fn increment(high: u64) -> u64 {
    if high & 0x0fff < 0x0fff {
        high + 1
    } else {
        (((high >> 16) + 1) << 16) | 0x7000
    }
}

/// Random numbers read from the OS 4 KiB at a time, per thread.
struct Random {
    pool: [u8; 4096],
    used: usize,
}

thread_local! {
    static RANDOM: RefCell<Random> = const { RefCell::new(Random { pool: [0; 4096], used: 4096 }) };
}

impl Random {
    // ponytail: std-only, Unix only; `getrandom` for Windows
    fn take(&mut self) -> u64 {
        if self.used == self.pool.len() {
            std::fs::File::open("/dev/urandom")
                .and_then(|mut source| std::io::Read::read_exact(&mut source, &mut self.pool))
                .expect("references need the operating system random source /dev/urandom");
            self.used = 0;
        }
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&self.pool[self.used..self.used + 8]);
        self.used += 8;
        u64::from_be_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn references_carry_the_time_and_stay_ordered() {
        let before = SystemTime::now();
        let references: Vec<Reference> = (0..1000).map(|_| Reference::new()).collect();
        let after = SystemTime::now();
        assert!(references.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(
            references
                .windows(2)
                .all(|pair| pair[0].to_string() < pair[1].to_string())
        );
        for reference in &references {
            assert_eq!((reference.0[6] >> 4, reference.0[8] >> 6), (7, 0b10));
            let made = reference.created_at();
            // created_at is truncated to the millisecond, and other tests
            // minting in parallel can push the counter a few milliseconds ahead.
            assert!(made + Duration::from_millis(1) > before);
            assert!(made <= after + Duration::from_millis(100));
        }
    }

    #[test]
    fn a_full_counter_carries_into_the_next_millisecond() {
        let high = (5 << 16) | 0x7ffe;
        assert_eq!(increment(high), (5 << 16) | 0x7fff);
        assert_eq!(increment(increment(high)), (6 << 16) | 0x7000);
    }

    #[test]
    fn threads_share_one_increasing_sequence() {
        let threads: Vec<_> = (0..8)
            .map(|_| std::thread::spawn(|| (0..2000).map(|_| Reference::new()).collect::<Vec<_>>()))
            .collect();
        let mut all: Vec<Reference> = threads
            .into_iter()
            .flat_map(|t| t.join().unwrap())
            .collect();
        let count = all.len();
        all.sort();
        all.dedup_by_key(|reference| reference.0[..8].to_vec());
        assert_eq!(all.len(), count, "no two references share time and counter");
    }

    #[test]
    fn range_brackets_its_window_only() {
        let at = |millis| UNIX_EPOCH + Duration::from_millis(millis);
        let (low, high) = Reference::range(at(1_000), at(2_000));
        let inside = [at(1_000), at(1_500), at(2_000)];
        for time in inside {
            let (first, last) = Reference::range(time, time);
            assert!(low <= first && last <= high, "{time:?}");
        }
        assert!(Reference::range(at(999), at(999)).1 < low);
        assert!(Reference::range(at(2_001), at(2_001)).0 > high);
        assert_eq!(low.created_at(), at(1_000));
        assert_eq!(high.created_at(), at(2_000));
        assert!(low.to_string().parse::<Reference>().is_ok());
        assert!(high.to_string().parse::<Reference>().is_ok());
    }

    #[test]
    fn parse_round_trips_and_refuses_everything_else() {
        let text = "0192f1c2-3b4a-7c5d-8e6f-a1b2c3d4e5f6";
        let reference: Reference = text.parse().unwrap();
        assert_eq!(reference.to_string(), text);
        assert_eq!(text.to_uppercase().parse(), Ok(reference));
        assert_eq!(format!("{reference:?}"), format!("Reference({text})"));
        for junk in [
            "",
            "0192f1c2-3b4a-7c5d-8e6f-a1b2c3d4e5f",
            "0192f1c2x3b4a-7c5d-8e6f-a1b2c3d4e5f6",
            "zz92f1c2-3b4a-7c5d-8e6f-a1b2c3d4e5f6",
            "0192f1c2-3b4a-4c5d-8e6f-a1b2c3d4e5f6", // version 4
            "0192f1c2-3b4a-7c5d-ce6f-a1b2c3d4e5f6", // wrong variant
            "0192f1c2-3b4a-7c5d-8e6f-a1b2c3d4e5fé",
        ] {
            assert_eq!(
                junk.parse::<Reference>(),
                Err(ParseReferenceError),
                "{junk}"
            );
        }
    }
}
