use log::{info, LevelFilter};
use std::io::Write;
use std::sync::Once;

static INIT: Once = Once::new();

const COLOR_RESET: &str = "\x1b[0m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_YELLOW: &str = "\x1b[33m";
pub fn init_logger() {
    INIT.call_once(|| {
        env_logger::Builder::new()
            .format(|buf, record| {
                let color = match record.level() {
                    log::Level::Error => COLOR_RED,
                    log::Level::Warn => COLOR_YELLOW,
                    log::Level::Info => COLOR_GREEN,
                    _ => COLOR_RESET,
                };
                writeln!(
                    buf,
                    "{}{:<5} - {}:{} - {}{}",
                    color,
                    record.level(),
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    record.args(),
                    COLOR_RESET
                )
            })
            .filter(None, LevelFilter::Info)
            .init();
    });
    info!("Logger init finished!")
}
