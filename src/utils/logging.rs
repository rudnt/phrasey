use anyhow::Context;
use log::debug;

use crate::types::LogLevel;

pub fn init(log_level: &LogLevel, log_dir_uri: &Option<String>) -> anyhow::Result<()> {
    let mut dispatcher = create_dispatcher(log_level);

    if log_level != &LogLevel::Off {
        dispatcher = set_output(dispatcher, log_dir_uri)?;
    }

    dispatcher.apply().context("Failed to apply logging configuration")?;
    debug!("Logging initialized with log_dir_uri: {:?}", log_dir_uri);
    Ok(())
}
    
fn set_output(mut dispatcher: fern::Dispatch, log_dir_uri: &Option<String>) -> anyhow::Result<fern::Dispatch> {
    if let Some(uri) = log_dir_uri {
        let dirpath = uri.strip_prefix("file://").context("Failed to parse log_dir_uri")?;
        let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
        let filename = format!("phrasey_{}.log", timestamp);
        let filepath = std::path::Path::new(dirpath).join(filename);
        let file = std::fs::File::create(filepath).context("Failed to create log file")?;
        dispatcher = dispatcher.chain(file);
    } else {
        dispatcher = dispatcher.chain(std::io::stderr());
    }

    Ok(dispatcher)
}

fn create_dispatcher(log_level: &LogLevel) -> fern::Dispatch {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{:5}] {}",
                chrono::Local::now().format("%H:%M:%S%.3f"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::from(log_level))
}

impl From<&LogLevel> for log::LevelFilter {
    fn from(level: &LogLevel) -> Self {
        match level {
            LogLevel::Off => log::LevelFilter::Off,
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Trace => log::LevelFilter::Trace,
        }
    }
}