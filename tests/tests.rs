use std::{collections::HashSet, sync::Arc, time::Duration};

use timed_locks::{Mutex, RwLock};

#[tokio::test(start_paused = true)]
#[should_panic]
async fn rwlock_read_deadlock_panic() {
	let lock = Arc::new(RwLock::new(HashSet::<usize>::new()));
	let read_lock = lock.clone();
	let _write_lock = (*lock).write().await;

	read_lock.read().await;
}

#[tokio::test(start_paused = true)]
#[should_panic]
async fn rwlock_read_deadlock_panic_custom_timeout() {
	let lock = Arc::new(RwLock::new_with_timeout(HashSet::<usize>::new(), Duration::from_secs(60)));
	let read_lock = lock.clone();
	let _write_lock = (*lock).write().await;

	read_lock.read().await;
}

#[tokio::test(start_paused = true)]
#[should_panic]
async fn rwlock_write_deadlock_panic() {
	let lock = Arc::new(RwLock::new(HashSet::<usize>::new()));
	let write_lock = lock.clone();
	let _read_lock = (*lock).read().await;

	write_lock.write().await;
}

#[tokio::test(start_paused = true)]
async fn rwlock_read_deadlock_error() {
	let lock = Arc::new(RwLock::new(HashSet::<usize>::new()));
	let read_lock = lock.clone();
	let _write_lock = (*lock).write().await;

	assert!(read_lock.read_err().await.is_err());
}

#[tokio::test(start_paused = true)]
async fn rwlock_write_deadlock_error() {
	let lock = Arc::new(RwLock::new(HashSet::<usize>::new()));
	let write_lock = lock.clone();
	let _read_lock = (*lock).read().await;

	assert!(write_lock.write_err().await.is_err());
}

#[tokio::test(start_paused = true)]
#[should_panic]
async fn mutex_deadlock_panic() {
	let mutex = Arc::new(Mutex::new(HashSet::<usize>::new()));
	let mutex = mutex.clone();
	let _guard = (*mutex).lock().await;

	mutex.lock().await;
}

#[tokio::test(start_paused = true)]
#[should_panic]
async fn mutex_deadlock_panic_custom_timeout() {
	let mutex = Arc::new(Mutex::new_with_timeout(HashSet::<usize>::new(), Duration::from_secs(60)));
	let mutex = mutex.clone();
	let _guard = (*mutex).lock().await;

	mutex.lock().await;
}

#[tokio::test(start_paused = true)]
async fn mutex_deadlock_error() {
	let mutex = Arc::new(Mutex::new(HashSet::<usize>::new()));
	let mutex = mutex.clone();
	let _guard = (*mutex).lock().await;

	assert!(mutex.lock_err().await.is_err());
}
