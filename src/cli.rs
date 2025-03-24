use crate::bindings::*;
use crate::stub;

use hyperlight_common::resource::BorrowedResourceGuard;

use std::io::Write;

pub struct Environment {
}

impl wasi::cli::Environment for Environment {
    fn get_environment(&mut self) -> Vec<(String, String)> {
        Vec::new()
    }
}

pub struct Streams {
}
impl wasi::io::Streams<stub::Void> for Streams {
}

pub enum InputStream {
    Stdin
}
impl wasi::io::streams::InputStream for Streams  {
    type T = InputStream;
}

pub enum OutputStream {
    Stdout,
    Stderr
}
impl wasi::io::streams::OutputStream<stub::Void> for Streams {
    type T = OutputStream;

    fn r#check_write(&mut self, _self: BorrowedResourceGuard<Self::T>) -> std::result::Result<u64,wasi::io::streams::StreamError<stub::Void> >  {
        Ok(1024)
    }

    fn r#write(&mut self, self_: BorrowedResourceGuard<Self::T>, r#contents:std::vec::Vec<u8>) -> std::result::Result<(),wasi::io::streams::StreamError<stub::Void> >  {
        self.blocking_write_and_flush(self_, contents)
    }

    fn r#blocking_write_and_flush(&mut self, self_: BorrowedResourceGuard<Self::T> ,r#contents:std::vec::Vec<u8>) -> std::result::Result<(),wasi::io::streams::StreamError<stub::Void> >  {
        match *self_ {
            OutputStream::Stdout => {
                let _ = std::io::stdout().write_all(&contents);
            }
            OutputStream::Stderr => {
                let _ = std::io::stderr().write_all(&contents);
            }
        };
        println!("blocking write and flush is about to return");
        Ok(())
    }

    fn r#blocking_flush(&mut self, _self: BorrowedResourceGuard<Self::T>) -> std::result::Result<(),wasi::io::streams::StreamError<stub::Void> >  {
        Ok(())
    }
}

impl wasi::cli::Stdin<InputStream> for Streams {
    fn get_stdin(&mut self) -> InputStream { InputStream::Stdin }
}

impl wasi::cli::Stdout<OutputStream> for Streams {
    fn get_stdout(&mut self) -> OutputStream { OutputStream::Stdout }
}

impl wasi::cli::Stderr<OutputStream> for Streams {
    fn get_stderr(&mut self) -> OutputStream { OutputStream::Stderr }
}

pub struct Preopens {}

impl wasi::filesystem::Preopens<stub::Void> for Preopens {
    fn get_directories(&mut self) -> Vec<(stub::Void, String)> {
        Vec::new()
    }
}
