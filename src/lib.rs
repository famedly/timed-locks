//! `timed-locks` is a set of smart pointers to `tokio::sync` locks that can be
//! used as drop-in replacement and will either panic or return an error after a
//! given timeout when the lock cannot be acquired. Default timeout is 30
//! seconds.
//!
//! # Motivation
//!
//! In smaller codebases it's fairly trivial to prevent deadlocks by making sure
//! that you simply don't introduce any. As the codebase gets more complex
//! however, it gets more complex to cover all branches with automated or manual
//! testing and so the chances to unintentionally introduce deadlocks get
//! higher. In case a potential deadlock is introduced and makes it way into
//! production, it might not happen directly but only after a while when a
//! specific scenario in the codebase happens. In such cases it's often wanted
//! that a service can recover. While you can monitor services for
//! responsiveness from the outside and restart them if required, doing that
//! is offloading the responsbility to a thirdparty. So in order to have a
//! backup plan inside the service itself, it can be useful to have failsafe
//! mechanism in place that makes it possible to have the service itself recover
//! from a deadlock.
//!
//! The default behavior of panicking was chosen since just returning an error
//! might not help break the deadlock in a service in case the bug is
//! selfenforcing. So to have higher chances of recovering the process or task
//! should be shutdown completely by just panicking. However, if you are certain
//! that in your specific scenario returning errors can lead to gracefully
//! handling a deadlock there are the `read_err` / `write_err` methods that
//! return an error instead of panicking.
//!
//! # Examples
//!
//! Deadlock that panics after 30 seconds:
//!
//! ```
//! # async {
//! let lock =
//! 	timed_locks::RwLock::new(std::collections::HashSet::<usize>::new());
//! let _lock = lock.read().await;
//! lock.write().await;
//! # };
//! ```
//!
//! Deadlock that returns an error after 30 seconds:
//!
//! ```
//! # async {
//! let lock =
//! 	timed_locks::RwLock::new(std::collections::HashSet::<usize>::new());
//! let _lock = lock.read().await;
//! lock.write_err().await.unwrap();
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
