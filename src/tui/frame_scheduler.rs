use std::time::{Duration, Instant};

#[derive(Debug)]
pub(crate) struct FrameScheduler {
    min_frame_interval: Duration,
    last_redraw_at: Option<Instant>,
    next_redraw_at: Option<Instant>,
}

impl FrameScheduler {
    pub(crate) fn new(target_fps: u16) -> Self {
        let target_fps = target_fps.max(1);
        Self {
            min_frame_interval: Duration::from_nanos(1_000_000_000 / u64::from(target_fps)),
            last_redraw_at: None,
            next_redraw_at: None,
        }
    }

    pub(crate) fn min_frame_interval(&self) -> Duration {
        self.min_frame_interval
    }

    pub(crate) fn request_redraw_now(&mut self, now: Instant) {
        self.request_redraw_at(now, now);
    }

    pub(crate) fn request_redraw_after(&mut self, now: Instant, delay: Duration) {
        self.request_redraw_at(now, now + delay);
    }

    pub(crate) fn next_wait_duration(&self, now: Instant) -> Option<Duration> {
        self.next_redraw_at
            .map(|deadline| deadline.saturating_duration_since(now))
    }

    pub(crate) fn consume_due_redraw(&mut self, now: Instant) -> bool {
        if self.next_redraw_at.is_some_and(|deadline| deadline <= now) {
            self.next_redraw_at = None;
            self.last_redraw_at = Some(now);
            return true;
        }

        false
    }

    fn request_redraw_at(&mut self, now: Instant, requested_at: Instant) {
        let capped_at = self.capped_deadline(now, requested_at);
        self.next_redraw_at = Some(match self.next_redraw_at {
            Some(existing) => existing.min(capped_at),
            None => capped_at,
        });
    }

    fn capped_deadline(&self, now: Instant, requested_at: Instant) -> Instant {
        let requested_at = requested_at.max(now);
        match self.last_redraw_at {
            Some(last_redraw_at) => requested_at.max(last_redraw_at + self.min_frame_interval),
            None => requested_at,
        }
    }

    #[cfg(test)]
    fn next_redraw_at(&self) -> Option<Instant> {
        self.next_redraw_at
    }
}

#[cfg(test)]
mod tests {
    use super::FrameScheduler;
    use std::time::{Duration, Instant};

    #[test]
    fn coalesces_multiple_immediate_redraw_requests() {
        let now = Instant::now();
        let mut scheduler = FrameScheduler::new(60);

        scheduler.request_redraw_now(now);
        scheduler.request_redraw_now(now + Duration::from_millis(1));

        assert_eq!(scheduler.next_redraw_at(), Some(now));
        assert!(scheduler.consume_due_redraw(now));
        assert!(!scheduler.consume_due_redraw(now));
    }

    #[test]
    fn respects_min_frame_interval_after_a_draw() {
        let now = Instant::now();
        let mut scheduler = FrameScheduler::new(60);

        scheduler.request_redraw_now(now);
        assert!(scheduler.consume_due_redraw(now));

        let next_request_at = now + Duration::from_millis(1);
        scheduler.request_redraw_now(next_request_at);

        assert_eq!(
            scheduler.next_redraw_at(),
            Some(now + scheduler.min_frame_interval())
        );
    }

    #[test]
    fn reports_correct_wait_duration_for_scheduled_redraw() {
        let now = Instant::now();
        let mut scheduler = FrameScheduler::new(60);

        scheduler.request_redraw_after(now, Duration::from_millis(40));

        assert_eq!(
            scheduler.next_wait_duration(now + Duration::from_millis(10)),
            Some(Duration::from_millis(30))
        );
        assert_eq!(
            scheduler.next_wait_duration(now + Duration::from_millis(50)),
            Some(Duration::ZERO)
        );
    }
}
