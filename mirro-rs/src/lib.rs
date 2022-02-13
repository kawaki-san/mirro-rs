use std::{io::stdout, sync::Arc, time::Duration};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};

use self::{
    app::{ui, App, AppReturn},
    inputs::{events::Events, InputEvent},
    io::IoEvent,
};

pub mod app;
pub mod inputs;
pub mod io;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn start_ui(app: Arc<Mutex<App>>) -> Result<()> {
    let stdout = stdout();
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // User event handler - captures inputs
    let tick_rate = Duration::from_millis(100);
    let mut events = Events::new(tick_rate);

    {
        let mut app = app.lock().await;
        // Here we assume the the first load is a long task
        app.dispatch(IoEvent::Initialise).await;
    }

    loop {
        let mut app = app.lock().await;
        // Render
        terminal.draw(|rect| ui::draw(rect, &app))?;

        /*
         * Handle inputs
         * Block the thread for at most @tick_rate to receive an input. For this application, it's
         * not necessary to reach 60fps, so it's enough to refresh every 100ms
         *
         * */
        let result = match events.next().await {
            /*
             * Process the user input, Application mutation is possible
             * */
            InputEvent::Input(key) => app.do_action(key).await,

            /*
             * No user input, however we may have to be doing some processing here, so mutation is
             * also a possibility
             * */
            InputEvent::Tick => app.update_on_tick().await,
        };
        /* Check if we should exit
         * Did user press q or Ctrl+c?
         * */
        if result == AppReturn::Exit {
            events.close();
            break;
        }
        /* One of the @do_action or @update_on_tick methods have to return AppReturn::Exit (instead
         * of AppReturn::Continue) to quit the application
         * */
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    disable_raw_mode()?;
    Ok(())
}
