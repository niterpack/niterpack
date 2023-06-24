use console::style;
use log::{Level, LevelFilter, Metadata, Record};

pub static LOGGER: NiterLogger = NiterLogger;

pub fn init() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("could not set logger")
}

pub struct NiterLogger;

impl log::Log for NiterLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => eprintln!(
                    "{} {}",
                    style("error:").for_stderr().red().bold(),
                    record.args(),
                ),
                Level::Warn => eprintln!(
                    "{} {}",
                    style("warn:").for_stderr().yellow().bold(),
                    record.args(),
                ),
                _ => println!("{}", record.args(),),
            }
        }
    }

    fn flush(&self) {}
}
