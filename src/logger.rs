use log::{Level, LevelFilter, Metadata, Record};
use owo_colors::{OwoColorize, Stream, Style};

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
                    "error:".if_supports_color(Stream::Stderr, |text| text
                        .style(Style::new().red().bold())),
                    record.args(),
                ),
                Level::Warn => eprintln!(
                    "{} {}",
                    "warn:".if_supports_color(Stream::Stderr, |text| text
                        .style(Style::new().yellow().bold())),
                    record.args(),
                ),
                _ => println!("{}", record.args(),),
            }
        }
    }

    fn flush(&self) {}
}
