use std::sync::atomic::{AtomicUsize, Ordering};

static ID: AtomicUsize = AtomicUsize::new(0);

///  Returns the unique monotonically increasing id.
fn unique_string() -> String {
    format!("{}", ID.fetch_add(1, Ordering::Relaxed))
}