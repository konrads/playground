//! Per-tenant rate limiter. Implement allow(tenant_id, timestamp) -> bool using a sliding window or token bucket, with separate limits per tenant.
//! This tests HashMap plus VecDeque handling and time-window edge cases: requests landing exactly on a window boundary, out-of-order timestamps, and memory growth with many idle tenants.
//! It maps to multi-tenant scale and API protection.
//!
//!
//! Given requests (tenant_id, timestamp_ms) in nondecreasing timestamp order, return whether each request is accepted.
//! Each tenant may have at most K accepted requests in the rolling window (t − W, t]. Rejected requests do not count.
//!     - Up to 200,000 requests.
//!     - Target: O(n) total time using hash maps and queues.
//!     - Follow-up: How would concurrent Lambda instances change the implementation?
//!     Tests: Identical timestamps, exact expiry boundary, interleaved tenants, long idle gaps.

// Design
//
// assume events are coming at right instant order (no going back in time)
//
// keep a window of timestamps per tenant_id, newest first: [t11, t5, t3, t1]
// assume window duration = 10s
// max requests = 4
//
// success scenario: when new request comes in @ t12:
// - trim the window to [t11, t5, t3]
// - add [t12, t11, t5, t3]
// - return true
//
// failure scenario: when new request comes in @ t12 (again):
// - trim the window to [t12, t11, t5, t3]
// - len == max requests (4) - fail
// - return false
//

use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    time::{Duration, Instant},
};

pub enum Policy {
    CountAccepted,
    CountAll,
}

pub struct RateLimiter<T> {
    max_requests: usize,
    window_duration: Duration,
    policy: Policy,
    limits: HashMap<T, VecDeque<Instant>>,
}

impl<T: Eq + Hash> RateLimiter<T> {
    pub fn new(max_requests: usize, window_duration: Duration, policy: Policy) -> Self {
        Self {
            max_requests,
            window_duration,
            policy,
            limits: Default::default(),
        }
    }

    pub fn allow(&mut self, tenant_id: T, now: Instant) -> bool {
        let window = self.limits.entry(tenant_id).or_default();
        // trim window to accommodate now - window_duration
        let i = window.partition_point(|&t| now - t < self.window_duration);
        window.truncate(i);
        let allowed = window.len() < self.max_requests;
        match self.policy {
            Policy::CountAccepted => {
                if allowed {
                    window.push_front(now)
                }
            }
            Policy::CountAll => {
                window.push_front(now);
                // only the newest max_requests can affect a decision
                window.truncate(self.max_requests);
            }
        }
        allowed
    }

    pub fn compact(&mut self, now: Instant) -> usize {
        let mut total = 0;
        self.limits.retain(|_, window| {
            let i = window.partition_point(|&t| now - t < self.window_duration);
            window.truncate(i);
            total += window.len();
            !window.is_empty()
        });
        total
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;

    fn fixture(policy: Policy) -> (Vec<Instant>, RateLimiter<&'static str>) {
        let now = Instant::now();
        let instants: Vec<_> = (0..100).map(|t| now + Duration::from_secs(t)).collect();

        let limiter = RateLimiter::new(4, Duration::from_secs(10), policy);
        (instants, limiter)
    }

    #[test]
    fn test_window_reject() {
        let tenant_id = "X";
        let (instants, mut limiter) = fixture(Policy::CountAccepted);

        assert!(limiter.allow(tenant_id, instants[1]));
        assert!(limiter.allow(tenant_id, instants[1]));
        assert!(limiter.allow(tenant_id, instants[1]));
        assert!(limiter.allow(tenant_id, instants[1]));
        assert!(!limiter.allow(tenant_id, instants[1]));
    }

    #[test]
    fn test_window() {
        let tenant_id = "X";
        let (instants, mut limiter) = fixture(Policy::CountAccepted);

        assert!(limiter.allow(tenant_id, instants[1]));
        assert!(limiter.allow(tenant_id, instants[3]));
        assert!(limiter.allow(tenant_id, instants[5]));
        assert!(limiter.allow(tenant_id, instants[11]));

        assert!(limiter.allow(tenant_id, instants[12]));
        assert!(!limiter.allow(tenant_id, instants[12]));
    }

    #[test]
    fn test_window_count_all() {
        let tenant_id = "X";
        let (instants, mut limiter) = fixture(Policy::CountAll);

        assert!(limiter.allow(tenant_id, instants[1]));
        assert!(limiter.allow(tenant_id, instants[3]));
        assert!(limiter.allow(tenant_id, instants[5]));
        assert!(limiter.allow(tenant_id, instants[11]));

        assert!(limiter.allow(tenant_id, instants[12]));
        assert!(!limiter.allow(tenant_id, instants[12]));
        // CountAccepted would accept: rejected instants[12] still fills the window
        assert!(!limiter.allow(tenant_id, instants[14]));
        // instants[11] expires, window: [14, 12, 12]
        assert!(limiter.allow(tenant_id, instants[21]));
    }

    #[test]
    fn test_multi_tenancy() {
        let tenant1_id = "X";
        let tenant2_id = "Y";
        let (instants, mut limiter) = fixture(Policy::CountAccepted);

        assert!(limiter.allow(tenant1_id, instants[1]));
        assert!(limiter.allow(tenant2_id, instants[1]));

        assert!(limiter.allow(tenant1_id, instants[3]));
        assert!(limiter.allow(tenant2_id, instants[3]));

        assert!(limiter.allow(tenant1_id, instants[5]));
        assert!(limiter.allow(tenant2_id, instants[5]));

        assert!(limiter.allow(tenant1_id, instants[11]));
        assert!(limiter.allow(tenant2_id, instants[11]));

        assert!(limiter.allow(tenant1_id, instants[12]));
        assert!(limiter.allow(tenant2_id, instants[12]));
    }

    #[test]
    fn test_compact() {
        let (instants, mut limiter) = fixture(Policy::CountAccepted);

        assert!(limiter.allow("X", instants[1]));
        assert!(limiter.allow("X", instants[1]));
        assert!(limiter.allow("Y", instants[1]));
        assert!(limiter.allow("Y", instants[8]));

        // X fully expired at exact boundary, Y keeps instants[8]
        assert_eq!(limiter.compact(instants[11]), 1);
        assert_eq!(limiter.limits.len(), 1);
        assert_eq!(limiter.limits["Y"], [instants[8]]);
    }
}
