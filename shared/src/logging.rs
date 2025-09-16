pub fn init_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let now = chrono::Utc::now();
            out.finish(format_args!(
                "{} [{:<7}] : [{}] @{} : {}",
                now.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                message
            ))
        })
        .chain(fern::log_file(current_log_file()).unwrap())
        .level(log::LevelFilter::Debug)
        .apply()?;

    std::panic::set_hook(Box::new(|panic_info| {
        log::error!(
            "{}\nBACKTRACE:\n{}",
            panic_info.to_string().replacen("\n", " ", 1),
            std::backtrace::Backtrace::force_capture().to_string()
        );
    }));

    log::info!("logger initialised");
    Ok(())
}

fn current_log_file() -> String {
    std::fs::create_dir_all("logs/").unwrap();
    format!("logs/{}.log", chrono::Utc::now().date_naive().to_string())
}
