//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu environment
//!    Extern
//!    Mu
use {
    crate::{
        classes::{
            char::{Char, Core as _},
            cons::{Cons, ConsIter, Core as _, Properties as _},
            fixnum::{Core as _, Fixnum},
            float::{Core as _, Float},
            function::{Core as _, Function},
            namespace::{Core as _, Namespace},
            stream::{Core as _, Stream},
            symbol::{Core as _, Properties as _, Symbol},
            vector::{Core as _, Vector},
        },
        core::{
            classes::{Class, Tag},
            compile, exception,
            exception::{Condition, Except},
            frame::Frame,
            namespaces::Core as _,
            read::Read,
        },
        image::heap::Heap,
        system::sys as system,
    },
    std::{cell::RefCell, collections::HashMap},
};

// extern
pub type MuCondition = exception::Condition;

// native functions
pub type MuFunctionType = fn(&Mu, &mut Frame) -> exception::Result<()>;

// mu environment
pub struct Mu {
    pub version: Tag,
    pub config: String,
    pub heap: RefCell<Heap>,
    pub system: system::System,

    // standard streams
    pub stdin: Tag,
    pub stdout: Tag,
    pub errout: Tag,

    // namespaces
    pub nil_ns: Tag,
    pub mu_ns: Tag,

    // compiler
    pub lexical_env: RefCell<Vec<(Tag, Vec<Tag>)>>,

    // caches
    pub fnmap: RefCell<Vec<MuFunctionType>>,
    #[allow(clippy::type_complexity)]
    pub ns_caches: RefCell<
        HashMap<
            String,
            (
                Tag,
                (RefCell<HashMap<String, Tag>>, RefCell<HashMap<String, Tag>>),
            ),
        >,
    >,
    pub frames: RefCell<HashMap<u64, RefCell<Vec<Frame>>>>,
}

pub trait Core {
    const VERSION: &'static str = "0.0.4";

    fn new(config: String) -> Self;
    fn funcall(&self, _: Tag, _: Tag) -> exception::Result<Tag>;
    fn eof(&self, _: Tag) -> bool;
    fn eval(&self, _: Tag) -> exception::Result<Tag>;
    fn eq(&self, _: Tag, _: Tag) -> bool;
    fn nil(&self) -> Tag;
    fn compile(&self, _: Tag) -> exception::Result<Tag>;
    fn read(&self, _: Tag, _: bool, _: Tag) -> exception::Result<Tag>;
    fn read_string(&self, _: String) -> exception::Result<Tag>;
    fn write(&self, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn write_string(&self, _: String, _: Tag) -> exception::Result<()>;
}

impl Core for Mu {
    fn new(config: String) -> Self {
        let mut mu = Mu {
            config,
            errout: Tag::nil(),
            fnmap: RefCell::new(Vec::new()),
            frames: RefCell::new(HashMap::new()),
            heap: RefCell::new(Heap::new(1024)),
            lexical_env: RefCell::new(Vec::new()),
            mu_ns: Tag::nil(),
            nil_ns: Tag::nil(),
            ns_caches: RefCell::new(HashMap::new()),
            stdin: Tag::nil(),
            stdout: Tag::nil(),
            system: system::System::new(),
            version: Tag::nil(),
        };

        mu.version = Vector::from_string(<Mu as Core>::VERSION).evict(&mu);

        mu.stdin = match Stream::open_stdin(&mu) {
            Err(_) => panic!("internal: can't open standard-input"),
            Ok(s) => s,
        };

        mu.stdout = match Stream::open_stdout(&mu) {
            Err(_) => panic!("internal: can't open standard-output"),
            Ok(s) => s,
        };

        mu.errout = match Stream::open_errout(&mu) {
            Err(_) => panic!("internal: can't open error-output"),
            Ok(s) => s,
        };

        mu.nil_ns = Namespace::new(&mu, "", Tag::nil()).evict(&mu);
        mu.mu_ns = Namespace::new(&mu, "mu", Tag::nil()).evict(&mu);

        match Namespace::add_ns(&mu, mu.nil_ns) {
            Ok(_) => (),
            Err(_) => panic!("internal: namespaces inconsistency"),
        };

        match Namespace::add_ns(&mu, mu.mu_ns) {
            Ok(_) => (),
            Err(_) => panic!("internal: namespaces inconsistency"),
        };

        Self::install_mu_symbols(&mu);

        mu
    }

