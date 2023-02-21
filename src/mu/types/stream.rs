//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu stream type
#![allow(unused_braces)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::identity_op)]

use {
    crate::{
        core::{
            classes::{Tag, TagType, TagU64, Type},
            exception,
            exception::{Condition, Exception},
            frame::Frame,
            mu::{Core as _, Mu},
        },
        image,
        system::stream::{Core as _, Stream as SystemStream},
        system::stream::{STDERR, STDIN, STDOUT},
        types::{
            char::Char,
            cons::{Cons, ConsIter, Core as _},
            fixnum::Fixnum,
            r#struct::Struct,
            symbol::{Core as _, Symbol},
            vector::{Core as _, Properties as _, Vector},
        },
    },
    std::cell::{Ref, RefMut},
};

pub enum Stream {
    File(String, bool, u8),
    String(String, bool, u8),
    Stdin(u8),
    Stdout(),
    Stderr(),
    Indirect(Image),
}

// stream image
pub struct Image {
    source: Tag,    // system file id (fixnum) | nil
    count: Tag,     // char count (fixnum)
    direction: Tag, // :input | :output (keyword)
    eof: Tag,       // end of file flag (bool)
    unch: Tag,      // pushbask for input streams (() | character)
}

impl Stream {
    pub fn evict(&self, mu: &Mu) -> Tag {
        match self {
            Stream::Indirect(image) => {
                let slices: &[[u8; 8]] = &[
                    image.source.as_slice(),
                    image.count.as_slice(),
                    image.direction.as_slice(),
                    image.eof.as_slice(),
                    image.unch.as_slice(),
                ];

                let mut heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
                Tag::Indirect(
                    TagU64::new()
                        .with_offset(heap_ref.alloc(slices, Type::Stream as u8) as u64)
                        .with_tag(TagType::Heap),
                )
            }
            _ => panic!("internal: stream type inconsistency"),
        }
    }

    pub fn to_image(mu: &Mu, tag: Tag) -> Image {
        match Tag::type_of(mu, tag) {
            Type::Stream => match tag {
                Tag::Indirect(main) => {
                    let heap_ref: Ref<image::heap::Heap> = mu.heap.borrow();

                    let image = Image {
                        source: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize, 8).unwrap(),
                        ),
                        count: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 8, 8).unwrap(),
                        ),
                        direction: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 16, 8).unwrap(),
                        ),
                        eof: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 24, 8).unwrap(),
                        ),
                        unch: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 32, 8).unwrap(),
                        ),
                    };

                    image
                }
                _ => panic!("internal: stream type inconsistency"),
            },
            _ => panic!("internal: stream type inconsistency"),
        }
    }

    pub fn update(mu: &Mu, image: &Image, stream: Tag) {
        let slices: &[[u8; 8]] = &[
            image.source.as_slice(),
            image.count.as_slice(),
            image.direction.as_slice(),
            image.eof.as_slice(),
            image.unch.as_slice(),
        ];

        let offset = match stream {
            Tag::Indirect(heap) => heap.offset(),
            _ => panic!("internal: tag format inconsistency"),
        } as usize;

        let mut heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
        heap_ref.write_image(slices, offset);
    }
}

pub trait Core {
    fn close(_: &Mu, _: Tag);
    fn is_eof(_: &Mu, _: Tag) -> bool;
    fn is_open(_: &Mu, _: Tag) -> bool;
    fn get_string(_: &Mu, _: Tag) -> exception::Result<Tag>;
    fn open_errout(_: &Mu) -> exception::Result<Tag>;
    fn open_file(_: &Mu, _: &str, is_input: bool) -> exception::Result<Tag>;
    fn open_stdin(_: &Mu) -> exception::Result<Tag>;
    fn open_stdout(_: &Mu) -> exception::Result<Tag>;
    fn open_string(_: &Mu, _: &str, is_input: bool) -> exception::Result<Tag>;
    fn read_byte(_: &Mu, _: Tag) -> exception::Result<Option<u8>>;
    fn read_char(_: &Mu, _: Tag) -> exception::Result<Option<char>>;
    fn unread_char(_: &Mu, _: Tag, _: char) -> exception::Result<Option<()>>;
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn write_byte(_: &Mu, _: Tag, _: u8) -> exception::Result<Option<()>>;
    fn write_char(_: &Mu, _: Tag, _: char) -> exception::Result<Option<()>>;
    fn view(_: &Mu, _: Tag) -> Tag;
}

