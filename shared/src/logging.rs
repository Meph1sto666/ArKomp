use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};
static FILE_GUARD: std::sync::Mutex<
    Option<std::sync::Arc<std::sync::Mutex<tracing_appender::non_blocking::WorkerGuard>>>,
> = std::sync::Mutex::new(None);

pub fn init_logger()
-> Result<std::sync::Arc<std::sync::Mutex<WorkerGuard>>, Box<dyn std::error::Error>> {
    std::fs::create_dir_all("logs/")?;
    let file_appender = tracing_appender::rolling::daily("./logs/", "arkomp.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let guard_arc = std::sync::Arc::new(std::sync::Mutex::new(guard));
    *FILE_GUARD.lock().unwrap() = Some(guard_arc.clone());

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_thread_names(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_filter(LevelFilter::TRACE);

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true) // Keep colors for console
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_thread_names(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(stdout_layer)
        .init();

    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!(
            "{}\nBACKTRACE:\n{}",
            panic_info.to_string().replacen("\n", " ", 1),
            std::backtrace::Backtrace::force_capture().to_string()
        );
    }));

    tracing::info!("logger initialised");
    Ok(guard_arc.clone())
}
