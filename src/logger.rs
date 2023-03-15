use log::{Level, LevelFilter, Metadata, Record};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub fn init() {
    log::set_boxed_logger(Box::new(NiterLogger::new()))
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("could not set logger")
}

pub struct NiterLogger {
    writer: BufferWriter,
    err_writer: BufferWriter
}

impl NiterLogger {
    pub fn new() -> NiterLogger {
        NiterLogger {
            writer: BufferWriter::stdout(ColorChoice::Auto),
            err_writer: BufferWriter::stderr(ColorChoice::Auto)
        }
    }
}

impl log::Log for NiterLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            use std::io::Write;

            match record.level() {
                Level::Error => {
                    let mut buffer = self.err_writer.buffer();
                    buffer
                        .set_color(ColorSpec::new()
                            .set_fg(Some(Color::Red))
                            .set_bold(true))
                        .and_then(|_| write!(buffer, "error:"))
                        .and_then(|_| buffer.reset())
                        .and_then(|_| write!(buffer, " {}\n", record.args()))
                        .and_then(|_| self.err_writer.print(&buffer))
                },
                Level::Warn => {
                    let mut buffer = self.err_writer.buffer();
                    buffer
                        .set_color(ColorSpec::new()
                            .set_fg(Some(Color::Yellow))
                            .set_bold(true))
                        .and_then(|_| write!(buffer, "warning:"))
                        .and_then(|_| buffer.reset())
                        .and_then(|_| write!(buffer, " {}\n", record.args()))
                        .and_then(|_| self.err_writer.print(&buffer))
                }
                _ => {
                    let mut buffer = self.writer.buffer();
                    buffer
                        .reset()
                        .and_then(|_| write!(buffer, "{}\n", record.args()))
                        .and_then(|_| self.writer.print(&buffer))
                }
            }.expect("could not write to logger buffer");
        }
    }

    fn flush(&self) {}
}

