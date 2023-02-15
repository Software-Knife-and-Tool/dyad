//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu environment
//!    Extern
//!    Mu
use {
    crate::{
        core::{
            classes::{Tag, Type},
            compile, exception,
            exception::{Condition, Exception},
            frame::Frame,
            namespace::Core as _,
            read::Read,
        },
        image::heap::Heap,
        system::sys as system,
        types::{
            char::{Char, Core as _},
            cons::{Cons, ConsIter, Core as _, Properties as _},
            fixnum::{Core as _, Fixnum},
            float::{Core as _, Float},
            function::{Core as _, Function},
            namespace::{Core as _, Namespace},
            r#struct::{Core as _, Struct},
            stream::{Core as _, Stream},
            symbol::{Core as _, Properties as _, Symbol},
            vector::{Core as _, Vector},
        },
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

    // environments
    pub compile: RefCell<Vec<(Tag, Vec<Tag>)>>,
    pub dynamic: RefCell<Vec<(Tag, Vec<Tag>)>>,
    pub lexical: RefCell<HashMap<u64, RefCell<Vec<Frame>>>>,

    // namespaces
    pub nil_ns: Tag,
    pub mu_ns: Tag,

    // standard streams
    pub stdin: Tag,
    pub stdout: Tag,
    pub errout: Tag,

    // symbol caches
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
}

pub trait Core {
    const VERSION: &'static str = "0.0.10";

    fn new(config: String) -> Self;
    fn apply(&self, _: Tag, _: Tag) -> exception::Result<Tag>;
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
            compile: RefCell::new(Vec::new()),
            config,
            dynamic: RefCell::new(Vec::new()),
            errout: Tag::nil(),
            heap: RefCell::new(Heap::new(1024)),
            lexical: RefCell::new(HashMap::new()),
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

    fn apply(&self, func: Tag, args: Tag) -> exception::Result<Tag> {
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
        match Tag::type_of(self, expr) {
            Type::Cons => {
                let func = Cons::car(self, expr);
                let args = Cons::cdr(self, expr);
                match Tag::type_of(self, func) {
                    Type::Keyword if func.eq_(Symbol::keyword("quote")) => {
                        Ok(Cons::car(self, args))
                    }
                    Type::Symbol => {
                        if Symbol::is_unbound(self, func) {
                            Err(Exception::raise(
                                self,
                                Condition::Unbound,
                                "core::eval",
                                func,
                            ))
                        } else {
                            let fnc = Symbol::value_of(self, func);
                            match Tag::type_of(self, fnc) {
                                Type::Function => self.apply(fnc, args),
                                _ => {
                                    Err(Exception::raise(self, Condition::Type, "core::eval", func))
                                }
                            }
                        }
                    }
                    Type::Function => self.apply(func, args),
                    _ => Err(Exception::raise(self, Condition::Type, "core::eval", func)),
                }
            }
            Type::Symbol => {
                if Symbol::is_unbound(self, expr) {
                    Err(Exception::raise(
                        self,
                        Condition::Unbound,
                        "core:eval",
                        expr,
                    ))
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
        match Tag::type_of(self, tag) {
            Type::Char => Char::write(self, tag, escape, stream),
            Type::Cons => Cons::write(self, tag, escape, stream),
            Type::Fixnum => Fixnum::write(self, tag, escape, stream),
            Type::Float => Float::write(self, tag, escape, stream),
            Type::Function => Function::write(self, tag, escape, stream),
            Type::Namespace => Namespace::write(self, tag, escape, stream),
            Type::Null | Type::Symbol | Type::Keyword => Symbol::write(self, tag, escape, stream),
            Type::Stream => Stream::write(self, tag, escape, stream),
            Type::Vector => Vector::write(self, tag, escape, stream),
            Type::Struct => Struct::write(self, tag, escape, stream),
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
