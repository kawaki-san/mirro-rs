use chrono::{DateTime, Local};
use linux_mirrors::archlinux::internal::{ArchMirrors, Url};
use tracing::{error, trace};
use tui::widgets::TableState;

use crate::{inputs::key::Key, io::IoEvent};

use self::{
    actions::{Action, Actions},
    state::{AppState, SelectedCountry, Widgets},
};

pub mod actions;
pub mod export;
pub mod state;
pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

#[derive(Clone, Copy, Debug)]
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
    clock: DateTime<Local>,
    table: TableState,
    selected_countries: Vec<SelectedCountry>,
    selected_table: TableState,
    focused_country: Url,
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
            clock: Local::now(),
            table: TableState::default(),
            selected_table: TableState::default(),
            selected_countries: vec![],
            focused_country: Url::default(),
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
            Action::SimpleExport,
            Action::RateExport,
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

    pub fn update_clock(&mut self, clock: DateTime<Local>) {
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
        self.update_mirrors_widget(table, i);
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
        self.update_mirrors_widget(table, i);
    }

    fn update_mirrors_widget(&mut self, table: ScrollableTables, index: usize) {
        match table {
            ScrollableTables::AllMirrors => {}
            ScrollableTables::SavedMirrors => {
                if let Some(country) = self.selected_countries.get(index) {
                    self.focused_country = country.country.clone()
                }
            }
        }
    }

    fn table_info(&mut self, table: ScrollableTables) -> (&mut TableState, usize) {
        match table {
            ScrollableTables::AllMirrors => (&mut self.table, self.mirrors.countries.len()),
            ScrollableTables::SavedMirrors => {
                (&mut self.selected_table, self.selected_countries.len())
            }
        }
    }

    pub fn focused_country(&self) -> &Url {
        &self.focused_country
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
                            if app.table.selected().is_some() {
                                app.table.select(None);
                            }
                        }
                        Key::Char(ch) => {
                            app.country_filter.push(ch);
                            if app.table.selected().is_some() {
                                app.table.select(None);
                            }
                        }
                        Key::Ctrl('a') => {
                            app.state.update_focused_widget(Widgets::Mirrors);
                        }
                        _ => {}
                    },
                    Widgets::Protocols => todo!(),
                    Widgets::Mirrors => match key {
                        Key::Enter | Key::Char(' ') => {
                            if let Some(index) = app.table.selected() {
                                let inner_list: Vec<_> = app
                                    .mirrors
                                    .countries
                                    .iter()
                                    .filter(|f| {
                                        f.country
                                            .to_lowercase()
                                            .contains(&app.country_filter.to_lowercase())
                                    })
                                    .collect();
                                if let Some(f) = inner_list.get(index) {
                                    let country = &f.country;
                                    if !app
                                        .selected_countries
                                        .iter()
                                        .any(|w| w.country.country.eq(country))
                                    {
                                        app.selected_countries.push(SelectedCountry {
                                            country: (*f).clone(),
                                            search_item: app.country_filter.clone(),
                                            index: index.try_into().unwrap(),
                                        });
                                    }
                                };
                            };
                        }
                        Key::Up | Key::Char('k') => app.scroll_next(ScrollableTables::AllMirrors),
                        Key::Down | Key::Char('j') => app.scroll_prev(ScrollableTables::AllMirrors),
                        _ => {}
                    },
                    Widgets::SelectedCountries => match key {
                        Key::Enter | Key::Char(' ') => {
                            if let Some(index) = app.selected_table.selected() {
                                if !app.selected_countries.is_empty() {
                                    app.selected_countries.remove(index);
                                    if !app.selected_countries.is_empty() {
                                        app.selected_table.select(Some(index));
                                    } else {
                                        app.selected_table.select(None)
                                    }
                                }
                            };
                        }
                        Key::Up | Key::Char('k') => app.scroll_next(ScrollableTables::SavedMirrors),
                        Key::Down | Key::Char('j') => {
                            app.scroll_prev(ScrollableTables::SavedMirrors);
                        }
                        _ => {}
                    },
                }
                AppReturn::Continue
            }
            Action::SimpleExport => {
                export::export_mirrors(app.selected_countries.clone(), false).await
            }
            Action::RateExport => {
                export::export_mirrors(app.selected_countries.clone(), true).await
            }
        }
    } else {
        AppReturn::Continue
    }
}
