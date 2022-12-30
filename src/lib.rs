use pgx::prelude::*;
use pgx::{pg_shmem_init, PgLwLock};
use pgx::PgSharedMemoryInitialization;

pgx::pg_module_magic!();

static SHARED_LOCK: PgLwLock<i32> = PgLwLock::new();

#[pg_guard]
pub extern "C" fn _PG_init() {
    pg_shmem_init!(SHARED_LOCK);
}

#[pg_extern]
fn trigger_assertion_error() {
    // Hold lock
    let _lock = SHARED_LOCK.share();
    // Call into pg_guarded postgres function which internally reports an error
    unsafe { pg_sys::format_type_extended(pg_sys::InvalidOid, -1, 0) };
}

#[pg_extern]
fn works() {
    // No problem if not holding the lock
    unsafe { pg_sys::format_type_extended(pg_sys::InvalidOid, -1, 0) };
}

#[pg_extern]
fn also_works() {
    // No problem if the unwind is triggered by a rust panic
    unsafe { pg_sys::format_type_extended(pg_sys::InvalidOid, -1, 0) };
}