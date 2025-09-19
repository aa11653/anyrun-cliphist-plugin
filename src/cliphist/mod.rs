//! This module provides functionality for interacting with `cliphist` via command line.
//! Please refer https://github.com/sentriz/cliphist/ for more details about `cliphist`.
mod actions;
mod types;

pub use actions::{copy_history, get_history, wipe_history};
#[allow(unused)]
pub use types::{ClipboardEntry, CliphistError};
