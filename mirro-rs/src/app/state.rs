use std::time::Duration;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        duration: Duration,
        counter_sleep: u32,
        counter_tick: u64,
        focused_widget: Widgets,
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Widgets {
    CountryFilter,
    Protocols,
    Mirrors,
}

impl AppState {
    pub fn initialized() -> Self {
        let duration = Duration::from_secs(1);
        let counter_sleep = 0;
        let counter_tick = 0;
        Self::Initialized {
            duration,
            counter_sleep,
            counter_tick,
            focused_widget: Widgets::CountryFilter,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_sleep(&mut self) {
        if let Self::Initialized { counter_sleep, .. } = self {
            *counter_sleep += 1;
        }
    }

    pub fn incr_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    pub fn count_sleep(&self) -> Option<u32> {
        if let Self::Initialized { counter_sleep, .. } = self {
            Some(*counter_sleep)
        } else {
            None
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized { duration, .. } = self {
            Some(duration)
        } else {
            None
        }
    }
    pub fn focused_widget(&self) -> Option<&Widgets> {
        match &self {
            AppState::Init => None,
            AppState::Initialized {
                duration: _,
                counter_sleep: _,
                counter_tick: _,
                focused_widget,
            } => Some(focused_widget),
        }
    }

    pub fn update_focused_widget(&mut self, widget: Widgets) {
        let duration = Duration::from_secs(1);
        let counter_sleep = 0;
        let counter_tick = 0;
        *self = Self::Initialized {
            duration,
            counter_sleep,
            counter_tick,
            focused_widget: widget,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
