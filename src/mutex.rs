//! Smart pointer to [`tokio::sync::Mutex`].

use std::time::Duration;

use tokio::time::timeout;

use crate::{Result, DEFAULT_TIMEOUT_DURATION};

/// Smart pointer to [`tokio::sync::Mutex`].
///
/// Wraps acquiring the lock into [`timeout`] with a [`Duration`] of 30 seconds
/// by default.
#[derive(Debug)]
pub struct Mutex<T> {
	/// The actual [`tokio::sync::Mutex`]
	inner: tokio::sync::Mutex<T>,
	/// The timeout duration
	timeout: Duration,
}

impl<T> Mutex<T> {
	/// Create new `Mutex` with default timeout of 30 seconds.
	pub fn new(value: T) -> Self {
		Self { inner: tokio::sync::Mutex::new(value), timeout: DEFAULT_TIMEOUT_DURATION }
	}

	/// Create new `Mutex` with given timeout.
	pub fn new_with_timeout(value: T, timeout: Duration) -> Self {
		Self { inner: tokio::sync::Mutex::new(value), timeout }
	}

	/// Wrapper around [`tokio::sync::Mutex::lock()`]. Will time out if the
	/// lock can’t get acquired until the timeout is reached.
	///
	/// # Panics
	///
	/// Panics when timeout is reached.
	pub async fn lock(&self) -> tokio::sync::MutexGuard<'_, T> {
		match timeout(self.timeout, self.inner.lock()).await {
			Ok(guard) => guard,
			Err(_) => panic!(
				"Timed out while waiting for `read` lock after {} seconds.",
				self.timeout.as_secs()
			),
		}
	}

	/// Wrapper around [`tokio::sync::Mutex::lock()`]. Will time out if the
	/// lock can't get acquired until the timeout is reached.
	///
	/// Returns an error if timeout is reached.
	pub async fn lock_err(&self) -> Result<tokio::sync::MutexGuard<'_, T>> {
		let guard = timeout(self.timeout, self.inner.lock())
			.await
			.map_err(|_| crate::Error::LockTimeout(self.timeout.as_secs()))?;

		Ok(guard)
	}
}

impl<T> std::ops::Deref for Mutex<T> {
	type Target = tokio::sync::Mutex<T>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T: Default> Default for Mutex<T> {
	fn default() -> Self {
		Self::new(T::default())
	}
}

impl<T> From<T> for Mutex<T> {
	fn from(value: T) -> Self {
		Self::new(value)
	}
}