impl Core for Stream {
    fn view(mu: &Mu, stream: Tag) -> Tag {
        let image = Self::to_image(mu, stream);

        Struct::to_tag(
            mu,
            Symbol::keyword("stream"),
            vec![
                image.source,
                image.count,
                image.direction,
                image.eof,
                image.unch,
            ],
        )
    }

    fn is_eof(mu: &Mu, stream: Tag) -> bool {
        let image = Self::to_image(mu, stream);

        match Tag::type_of(mu, image.direction) {
            Type::Keyword if image.direction.eq_(Symbol::keyword("input")) => {
                if !image.unch.null_() {
                    false
                } else {
                    !image.eof.null_()
                }
            }
            _ => !image.eof.null_(),
        }
    }

    fn is_open(mu: &Mu, stream: Tag) -> bool {
        let image = Self::to_image(mu, stream);

        !image.source.eq_(Tag::t())
    }

    fn close(mu: &Mu, stream: Tag) {
        let mut image = Self::to_image(mu, stream);

        SystemStream::close(
            &mu.system.streams,
            Fixnum::as_i64(mu, image.source) as usize,
        );

        image.source = Tag::t();
        Self::update(mu, &image, stream);
    }

    fn get_string(mu: &Mu, tag: Tag) -> exception::Result<Tag> {
        if !Self::is_open(mu, tag) {
            return Err(Exception::raise(
                mu,
                Condition::Open,
                "stream::get_string",
                tag,
            ));
        }

        match Tag::type_of(mu, tag) {
            Type::Stream => {
                let mut image = Self::to_image(mu, tag);
                let source = image.source;

                match Tag::type_of(mu, image.direction) {
                    Type::Keyword if image.direction.eq_(Symbol::keyword("output")) => {
                        image.source = Tag::nil();
                        Self::update(mu, &image, tag);

                        match Tag::type_of(mu, source) {
                            Type::Null => Ok(Vector::from_string("").evict(mu)),
                            Type::Cons => {
                                let string = ConsIter::new(mu, source).fold(
                                    String::from(""),
                                    |mut acc, cons| {
                                        acc.push(Char::as_char(mu, Cons::car(mu, cons)));
                                        acc
                                    },
                                );

                                Ok(
                                    Vector::from_string(&string.chars().rev().collect::<String>())
                                        .evict(mu),
                                )
                            }
                            _ => Err(Exception::raise(
                                mu,
                                Condition::Type,
                                "stream::get_string",
                                tag,
                            )),
                        }
                    }
                    _ => panic!("internal: tag format inconsistency"),
                }
            }
            _ => panic!("internal: stream type required"),
        }
    }

    fn write(mu: &Mu, tag: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        match Tag::type_of(mu, tag) {
            Type::Stream => {
                let image = Self::to_image(mu, tag);
                match Tag::type_of(mu, image.source) {
                    Type::Keyword => mu.write_string("#<stream: closed>".to_string(), stream),
                    Type::Fixnum => mu.write_string(
                        format!("#<stream: id: {}>", Fixnum::as_i64(mu, image.source)),
                        stream,
                    ),
                    Type::Null | Type::Cons | Type::Vector => {
                        mu.write_string("#<stream: string>".to_string(), stream)
                    }
                    _ => panic!(
                        "internal: stream type inconsistency {:?}",
                        Tag::type_of(mu, image.source)
                    ),
                }
            }
            _ => panic!("internal: stream type inconsistency"),
        }
    }

