//! `timed-locks` is a set of smart pointers to `tokio::sync` locks that can be
//! used as drop-in replacement and will panic after a given timeout when the
//! lock cannot be acquired. Default timeout is 30 seconds. Failsafe mechanism
//! to give your service a chance to recover from a deadlock.
//!
//! # Example
//!
//! Deadlock that panics after 30 seconds:
//!
//! ```
//! # async {
//! let lock = timed_locks::RwLock::new(std::collections::HashSet::<usize>::new());
//! let _lock = lock.read().await;
//! lock.write();
//! # };
//! ```

#![deny(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_qualifications)]
#![warn(
	missing_debug_implementations,
	missing_docs,
	unused_import_braces,
	dead_code,
	clippy::unwrap_used,
	clippy::expect_used,
	clippy::missing_docs_in_private_items,
	clippy::missing_panics_doc
)]

mod mutex;
mod rwlock;

use std::time::Duration;

pub use mutex::Mutex;
pub use rwlock::RwLock;

/// Duration constant of 30 seconds for the default timeout.
pub const DEFAULT_TIMEOUT_DURATION: Duration = Duration::from_secs(30);

/// Custom result.
pub type Result<T> = std::result::Result<T, Error>;

/// Timed locks errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Mutex lock timeout error.
	#[error("Timed out while waiting for `lock` after {0} seconds.")]
	LockTimeout(u64),

	/// RwLock::read lock timeout error.
	#[error("Timed out while waiting for `read` lock after {0} seconds.")]
	ReadLockTimeout(u64),

	/// RwLock::write lock timeout error.
	#[error("Timed out while waiting for `write` lock after {0} seconds.")]
	WriteLockTimeout(u64),

	/// `tokio::sync::TryLockError` error.
	#[error(transparent)]
	TokioSyncTryLock(#[from] tokio::sync::TryLockError),
}