    fn nil(&self) -> Tag {
        Tag::nil()
    }

    fn funcall(&self, func: Tag, args: Tag) -> exception::Result<Tag> {
        let value = Tag::nil();
        let mut argv = Vec::new();

        for cons in ConsIter::new(self, args) {
            match self.eval(Cons::car(self, cons)) {
                Ok(arg) => argv.push(arg),
                Err(e) => return Err(e),
            }
        }

        Frame { func, argv, value }.apply(self, func)
    }

    fn eq(&self, tag: Tag, tag1: Tag) -> bool {
        tag.eq_(tag1)
    }

    fn eval(&self, expr: Tag) -> exception::Result<Tag> {
        match Tag::class_of(self, expr) {
            Class::Cons => {
                let func = Cons::car(self, expr);
                let args = Cons::cdr(self, expr);
                match Tag::class_of(self, func) {
                    Class::Keyword if func.eq_(Symbol::keyword("quote")) => {
                        Ok(Cons::car(self, args))
                    }
                    Class::Symbol => {
                        if Symbol::is_unbound(self, func) {
                            Err(Except::raise(self, Condition::Unbound, "core::eval", func))
                        } else {
                            let fnc = Symbol::value_of(self, func);
                            match Tag::class_of(self, fnc) {
                                Class::Function => self.funcall(fnc, args),
                                _ => {
                                    Err(Except::raise(self, Condition::Type, "eval::funcall", func))
                                }
                            }
                        }
                    }
                    Class::Function => self.funcall(func, args),
                    _ => Err(Except::raise(self, Condition::Type, "eval::funcall", func)),
                }
            }
            Class::Symbol => {
                if Symbol::is_unbound(self, expr) {
                    Err(Except::raise(self, Condition::Unbound, "symbol", expr))
                } else {
                    Ok(Symbol::value_of(self, expr))
                }
            }
            _ => Ok(expr),
        }
    }

    fn compile(&self, tag: Tag) -> exception::Result<Tag> {
        compile::compile(self, tag)
    }

    fn eof(&self, stream: Tag) -> bool {
        Stream::is_eof(self, stream)
    }

    fn read(&self, stream: Tag, eofp: bool, eof_value: Tag) -> exception::Result<Tag> {
        <Mu as Read>::read(self, stream, eofp, eof_value, false)
    }

    fn read_string(&self, string: String) -> exception::Result<Tag> {
        match Stream::open_string(self, &string, true) {
            Ok(stream) => <Mu as Read>::read(self, stream, true, Tag::nil(), false),
            Err(e) => Err(e),
        }
    }

    fn write(&self, tag: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        match Tag::class_of(self, tag) {
            Class::Char => Char::write(self, tag, escape, stream),
            Class::Cons => Cons::write(self, tag, escape, stream),
            Class::Fixnum => Fixnum::write(self, tag, escape, stream),
            Class::Float => Float::write(self, tag, escape, stream),
            Class::Function => Function::write(self, tag, escape, stream),
            Class::Namespace => Namespace::write(self, tag, escape, stream),
            Class::Null | Class::Symbol | Class::Keyword => {
                Symbol::write(self, tag, escape, stream)
            }
            Class::Stream => Stream::write(self, tag, escape, stream),
            Class::Vector => Vector::write(self, tag, escape, stream),
            _ => panic!("internal: write type inconsistency"),
        }
    }

    fn write_string(&self, str: String, stream: Tag) -> exception::Result<()> {
        for ch in str.chars() {
            match Stream::write_char(self, stream, ch) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mu() {
        assert_eq!(2 + 2, 4);
    }
}
