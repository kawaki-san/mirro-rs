use mirro_rs::{
    app::{config::MirrorsConfig, App},
    io::handler::IoAsyncHandler,
    start_ui,
};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tracing::error;

#[tokio::main]
async fn main() -> mirro_rs::Result<()> {
    let (_guard, config) = initialise_app();
    let (mirrors_tx, mirrors_rx) = tokio::sync::mpsc::channel(16);

    /* Sharing the IoEvents between threads */
    let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel(100);

    // Since application state can be accessed and mutated across threads
    let app = Arc::new(Mutex::new(App::new(sync_io_tx.clone(), config)));
    let app_ui = Arc::clone(&app);
    let app_clock = Arc::clone(&app);

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
                error!("{e}");
                let local_file = include_str!("../../assets/arch_mirrors.json");
                serde_json::from_str(local_file).expect("could not load backup file")
            }
        };
        mirrors_tx.send(mirrors).await
    });
    tokio::spawn(async move {
        loop {
            let mut app = app_clock.lock().await;
            let dt = chrono::offset::Local::now();
            app.update_clock(dt);
            tokio::time::sleep(Duration::from_micros(100)).await;
        }
    });

    start_ui(app_ui).await?;
    Ok(())
}

fn initialise_app() -> (tracing_appender::non_blocking::WorkerGuard, MirrorsConfig) {
    let m = clap::app_from_crate!()
        .arg(
            clap::Arg::new("log level")
                .takes_value(true)
                .short('l')
                .long("log")
                .help("Override the default ['trace'] log level"),
        )
        .arg(
            clap::Arg::new("config")
                .takes_value(true)
                .short('c')
                .long("config")
                .help("Read custom config.toml file [uses $XDG_CONFIG_HOME if not specified]"),
        )
        .get_matches();
    let mut log_valid = true;
    let log_level = match m.value_of("log level") {
        Some(val) => match val.to_lowercase().as_str() {
            "warn" => tracing::Level::WARN,
            "info" => tracing::Level::INFO,
            "error" => tracing::Level::ERROR,
            "debug" => tracing::Level::DEBUG,
            "trace" => tracing::Level::TRACE,
            _ => {
                log_valid = false;
                tracing::Level::TRACE
            }
        },
        None => tracing::Level::DEBUG,
    };

    let configuration: MirrorsConfig = match m.value_of("config") {
        None => try_default(),
        Some(conf) => match std::fs::read_to_string(conf) {
            Ok(str) => match toml::from_str(&str) {
                Ok(f) => f,
                Err(e) => {
                    error!("{}", e);
                    try_default()
                }
            },
            Err(e) => {
                tracing::error!("{}: {}", conf, e);
                try_default()
            }
        },
    };

    setup_logger((log_level, log_valid), configuration)
}

fn try_default() -> MirrorsConfig {
    let defaults = include_str!("../../mirro-rs.toml");
    toml::from_str(defaults).unwrap()
}
fn setup_logger(
    log_level: (tracing::Level, bool),
    configuration: MirrorsConfig,
) -> (tracing_appender::non_blocking::WorkerGuard, MirrorsConfig) {
    let file_appender = tracing_appender::rolling::daily("/tmp", "mirro-rs-log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(file_writer)
        .with_ansi(false)
        .init();
    tracing::info!(
        "{} {} has started",
        clap::crate_name!(),
        clap::crate_version!()
    );
    if !log_level.1 {
        tracing::error!("invalid log level passed, using default: [debug]");
    }
    (guard, configuration)
}
