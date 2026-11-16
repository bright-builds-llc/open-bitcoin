// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Short grace for advisory locks still held by a child between fork and exec.
//!
//! A lock belongs to the open file description, and a forked child shares every
//! description its parent had open until exec closes them. Closing a lock file
//! while another thread spawns a process therefore leaves the lock held for a
//! few milliseconds after its owner dropped it, and an immediate reopen in the
//! same process would be refused as if a second process owned the store.

use std::{
    thread,
    time::{Duration, Instant},
};

/// Long enough to outlast a descheduled child on a loaded host, short enough
/// that a store really owned by another process is still refused promptly.
const STALE_HOLDER_GRACE: Duration = Duration::from_millis(250);
const POLL_INTERVAL: Duration = Duration::from_millis(2);

/// Repeats `attempt` while it reports a held lock, for at most the grace period.
/// Every other outcome, and the last held-lock refusal, is returned unchanged.
pub(super) fn retry_while_locked<T, E>(
    attempt: impl FnMut() -> Result<T, E>,
    is_locked: impl Fn(&E) -> bool,
) -> Result<T, E> {
    retry_while_locked_for(STALE_HOLDER_GRACE, attempt, is_locked)
}

fn retry_while_locked_for<T, E>(
    grace: Duration,
    mut attempt: impl FnMut() -> Result<T, E>,
    is_locked: impl Fn(&E) -> bool,
) -> Result<T, E> {
    let deadline = Instant::now() + grace;
    loop {
        let result = attempt();
        let still_locked = matches!(&result, Err(error) if is_locked(error));
        if !still_locked || Instant::now() >= deadline {
            return result;
        }
        thread::sleep(POLL_INTERVAL);
    }
}

#[cfg(test)]
mod tests {
    use super::retry_while_locked_for;
    use std::time::{Duration, Instant};

    #[derive(Debug, PartialEq)]
    enum Refusal {
        Locked,
        Other,
    }

    fn is_locked(refusal: &Refusal) -> bool {
        *refusal == Refusal::Locked
    }

    #[test]
    fn lock_released_within_grace_is_acquired() {
        // Arrange
        let mut attempts = 0;

        // Act
        let result = retry_while_locked_for(
            Duration::from_secs(5),
            || {
                attempts += 1;
                if attempts < 3 {
                    Err(Refusal::Locked)
                } else {
                    Ok(attempts)
                }
            },
            is_locked,
        );

        // Assert
        assert_eq!(result, Ok(3));
    }

    #[test]
    fn lock_held_past_grace_is_refused() {
        // Arrange
        let grace = Duration::from_millis(20);
        let started = Instant::now();

        // Act
        let result: Result<(), Refusal> =
            retry_while_locked_for(grace, || Err(Refusal::Locked), is_locked);

        // Assert
        assert_eq!(result, Err(Refusal::Locked));
        assert!(started.elapsed() >= grace);
    }

    #[test]
    fn other_refusal_is_returned_without_retry() {
        // Arrange
        let mut attempts = 0;

        // Act
        let result: Result<(), Refusal> = retry_while_locked_for(
            Duration::from_secs(5),
            || {
                attempts += 1;
                Err(Refusal::Other)
            },
            is_locked,
        );

        // Assert
        assert_eq!(result, Err(Refusal::Other));
        assert_eq!(attempts, 1);
    }
}
