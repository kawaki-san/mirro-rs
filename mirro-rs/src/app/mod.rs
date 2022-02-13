use linux_mirrors::archlinux::internal::ArchMirrors;

use crate::{inputs::key::Key, io::IoEvent};

use self::{
    actions::{Action, Actions},
    state::AppState,
};

pub mod actions;
pub mod state;
pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

/// The main application, containing the state
pub struct App {
    is_loading: bool,
    actions: Actions,
    /// We could dispatch an IO event
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    state: AppState,
    mirrors: ArchMirrors,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self {
        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let state = AppState::default();

        Self {
            io_tx,
            actions,
            is_loading,
            state,
            mirrors: ArchMirrors::default(),
        }
    }

    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                actions::Action::Quit => AppReturn::Exit,
                actions::Action::Sleep => {
                    if let Some(duration) = self.state.duration().cloned() {
                        // Sleep is an I/O action, we dispatch on the IO channel that's run on another thread
                        self.dispatch(IoEvent::Sleep(duration)).await
                    }
                    AppReturn::Continue
                }
                actions::Action::IncrementDelay => {
                    // TODO
                    AppReturn::Continue
                }
                actions::Action::DecrementDelay => {
                    // TODO
                    AppReturn::Continue
                }
            }
        } else {
            // No action associated with key
            AppReturn::Continue
        }
    }

    pub async fn update_on_tick(&mut self) -> AppReturn {
        // TODO update clock
        AppReturn::Continue
    }

    /// Send a network event to the IO thread
    pub async fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in io/handler.rs
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action).await {
            self.is_loading = false;
            eprintln!("Error from dispatch {}", e);
        };
    }

    pub fn initialized(&mut self) {
        // Update contextual actions
        self.actions = vec![
            Action::Quit,
            Action::Sleep,
            Action::IncrementDelay,
            Action::DecrementDelay,
        ]
        .into();
        self.state = AppState::initialized()
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }

    pub fn slept(&mut self) {
        self.state.incr_sleep();
    }

    pub fn update_mirrors(&mut self, mirrors: ArchMirrors) {
        self.mirrors = mirrors;
    }
}
