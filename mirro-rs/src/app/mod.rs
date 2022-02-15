use chrono::{DateTime, Utc};
use linux_mirrors::archlinux::internal::ArchMirrors;
use tracing::{error, trace};
use tui::widgets::TableState;

use crate::{inputs::key::Key, io::IoEvent};

use self::{
    actions::{Action, Actions},
    state::{AppState, SelectedCountry, Widgets},
};

pub mod actions;
pub mod state;
pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

enum ScrollableTables {
    AllMirrors,
    SavedMirrors,
}

/// The main application, containing the state
pub struct App {
    is_loading: bool,
    actions: Actions,
    /// We could dispatch an IO event
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    state: AppState,
    mirrors: ArchMirrors,
    country_filter: String,
    clock: DateTime<Utc>,
    table: TableState,
    selected_countries: Vec<SelectedCountry>,
    selected_table: TableState,
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
            country_filter: String::default(),
            clock: Utc::now(),
            table: TableState::default(),
            selected_table: TableState::default(),
            selected_countries: vec![],
        }
    }

    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            trace!("Using action {}", &action);
            key_handler(*action, self, key).await
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
            error!("Error from dispatch {}", e);
        };
    }

    pub fn initialized(&mut self) {
        // Update contextual actions
        self.actions = vec![
            Action::Quit,
            Action::Sleep,
            Action::Focus(Widgets::CountryFilter),
            Action::Focus(Widgets::Protocols),
            Action::Focus(Widgets::Mirrors),
            Action::Focus(Widgets::SelectedCountries),
            Action::Action,
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

    pub fn update_clock(&mut self, clock: DateTime<Utc>) {
        self.clock = clock;
    }

    pub fn update_mirrors(&mut self, mirrors: &ArchMirrors) {
        self.mirrors = mirrors.clone();
    }

    fn scroll_prev(&mut self, table: ScrollableTables) {
        let (state, items) = self.table_info(table);
        let i = match state.selected() {
            Some(i) => {
                if i >= items - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    fn scroll_next(&mut self, table: ScrollableTables) {
        let (state, items) = self.table_info(table);
        let i = match state.selected() {
            Some(i) => {
                if i == 0 {
                    items - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    fn table_info(&mut self, table: ScrollableTables) -> (&mut TableState, usize) {
        match table {
            ScrollableTables::AllMirrors => (&mut self.table, self.mirrors.countries.len()),
            ScrollableTables::SavedMirrors => {
                (&mut self.selected_table, self.selected_countries.len())
            }
        }
    }
}

async fn key_handler(action: Action, app: &mut App, key: Key) -> AppReturn {
    if let Some(focused_widget) = app.state.focused_widget() {
        match action {
            Action::Quit => AppReturn::Exit,
            Action::Sleep => {
                if let Some(duration) = app.state.duration().cloned() {
                    // Sleep is an I/O action, we dispatch on the IO channel that's run on another thread
                    app.dispatch(IoEvent::Sleep(duration)).await
                }
                AppReturn::Continue
            }
            Action::Focus(widget) => match widget {
                Widgets::CountryFilter => {
                    trace!("country widget focused");
                    app.state.update_focused_widget(Widgets::CountryFilter);
                    AppReturn::Continue
                }
                Widgets::Protocols => {
                    trace!("protocols widget focused");
                    app.state.update_focused_widget(Widgets::Protocols);
                    AppReturn::Continue
                }
                Widgets::Mirrors => {
                    trace!("mirrors widget focused");
                    app.state.update_focused_widget(Widgets::Mirrors);
                    AppReturn::Continue
                }
                Widgets::SelectedCountries => {
                    trace!("selected countries widget focused");
                    app.state.update_focused_widget(Widgets::SelectedCountries);
                    AppReturn::Continue
                }
            },
            Action::Action => {
                match focused_widget {
                    Widgets::CountryFilter => match key {
                        Key::Backspace => {
                            app.country_filter.pop();
                        }
                        Key::Char(ch) => app.country_filter.push(ch),
                        _ => {}
                    },
                    Widgets::Protocols => todo!(),
                    Widgets::Mirrors => match key {
                        Key::Enter | Key::Char(' ') => {
                            if let Some(index) = app.table.selected() {
                                if let Some(f) = app.mirrors.countries.get(index) {
                                    let country = &f.country;
                                    if !app
                                        .selected_countries
                                        .iter()
                                        .any(|w| w.country.country.eq(country))
                                    {
                                        app.selected_countries.push(SelectedCountry {
                                            country: f.to_owned(),
                                            search_item: app.country_filter.clone(),
                                            index: index.try_into().unwrap(),
                                        });
                                    }
                                };
                            };
                        }
                        Key::Esc => todo!(),
                        Key::Up | Key::Char('k') => app.scroll_next(ScrollableTables::AllMirrors),
                        Key::Down | Key::Char('j') => app.scroll_prev(ScrollableTables::AllMirrors),
                        _ => {}
                    },
                    Widgets::SelectedCountries => match key {
                        Key::Enter | Key::Char(' ') => {
                            todo!()
                        }
                        Key::Esc => todo!(),
                        Key::Up | Key::Char('k') => app.scroll_next(ScrollableTables::SavedMirrors),
                        Key::Down | Key::Char('j') => {
                            app.scroll_prev(ScrollableTables::SavedMirrors)
                        }
                        _ => {}
                    },
                }
                AppReturn::Continue
            }
        }
    } else {
        AppReturn::Continue
    }
}
