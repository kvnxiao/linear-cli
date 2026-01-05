//! UI utilities for progress indicators and spinners.

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Creates a spinner with the given message.
pub fn spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[".", "..", "...", "....", ".....", "......"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// A simple spinner that can be manually controlled.
pub struct Spinner {
    pb: ProgressBar,
}

impl Spinner {
    /// Creates a new spinner with the given message.
    pub fn new(message: &str) -> Self {
        Self {
            pb: spinner(message),
        }
    }

    /// Finishes the spinner, clearing it from the terminal.
    pub fn finish(&self) {
        self.pb.finish_and_clear();
    }
}
