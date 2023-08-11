use console::style;
use eyre::Chain;
use log::{Level, LevelFilter, Metadata, Record};

pub static LOGGER: NiterLogger = NiterLogger;

pub fn install() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("failed to set set logger");

    eyre::set_hook(Box::new(move |_| Box::new(NiterLogger))).expect("failed to set eyre hook");
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

impl eyre::EyreHandler for NiterLogger {
    fn debug(
        &self,
        error: &(dyn std::error::Error + 'static),
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write!(f, "{}", error)?;

        if let Some(source) = error.source() {
            write!(f, "\n\n{}", style("caused by:").bold())?;

            let chain = Chain::new(source);

            if chain.len() == 1 {
                write!(f, "\n  {}", source)?;
            } else {
                for (i, error) in Chain::new(source).enumerate() {
                    write!(f, "\n  {}: {}", i, error)?;
                }
            }
        }

        Ok(())
    }

    fn display(
        &self,
        error: &(dyn std::error::Error + 'static),
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        self.debug(error, f)
    }
}
