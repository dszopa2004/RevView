use std::time::{Duration, Instant};

use crate::commands;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetryKind {
    Rpm,
    Speed,
    CoolantTemp,
    IntakeTemp,
    ThrottlePosition,
}

impl TelemetryKind {
    pub fn command(self) -> &'static str {
        match self {
            Self::Rpm => commands::RPM_COMMAND,
            Self::Speed => commands::SPEED_COMMAND,
            Self::CoolantTemp => commands::COOLANT_TEMP_COMMAND,
            Self::IntakeTemp => commands::INTAKE_TEMP_COMMAND,
            Self::ThrottlePosition => commands::THROTTLE_POSITION_COMMAND,
        }
    }

    pub fn interval(self) -> Duration {
        match self {
            Self::Rpm => Duration::from_millis(100),
            Self::Speed => Duration::from_millis(200),
            Self::ThrottlePosition => Duration::from_millis(100),
            Self::CoolantTemp => Duration::from_millis(1000),
            Self::IntakeTemp => Duration::from_millis(1000),
        }
    }
}

struct PollTask {
    kind: TelemetryKind,
    next_due: Instant,
    interval: Duration,
}

pub struct PollingScheduler {
    tasks: Vec<PollTask>,
    next_request_at: Instant,
    min_request_interval: Duration,
}

impl PollingScheduler {
    pub fn new() -> Self {
        let now = Instant::now();

        let kinds = [
            TelemetryKind::Rpm,
            TelemetryKind::Speed,
            TelemetryKind::ThrottlePosition,
            TelemetryKind::CoolantTemp,
            TelemetryKind::IntakeTemp,
        ];

        let tasks = kinds
            .iter()
            .map(|&kind| PollTask {
                kind,
                next_due: now,
                interval: kind.interval(),
            })
            .collect();

        Self {
            tasks,
            next_request_at: now,
            min_request_interval: Duration::from_millis(50),
        }
    }

    // Returns the next telemetry request if one is due.
    pub fn next_due(&self, now: Instant) -> Option<TelemetryKind> {
        if now < self.next_request_at {
            return None;
        }

        self.tasks
            .iter()
            .filter(|task| task.next_due <= now)
            .min_by_key(|task| task.next_due)
            .map(|task| task.kind)
    }

    // Call after each request, whether successful or not.
    pub fn complete(&mut self, kind: TelemetryKind, completed_at: Instant) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.kind == kind) {
            // Schedule from completion rather than the previous deadline.
            // This prevents a backlog of missed requests.
            task.next_due = completed_at + task.interval;
        }

        self.next_request_at = completed_at + self.min_request_interval;
    }

    // Returns how long until another request can be sent.
    pub fn time_until_next_request(&self, now: Instant) -> Duration {
        let earliest_due = self
            .tasks
            .iter()
            .map(|task| task.next_due)
            .min()
            .unwrap_or(now);

        let ready_at = earliest_due.max(self.next_request_at);

        ready_at.saturating_duration_since(now)
    }
}
