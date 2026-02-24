use anyhow::{Result, anyhow};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    pub fn check_cancelled(&self, stage: &'static str) -> Result<()> {
        if self.is_cancelled() {
            return Err(anyhow!("operation cancelled at stage={stage}"));
        }
        Ok(())
    }
}
