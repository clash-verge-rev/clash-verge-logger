use std::{io::Write, thread};

use flexi_logger::DeferredNow;
use log::{LevelFilter, Record};
#[cfg(feature = "color")]
use nu_ansi_term::Color;

use std::borrow::Cow;

pub fn level_filter_to_string(log_level: &LevelFilter) -> Cow<'static, str> {
    #[cfg(feature = "color")]
    {
        match log_level {
            LevelFilter::Off => Cow::Owned(Color::Fixed(8).paint("OFF").to_string()),
            LevelFilter::Error => Cow::Owned(Color::Red.paint("ERROR").to_string()),
            LevelFilter::Warn => Cow::Owned(Color::Yellow.paint("WARN ").to_string()),
            LevelFilter::Info => Cow::Owned(Color::Green.paint("INFO ").to_string()),
            LevelFilter::Debug => Cow::Owned(Color::Blue.paint("DEBUG").to_string()),
            LevelFilter::Trace => Cow::Owned(Color::Purple.paint("TRACE").to_string()),
        }
    }
    #[cfg(not(feature = "color"))]
    {
        match log_level {
            LevelFilter::Off => Cow::Borrowed("OFF"),
            LevelFilter::Error => Cow::Borrowed("ERROR"),
            LevelFilter::Warn => Cow::Borrowed("WARN"),
            LevelFilter::Info => Cow::Borrowed("INFO"),
            LevelFilter::Debug => Cow::Borrowed("DEBUG"),
            LevelFilter::Trace => Cow::Borrowed("TRACE"),
        }
    }
}

pub fn console_format(
    w: &mut dyn Write,
    now: &mut DeferredNow,
    record: &Record,
) -> std::io::Result<()> {
    let current_thread = thread::current();
    let thread_name = current_thread.name().unwrap_or("unnamed");

    let level = level_filter_to_string(&record.level().to_level_filter());
    let line = record.line().map_or(0, |l| l);
    let module = record.module_path().unwrap_or("<unnamed>");

    write!(
        w,
        "[{}] {} [{}:{}] T[{}] {}",
        now.format("%H:%M:%S%.3f"),
        level,
        module,
        line,
        thread_name,
        record.args(),
    )
}

pub fn file_format_with_level(
    w: &mut dyn Write,
    now: &mut DeferredNow,
    record: &Record,
) -> std::io::Result<()> {
    write!(
        w,
        "[{}] {} {}",
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        record.level(),
        record.args(),
    )
}

pub fn file_format_without_level(
    w: &mut dyn Write,
    now: &mut DeferredNow,
    record: &Record,
) -> std::io::Result<()> {
    write!(
        w,
        "[{}] {}",
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        record.args(),
    )
}