    fn open_file(mu: &Mu, path: &str, is_input: bool) -> exception::Result<Tag> {
        let stream = Stream::File(path.to_string(), is_input, 0);
        let id = SystemStream::open(&mu.system.streams, path, is_input).unwrap();

        let image = Image {
            source: Fixnum::as_tag(id as i64),
            count: Fixnum::as_tag(0),
            direction: if is_input {
                Symbol::keyword("input")
            } else {
                Symbol::keyword("output")
            },
            eof: Tag::nil(),
            unch: Tag::nil(),
        };

        Ok(Stream::Indirect(image).evict(mu))
    }

    fn open_string(mu: &Mu, str: &str, is_input: bool) -> exception::Result<Tag> {
        let string = Stream::String(str.to_string(), is_input, 0);

        let image = Image {
            source: if is_input {
                Vector::from_string(str).evict(mu)
            } else {
                let mut cons = Tag::nil();

                for ch in str.chars() {
                    cons = Cons::new(Char::as_tag(ch), cons).evict(mu);
                }

                cons
            },
            count: Fixnum::as_tag(0),
            direction: if is_input {
                Symbol::keyword("input")
            } else {
                Symbol::keyword("output")
            },
            eof: Tag::nil(),
            unch: Tag::nil(),
        };

        Ok(Stream::Indirect(image).evict(mu))
    }

    fn open_stdin(mu: &Mu) -> exception::Result<Tag> {
        let image = Image {
            source: Fixnum::as_tag(STDIN as i64),
            count: Fixnum::as_tag(0),
            direction: Symbol::keyword("input"),
            eof: Tag::nil(),
            unch: Tag::nil(),
        };

        Ok(Stream::Indirect(image).evict(mu))
    }

    fn open_stdout(mu: &Mu) -> exception::Result<Tag> {
        let image = Image {
            source: Fixnum::as_tag(STDOUT as i64),
            count: Fixnum::as_tag(0),
            direction: Symbol::keyword("output"),
            eof: Tag::nil(),
            unch: Tag::nil(),
        };

        Ok(Stream::Indirect(image).evict(mu))
    }

    fn open_errout(mu: &Mu) -> exception::Result<Tag> {
        let image = Image {
            source: Fixnum::as_tag(STDERR as i64),
            count: Fixnum::as_tag(0),
            direction: Symbol::keyword("output"),
            eof: Tag::nil(),
            unch: Tag::nil(),
        };

        Ok(Stream::Indirect(image).evict(mu))
    }

