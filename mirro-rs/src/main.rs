use mirro_rs::{app::App, io::handler::IoAsyncHandler, start_ui};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> mirro_rs::Result<()> {
    let (mirrors_tx, mirrors_rx) = tokio::sync::mpsc::channel(16);

    /* Sharing the IoEvents between threads */
    let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel(100);

    // Since application state can be accessed and mutated across threads
    let app = Arc::new(Mutex::new(App::new(sync_io_tx.clone())));
    let app_ui = Arc::clone(&app);

    // New thread to process @IoEvent. The @IoEvent processing loop delegates to the @IoAsyncHandler
    tokio::spawn(async move {
        let mut handler = IoAsyncHandler::new(app, mirrors_rx);
        while let Some(io_event) = sync_io_rx.recv().await {
            handler.handle_io_event(io_event).await;
        }
    });
    tokio::spawn(async move {
        let mirrors = match linux_mirrors::archlinux::mirrors().await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("{e}");
                let local_file = include_str!("../../assets/arch_mirrors.json");
                serde_json::from_str(local_file).unwrap()
            }
        };
        mirrors_tx.send(mirrors).await
    });
    start_ui(app_ui).await?;
    Ok(())
}
