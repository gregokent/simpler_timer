//!
//! # simple_timer
//! 
//! A simple timer mechanism to track arbitrary timeouts.
//! It doesn't do anything fancy, e.g. no callbacks upon expiry, just give it a Duration
//! and poll if the timer is expired. Timers can be reset and reused
//! for periodic contexts, such as a simple time based control loop.
//! 
//! # Example
//! 
//! ```
//! use std::time::Duration;
//! 
//! // 100ms timer
//! let tick = Timer::with_duration(Duration::from_millis(100));
//! // 1 sec timer
//! let end = Timer::with_duration(Duration::from_secs(1));
//! 
//! loop {
//!     if tick.expired() {
//!         // do something interesting
//!         println!("tick");
//!         tick.reset();
//!     }
//! 
//!     if end.expired() { 
//!         // don't reset, let's get out of here
//!         break; 
//!     }
//! }
//! 
//! println!("total time: {}ms", end.elapsed().as_millis());
//! 
//! ```
//!
//! The decision was made intentionally to only require a `&self` for 
//! resetting a timer so that another object can own a `Timer` and not require
//! `&mut self` of the object owning the timer.
//! 
use std::cell::Cell;
use std::time::{Duration, Instant};

/// Timer provides extremely basic timing abilities
#[derive(Debug, Clone)]
pub struct Timer {
    instant: Cell<Instant>,
    duration: Duration,
}


impl Timer {
    /// Creates a new timer of zero `Duration`.
    /// 
    /// Similar to `std::time::Instant` as this is really only useful 
    /// for getting `elapsed` time since `reset`
    pub fn new() -> Timer {
        Timer {
            instant: Cell::new(Instant::now()),
            duration: Duration::from_secs(0),
        }
    }

    /// Creates a new timer with `duration` length
    pub fn with_duration(duration: Duration) -> Timer {
        let mut timer = Timer::new();
        timer.duration = duration;
        timer
    }

    /// Resets the timer.
    /// 
    /// # Note
    /// Timers with `Duration` > 0 will no longer be expired after `reset()`
    /// `elapsed()` will start over at 0 after a `reset()`
    pub fn reset(&self) {
        self.instant.set(Instant::now());
    }

    /// Check if the timer is expired
    pub fn expired(&self) -> bool {
        self.instant.get().elapsed() > self.duration
    }

    /// Get `Duration` of time elapsed since `Timer` `reset`
    /// 
    /// # Note
    /// A newly constructed timer is considered to be `reset`
    pub fn elapsed(&self) -> Duration {
        self.instant.get().elapsed()
    }
}

#[cfg(test)]
mod tests {
}
