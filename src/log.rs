use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub fn elog(message: String) {
    let buffer_writer = BufferWriter::stderr(ColorChoice::Auto);
    let mut buffer = buffer_writer.buffer();

    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();
    write!(buffer, "error:").unwrap();
    buffer.reset().unwrap();
    write!(buffer, " {}\n", message).unwrap();

    buffer_writer.print(&buffer).unwrap();
}

#[macro_export]
macro_rules! elog {
    ($($arg:tt)*) => {
        $crate::log::elog(format!($($arg)*))
    }
}

pub fn log(message: String) {
    println!("{}", message)
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::log::log(format!($($arg)*))
    }
}
