use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use linux_mirrors::archlinux::internal::ArchMirrors;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error, trace};

use super::IoEvent;
use crate::app::App;
use crate::Result;
/// In the IO thread, we handle IO event without blocking the UI thread
pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
    mirrors_receiver: Receiver<ArchMirrors>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>, mirrors_receiver: Receiver<ArchMirrors>) -> Self {
        Self {
            app,
            mirrors_receiver,
        }
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialise => self.do_initialize().await,
            IoEvent::Sleep(duration) => self.do_sleep(duration).await,
        };

        if let Err(err) = result {
            error!("{err}");
        }

        let mut app = self.app.lock().await;
        app.loaded();
    }

    /// Get your mirrors here
    async fn do_initialize(&mut self) -> Result<()> {
        // get mirrors
        while let Some(mirrors) = &self.mirrors_receiver.recv().await {
            let mut app = self.app.lock().await;
            app.update_mirrors(mirrors);
        }
        let mut app = self.app.lock().await;
        app.initialized(); // we could update the app state
        debug!("👍 Application initialized");

        Ok(())
    }

    /// Just take a little break
    async fn do_sleep(&mut self, duration: Duration) -> Result<()> {
        trace!("sleeping for {:?}...", duration);
        let utc: DateTime<Utc> = Utc::now();
        tokio::time::sleep(duration).await;
        trace!("waking up");
        // Notify the app for having slept
        let mut app = self.app.lock().await;
        app.update_clock(utc);
        app.slept();
        Ok(())
    }
}
