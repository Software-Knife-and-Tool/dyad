//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! system streams
use {
    crate::core::{
        classes::Tag,
        exception,
        exception::{Condition, Exception},
    },
    std::{
        cell::{Ref, RefCell, RefMut},
        fs,
        io::{Read, Write},
    },
};

// we cannot possibly ever have this many streams open
pub const STDIN: usize = 0x80000000;
pub const STDOUT: usize = 0x80000001;
pub const STDERR: usize = 0x80000002;

pub struct Stream {
    filetab: RefCell<Vec<RefCell<fs::File>>>,
}

impl Default for Stream {
    fn default() -> Self {
        Self::new()
    }
}

impl Stream {
    pub fn new() -> Self {
        Stream {
            filetab: RefCell::new(Vec::new()),
        }
    }
}

pub trait Core {
    fn close(_: &Stream, _: usize);
    fn flush(_: &Stream, _: usize);
    fn open(_: &Stream, _: &str, is_input: bool) -> exception::Result<usize>;
    fn read_byte(_: &Stream, _: usize) -> exception::Result<Option<u8>>;
    fn write_byte(_: &Stream, _: usize, _: u8) -> exception::Result<Option<()>>;
}

impl Core for Stream {
    fn flush(stream: &Stream, index: usize) {
        let tab_ref: Ref<Vec<RefCell<fs::File>>> = stream.filetab.borrow();

        match index {
            STDOUT => {
                std::io::stdout().flush().unwrap();
            }
            STDERR => {
                std::io::stderr().flush().unwrap();
            }
            _ => {
                if index >= tab_ref.len() {
                    panic!();
                }
            }
        }
    }

    fn close(stream: &Stream, index: usize) {
        let tab_ref: Ref<Vec<RefCell<fs::File>>> = stream.filetab.borrow();

        match index {
            STDIN | STDOUT | STDERR => (),
            _ => {
                if index >= tab_ref.len() {
                    panic!();
                } else {
                    let file: Ref<fs::File> = tab_ref[index].borrow();
                    std::mem::drop(file);
                }
            }
        }
    }

    fn open(stream: &Stream, path: &str, is_input: bool) -> exception::Result<usize> {
        let file = if is_input {
            match fs::File::open(path) {
                Ok(file) => file,
                Err(_) => {
                    return Err(Exception {
                        condition: Condition::Open,
                        source: "system::open".to_string(),
                        tag: Tag::nil(),
                    })
                }
            }
        } else {
            match fs::File::create(path) {
                Ok(file) => file,
                Err(_) => {
                    return Err(Exception {
                        condition: Condition::Open,
                        source: "system::open".to_string(),
                        tag: Tag::nil(),
                    })
                }
            }
        };

        let desc = RefCell::new(file);
        let mut tab_ref: RefMut<Vec<RefCell<fs::File>>> = stream.filetab.borrow_mut();
        let index = tab_ref.len();

        tab_ref.push(desc);
        Ok(index)
    }

    fn read_byte(stream: &Stream, stream_id: usize) -> exception::Result<Option<u8>> {
        let tab_ref: Ref<Vec<RefCell<fs::File>>> = stream.filetab.borrow();
        let mut buf = [0; 1];

        match stream_id {
            STDIN => match std::io::stdin().read(&mut buf) {
                Ok(nread) => {
                    if nread == 0 {
                        Ok(None)
                    } else {
                        Ok(Some(buf[0]))
                    }
                }
                Err(_) => Err(Exception {
                    condition: Condition::Read,
                    source: "system::read_byte".to_string(),
                    tag: Tag::nil(),
                }),
            },
            _ if stream_id < tab_ref.len() => {
                let mut file_ref: RefMut<fs::File> = tab_ref[stream_id].borrow_mut();
                match file_ref.read(&mut buf) {
                    Ok(nread) => {
                        if nread == 0 {
                            Ok(None)
                        } else {
                            Ok(Some(buf[0]))
                        }
                    }
                    Err(_) => Err(Exception {
                        condition: Condition::Read,
                        source: "system::read_byte".to_string(),
                        tag: Tag::nil(),
                    }),
                }
            }
            _ => panic!(),
        }
    }

    fn write_byte(stream: &Stream, stream_id: usize, byte: u8) -> exception::Result<Option<()>> {
        let tab_ref: Ref<Vec<RefCell<fs::File>>> = stream.filetab.borrow();
        let buf = [byte; 1];

        match stream_id {
            STDOUT => match std::io::stdout().write(&buf) {
                Ok(_) => Ok(None),
                Err(_) => Err(Exception {
                    condition: Condition::Write,
                    source: "system::write_byte".to_string(),
                    tag: Tag::nil(),
                }),
            },
            STDERR => match std::io::stderr().write(&buf) {
                Ok(_) => Ok(None),
                Err(_) => Err(Exception {
                    condition: Condition::Write,
                    source: "system::write_byte".to_string(),
                    tag: Tag::nil(),
                }),
            },
            _ if stream_id < tab_ref.len() => {
                let mut file_ref: RefMut<fs::File> = tab_ref[stream_id].borrow_mut();
                match file_ref.write(&buf) {
                    Ok(_) => Ok(None),
                    Err(_) => Err(Exception {
                        condition: Condition::Write,
                        source: "system::write_byte".to_string(),
                        tag: Tag::nil(),
                    }),
                }
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::system::stream::Stream;

    #[test]
    fn stream() {
        match Stream::new() {
            _ => assert_eq!(true, true),
        }
    }
}
