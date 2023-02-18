//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu functions
use crate::{
    core::{
        classes::{Tag, Type},
        exception,
        exception::{Condition, Exception},
        frame::Frame,
        mu::{Core as _, Mu},
    },
    types::{
        char::{Char, Core as _},
        cons::{Cons, ConsIter, Core as _},
        fixnum::{Core as _, Fixnum},
        float::{Core as _, Float},
        function::{Core as _, Function},
        namespace::{Core as _, Namespace},
        r#struct::{Core as _, Struct},
        stream::{Core as _, Stream},
        symbol::{Core as _, Symbol},
        vector::{Core as _, Vector},
    },
};

pub trait MuFunction {
    fn mu_compile(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_eval(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_exit(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_apply(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_if(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_write(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_view(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_tag_of(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_fix(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Mu {
    fn mu_compile(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match mu.compile(fp.argv[0]) {
            Ok(tag) => tag,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn mu_eval(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match mu.eval(fp.argv[0]) {
            Ok(tag) => tag,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn mu_apply(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let args = fp.argv[1];

        fp.value = match Tag::type_of(mu, func) {
            Type::Function => match Tag::type_of(mu, args) {
                Type::Null | Type::Cons => {
                    let value = Tag::nil();
                    let mut argv = Vec::new();

                    for cons in ConsIter::new(mu, args) {
                        argv.push(Cons::car(mu, cons))
                    }

                    match (Frame { func, argv, value }).apply(mu, func) {
                        Ok(value) => value,
                        Err(e) => return Err(e),
                    }
                }
                _ => return Err(Exception::raise(mu, Condition::Type, "mu:apply", args)),
            },
            _ => return Err(Exception::raise(mu, Condition::Type, "mu:apply", func)),
        };

        Ok(())
    }

    fn mu_write(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let form = fp.argv[0];
        let escape = fp.argv[1];
        let stream = fp.argv[2];

        fp.value = form;

        match Tag::type_of(mu, stream) {
            Type::Stream => match mu.write(form, !escape.null_(), stream) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            _ => Err(Exception::raise(mu, Condition::Type, "mu:write", stream)),
        }
    }

    fn mu_if(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let test = fp.argv[0];
        let true_fn = fp.argv[1];
        let false_fn = fp.argv[2];

        fp.value = match Tag::type_of(mu, true_fn) {
            Type::Function => match Tag::type_of(mu, false_fn) {
                Type::Function => {
                    match mu.apply(if test.null_() { false_fn } else { true_fn }, Tag::nil()) {
                        Ok(tag) => tag,
                        Err(e) => return Err(e),
                    }
                }
                _ => return Err(Exception::raise(mu, Condition::Type, "mu::if", false_fn)),
            },
            _ => return Err(Exception::raise(mu, Condition::Type, "mu::if", true_fn)),
        };

        Ok(())
    }

    fn mu_exit(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let rc = fp.argv[0];

        match Tag::type_of(mu, rc) {
            Type::Fixnum => std::process::exit(Fixnum::as_i64(mu, rc) as i32),
            _ => Err(Exception::raise(mu, Condition::Type, "mu:exit", rc)),
        }
    }

    fn mu_view(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        fp.value = match Tag::type_of(mu, tag) {
            Type::Char => Char::view(mu, tag),
            Type::Cons => Cons::view(mu, tag),
            Type::Fixnum => Fixnum::view(mu, tag),
            Type::Float => Float::view(mu, tag),
            Type::Function => Function::view(mu, tag),
            Type::Namespace => Namespace::view(mu, tag),
            Type::Null | Type::Symbol | Type::Keyword => Symbol::view(mu, tag),
            Type::Stream => Stream::view(mu, tag),
            Type::Struct => Struct::view(mu, tag),
            Type::Vector => Vector::view(mu, tag),
            _ => return Err(Exception::raise(mu, Condition::Type, "mu:view", tag)),
        };

        Ok(())
    }

    fn mu_tag_of(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Fixnum::as_tag(fp.argv[0].as_u64() as i64);

        Ok(())
    }

    fn mu_fix(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];

        fp.value = fp.argv[1];

        match Tag::type_of(mu, func) {
            Type::Function => {
                loop {
                    let value = Tag::nil();
                    let argv = vec![fp.value];
                    let result = Frame { func, argv, value }.apply(mu, func);

                    fp.value = match result {
                        Ok(value) => {
                            if value.eq_(fp.value) {
                                break;
                            }

                            value
                        }
                        Err(e) => return Err(e),
                    };
                }

                Ok(())
            }
            _ => Err(Exception::raise(mu, Condition::Type, "mu:fix", func)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mu_functions() {
        assert_eq!(2 + 2, 4);
    }
}
