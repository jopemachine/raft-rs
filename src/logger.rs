#![allow(missing_docs)]

use slog::{debug, error, info, trace, warn};
use std::fmt::Debug;

pub trait Logger: Send + Sync + Debug {
    fn trace(&self, msg: &str);
    fn debug(&self, msg: &str);
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
    fn fatal(&self, msg: &str);
}

#[derive(Clone, Debug)]
pub struct Slogger {
    pub slog: slog::Logger,
}

impl Logger for Slogger {
    fn trace(&self, msg: &str) {
        trace!(self.slog, "{}", msg);
    }

    fn debug(&self, msg: &str) {
        debug!(self.slog, "{}", msg);
    }

    fn info(&self, msg: &str) {
        info!(self.slog, "{}", msg);
    }

    fn warn(&self, msg: &str) {
        warn!(self.slog, "{}", msg);
    }

    fn error(&self, msg: &str) {
        error!(self.slog, "{}", msg);
    }

    fn fatal(&self, msg: &str) {
        fatal!(self.slog, "{}", msg);
    }
}
