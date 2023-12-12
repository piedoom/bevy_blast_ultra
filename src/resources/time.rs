use bevy::prelude::*;
use std::time::Duration;

/// Counts the game time elapsed
#[derive(serde::Deserialize, Clone, Resource)]
pub struct GameTime {
    pub start: Duration,
    pub end: Option<Duration>,
}

impl From<Duration> for GameTime {
    fn from(value: Duration) -> Self {
        GameTime {
            start: value,
            end: None,
        }
    }
}

/// Get the amount of time elapsed since the start of the game
impl GameTime {
    pub fn reset(&mut self, time: &Time) {
        self.start = time.elapsed();
        self.end = None;
    }

    pub fn elapsed(&self, time: &Time) -> Duration {
        time.elapsed() - self.start
    }

    pub fn finish(&mut self, time: &Time) {
        self.end = Some(time.elapsed());
    }

    pub fn final_time(&self) -> Option<Duration> {
        self.end.map(|end| end - self.start)
    }
}