    fn read_char(mu: &Mu, stream: Tag) -> exception::Result<Option<char>> {
        match Self::read_byte(mu, stream) {
            Ok(Some(byte)) => Ok(Some(byte as char)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn read_byte(mu: &Mu, stream: Tag) -> exception::Result<Option<u8>> {
        let system_stream = &mu.system.streams;
        let mut image = Self::to_image(mu, stream);
        let unch = image.unch;

        if !Self::is_open(mu, stream) {
            return Err(Exception::raise(
                mu,
                Condition::Open,
                "stream::read_byte",
                stream,
            ));
        }

        if image.direction.eq_(Symbol::keyword("output")) {
            return Err(Exception::raise(
                mu,
                Condition::Stream,
                "stream::read_byte",
                stream,
            ));
        }

        if Self::is_eof(mu, stream) {
            return Ok(None);
        }

        match Tag::type_of(mu, image.source) {
            Type::Fixnum => {
                let stream_id = Fixnum::as_i64(mu, image.source) as usize;

                if unch.null_() {
                    match SystemStream::read_byte(system_stream, stream_id) {
                        Ok(opt) => match opt {
                            Some(byte) => Ok(Some(byte)),
                            None => {
                                image.eof = Tag::t();
                                Self::update(mu, &image, stream);
                                Ok(None)
                            }
                        },
                        Err(e) => {
                            e.print(mu);
                            Err(e)
                        }
                    }
                } else {
                    image.unch = Tag::nil();
                    Self::update(mu, &image, stream);

                    Ok(Some(Char::as_char(mu, unch) as u8))
                }
            }
            Type::Vector => {
                if unch.null_() {
                    let mut index = Fixnum::as_i64(mu, image.count) as usize;
                    let length = Vector::length_of(mu, image.source);
                    let ch = Vector::r#ref(mu, image.source, index);

                    index += 1;
                    if index == length {
                        image.eof = Tag::t();
                    }

                    image.count = Fixnum::as_tag(index as i64);
                    Self::update(mu, &image, stream);

                    match ch {
                        Some(ch) => Ok(Some(Char::as_char(mu, ch) as u8)),
                        None => panic!("internal: string stream inconsistency"),
                    }
                } else {
                    image.unch = Tag::nil();
                    Self::update(mu, &image, stream);

                    Ok(Some(Char::as_char(mu, unch) as u8))
                }
            }
            _ => panic!("internal: stream type inconsistency"),
        }
    }

    fn unread_char(mu: &Mu, stream: Tag, ch: char) -> exception::Result<Option<()>> {
        let mut image = Self::to_image(mu, stream);

        if !Self::is_open(mu, stream) {
            return Err(Exception::raise(
                mu,
                Condition::Open,
                "stream::unread_char",
                stream,
            ));
        }

        if image.direction.eq_(Symbol::keyword("output")) {
            return Err(Exception::raise(
                mu,
                Condition::Stream,
                "stream::unread_char",
                stream,
            ));
        }

        if image.unch.null_() {
            image.unch = Char::as_tag(ch);
            Self::update(mu, &image, stream);

            Ok(None)
        } else {
            Err(Exception::raise(
                mu,
                Condition::Error,
                "stream::unread_char",
                Char::as_tag(ch),
            ))
        }
    }

    fn write_char(mu: &Mu, stream: Tag, ch: char) -> exception::Result<Option<()>> {
        let system_stream = &mu.system.streams;
        let mut image = Self::to_image(mu, stream);

        if !Self::is_open(mu, stream) {
            return Err(Exception::raise(
                mu,
                Condition::Open,
                "stream::write_char",
                stream,
            ));
        }

        if image.direction.eq_(Symbol::keyword("input")) {
            return Err(Exception::raise(
                mu,
                Condition::Stream,
                "system::write_char",
                stream,
            ));
        }

        match Tag::type_of(mu, image.source) {
            Type::Fixnum => {
                let stream_id = Fixnum::as_i64(mu, image.source) as usize;
                SystemStream::write_byte(system_stream, stream_id, ch as u8)
            }
            Type::Null | Type::Cons => {
                image.source = Cons::new(Char::as_tag(ch), image.source).evict(mu);
                image.count = Fixnum::as_tag(Fixnum::as_i64(mu, image.count) + 1);
                Self::update(mu, &image, stream);

                Ok(None)
            }
            _ => panic!("internal: stream state inconsistency"),
        }
    }

    fn write_byte(mu: &Mu, stream: Tag, byte: u8) -> exception::Result<Option<()>> {
        let system_stream = &mu.system.streams;
        let mut image = Self::to_image(mu, stream);

        if !Self::is_open(mu, stream) {
            return Err(Exception::raise(
                mu,
                Condition::Open,
                "stream::write_byte",
                stream,
            ));
        }

        if image.direction.eq_(Symbol::keyword("input")) {
            return Err(Exception::raise(
                mu,
                Condition::Stream,
                "system::write_byte",
                stream,
            ));
        }

        match Tag::type_of(mu, image.source) {
            Type::Fixnum => {
                let stream_id = Fixnum::as_i64(mu, image.source) as usize;
                SystemStream::write_byte(system_stream, stream_id, byte)
            }
            Type::Null | Type::Cons => {
                image.source = Cons::new(Fixnum::as_tag(byte as i64), image.source).evict(mu);
                image.count = Fixnum::as_tag(Fixnum::as_i64(mu, image.count) + 1);
                Self::update(mu, &image, stream);

                Ok(None)
            }
            _ => panic!(
                "internal: {:?} stream state inconsistency",
                Tag::type_of(mu, image.source)
            ),
        }
    }
}

pub trait MuFunction {
    fn mu_close(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_eof(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_get_string(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_open(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_openp(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_read(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_read_byte(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_read_char(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_unread_byte(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_unread_char(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_write(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_write_byte(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_write_char(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Stream {
    fn mu_close(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];

        fp.value = match Tag::type_of(mu, stream) {
            Type::Stream => {
                if Self::is_open(mu, stream) {
                    Self::close(mu, stream);
                    Tag::t()
                } else {
                    Tag::nil()
                }
            }
            _ => return Err(Exception::raise(mu, Condition::Type, "mu:close", stream)),
        };

        Ok(())
    }

    fn mu_openp(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];

        fp.value = match Tag::type_of(mu, stream) {
            Type::Stream => {
                if Self::is_open(mu, stream) {
                    stream
                } else {
                    Tag::nil()
                }
            }
            _ => return Err(Exception::raise(mu, Condition::Type, "mu:openp", stream)),
        };

        Ok(())
    }

    fn mu_open(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let st_type = fp.argv[0];
        let st_dir = fp.argv[1];
        let st_arg = fp.argv[2];

        let arg = match Tag::type_of(mu, st_arg) {
            Type::Vector => Vector::as_string(mu, st_arg),
            _ => return Err(Exception::raise(mu, Condition::Type, "mu:open", st_arg)),
        };

        let dir = match Tag::type_of(mu, st_dir) {
            Type::Keyword if st_dir.eq_(Symbol::keyword("input")) => true,
            Type::Keyword if st_dir.eq_(Symbol::keyword("output")) => false,
            _ => return Err(Exception::raise(mu, Condition::Type, "mu:open", st_dir)),
        };

        match Tag::type_of(mu, st_type) {
            Type::Keyword if st_type.eq_(Symbol::keyword("file")) => {
                fp.value = Self::open_file(mu, &arg, dir).unwrap();
                Ok(())
            }
            Type::Keyword if st_type.eq_(Symbol::keyword("string")) => {
                fp.value = Self::open_string(mu, &arg, dir).unwrap();
                Ok(())
            }
            _ => Err(Exception::raise(mu, Condition::Type, "mu:open", st_type)),
        }
    }

    fn mu_read(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let eofp = fp.argv[1];
        let eof_value = fp.argv[2];

        match Tag::type_of(mu, stream) {
            Type::Stream => match mu.read(stream, !eofp.null_(), eof_value) {
                Ok(tag) => {
                    fp.value = tag;
                    Ok(())
                }
                Err(e) => Err(e),
            },
            _ => Err(Exception::raise(mu, Condition::Type, "mu:read", stream)),
        }
    }

    fn mu_write(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let value = fp.argv[0];
        let escape = fp.argv[1];
        let stream = fp.argv[2];

        match Tag::type_of(mu, stream) {
            Type::Stream => match mu.write(value, !escape.null_(), stream) {
                Ok(_) => {
                    fp.value = value;
                    Ok(())
                }
                Err(e) => Err(e),
            },
            _ => Err(Exception::raise(mu, Condition::Type, "mu:write", stream)),
        }
    }

    fn mu_eof(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];

        match Tag::type_of(mu, stream) {
            Type::Stream => {
                fp.value = if Self::is_eof(mu, stream) {
                    Tag::t()
                } else {
                    Tag::nil()
                };
                Ok(())
            }
            _ => Err(Exception::raise(mu, Condition::Type, "mu:eof", stream)),
        }
    }

    fn mu_get_string(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];

        match Tag::type_of(mu, stream) {
            Type::Stream => match Self::get_string(mu, stream) {
                Ok(tag) => {
                    fp.value = tag;
                    Ok(())
                }
                Err(e) => Err(e),
            },
            _ => Err(Exception::raise(mu, Condition::Type, "mu:get-str", stream)),
        }
    }

    fn mu_read_char(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let eofp = fp.argv[1];
        let eof_value = fp.argv[2];

        match Tag::type_of(mu, stream) {
            Type::Stream => match Self::read_char(mu, stream) {
                Ok(Some(ch)) => {
                    fp.value = Char::as_tag(ch);
                    Ok(())
                }
                Ok(None) if !eofp.null_() => {
                    fp.value = eof_value;
                    Ok(())
                }
                Ok(None) => Err(Exception::raise(mu, Condition::Eof, "mu:read-char", stream)),
                Err(e) => Err(e),
            },
            _ => Err(Exception::raise(
                mu,
                Condition::Type,
                "mu:read-char",
                stream,
            )),
        }
    }

    fn mu_read_byte(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let eofp = fp.argv[1];
        let eof_value = fp.argv[2];

        match Tag::type_of(mu, stream) {
            Type::Stream => match Self::read_byte(mu, stream) {
                Ok(Some(byte)) => {
                    fp.value = Fixnum::as_tag(byte as i64);
                    Ok(())
                }
                Ok(None) if !eofp.null_() => {
                    fp.value = eof_value;
                    Ok(())
                }
                Ok(None) => Err(Exception::raise(mu, Condition::Eof, "mu:read-byte", stream)),
                Err(e) => Err(e),
            },
            _ => Err(Exception::raise(
                mu,
                Condition::Type,
                "mu:read-byte",
                stream,
            )),
        }
    }

    fn mu_unread_byte(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let byte = fp.argv[1];

        match Tag::type_of(mu, stream) {
            Type::Stream => {
                match Self::unread_char(mu, stream, (Fixnum::as_i64(mu, byte) as u8) as char) {
                    Ok(Some(_)) => {
                        fp.value = byte;
                        Ok(())
                    }
                    Ok(None) => Err(Exception::raise(
                        mu,
                        Condition::Eof,
                        "mu:unread-byte",
                        stream,
                    )),
                    Err(e) => Err(e),
                }
            }
            _ => Err(Exception::raise(
                mu,
                Condition::Type,
                "mu:unread-byte",
                stream,
            )),
        }
    }

    fn mu_unread_char(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let ch = fp.argv[1];

        match Tag::type_of(mu, stream) {
            Type::Stream => match Self::unread_char(mu, stream, Char::as_char(mu, ch)) {
                Ok(Some(_)) => {
                    fp.value = ch;
                    Ok(())
                }
                Ok(None) => Err(Exception::raise(
                    mu,
                    Condition::Eof,
                    "mu:unread-char",
                    stream,
                )),
                Err(e) => Err(e),
            },
            _ => Err(Exception::raise(
                mu,
                Condition::Type,
                "mu:unread-char",
                stream,
            )),
        }
    }

    fn mu_write_char(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let ch = fp.argv[0];
        let stream = fp.argv[1];

        match Tag::type_of(mu, ch) {
            Type::Char => match Tag::type_of(mu, stream) {
                Type::Stream => match Self::write_char(mu, stream, Char::as_char(mu, ch)) {
                    Ok(_) => {
                        fp.value = ch;
                        Ok(())
                    }
                    Err(e) => Err(e),
                },
                _ => Err(Exception::raise(
                    mu,
                    Condition::Type,
                    "mu:write-char",
                    stream,
                )),
            },
            _ => Err(Exception::raise(
                mu,
                Condition::Type,
                "mu:write-char",
                stream,
            )),
        }
    }

    fn mu_write_byte(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let byte = fp.argv[0];
        let stream = fp.argv[1];

        match Tag::type_of(mu, byte) {
            Type::Fixnum if Fixnum::as_i64(mu, byte) < 256 => match Tag::type_of(mu, stream) {
                Type::Stream => {
                    match Self::write_byte(mu, stream, Fixnum::as_i64(mu, byte) as u8) {
                        Ok(_) => {
                            fp.value = byte;
                            Ok(())
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => Err(Exception::raise(
                    mu,
                    Condition::Type,
                    "mu:write-char",
                    stream,
                )),
            },
            _ => Err(Exception::raise(
                mu,
                Condition::Type,
                "mu:write-byte",
                stream,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
