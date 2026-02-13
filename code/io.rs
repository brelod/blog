/* ANCHOR: stdio */
use crate::error::{Error, Result};
use core::fmt;

pub trait Write {
    fn flush() -> Result<()>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(n) => {
                    buf = &buf[n..];
                }
                Err(Error::EINTR) => { /* retry */ }
                Err(other) => return Err(other),
            }
        }
        Ok(())
    }
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn read_all(&mut self, mut buf: &mut [u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(n) => {
                    buf = &mut buf[n..];
                }
                Err(Error::EINTR) => { /* retry */ }
                Err(other) => return Err(other),
            }
        }
        Ok(())
    }
}

pub struct File {
    fd: u32,
}

impl File {
    pub fn new(fd: u32) -> Self {
        Self { fd }
    }
}

impl Write for File {
    fn flush() -> Result<()> {
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        crate::syscall::write(self.fd, buf)
    }
}

impl fmt::Write for File {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.write(s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        crate::syscall::read(self.fd, buf)
    }
}

pub fn stdin() -> File {
    File::new(0)
}

pub fn stdout() -> File {
    File::new(1)
}

pub fn stderr() -> File {
    File::new(2)
}
/* ANCHOR_END: stdio */

/* ANCHOR: print-macro */
#[macro_export]
macro_rules! print {
    ($fmt:literal $(,$($args:expr)*)?) => {{
        use core::fmt::Write;
        write!($crate::io::stdout(), $fmt, $($($args),*)?).unwrap();
    }};
    ($fmt:literal, $($args:expr),*) => {{
        use core::fmt::Write;
        write!($crate::io::stdout(), $fmt, $($args),*).unwrap();
    }}
}

#[macro_export]
macro_rules! println {
    ($fmt:literal $(,$($args:expr)*)?) => {{
        $crate::print!("{}\n", format_args!($fmt, $($($args),*)?))
    }};
    ($fmt:literal, $($args:expr),*) => {{
        $crate::print!("{}\n", format_args!($fmt, $($args),*))
    }}
}

#[macro_export]
macro_rules! eprint {
    ($fmt:literal $(,$($args:expr)*)?) => {{
        use core::fmt::Write;
        write!($crate::io::stderr(), $fmt, $($($args),*)?).unwrap();
    }}
}

#[macro_export]
macro_rules! eprintln {
    ($fmt:literal $(,$($args:expr)*)?) => {{
        $crate::eprint!("{}\n", format_args!($fmt, $($($args),*)?))
    }}
}
/* ANCHOR_END: print-macro */
