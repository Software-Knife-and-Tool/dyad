//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu functions
use crate::{
    classes::{
        char::{Char, Core as _},
        cons::{Cons, ConsIter, Core as _, Properties},
        fixnum::{Core as _, Fixnum},
        float::{Core as _, Float},
        function::{Core as _, Function},
        namespace::{Core as _, Namespace},
        stream::{Core as _, Stream},
        symbol::{Core as _, Symbol},
        vector::{Core as _, Vector},
    },
    core::{
        classes::{Class, Tag},
        exception,
        exception::{Condition, Except},
        frame::Frame,
        mu::{Core as _, Mu},
    },
};

pub trait MuFunction {
    fn mu_compile(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_eval(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_exit(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_funcall(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_if(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_write(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_view(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_tag_of(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_fix(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_fix_env(_: &Mu, _: &mut Frame) -> exception::Result<()>;
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

    fn mu_funcall(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let args = fp.argv[1];

        fp.value = match Tag::class_of(mu, func) {
            Class::Function => match Tag::class_of(mu, args) {
                Class::Null | Class::Cons => match mu.funcall(func, args) {
                    Ok(tag) => tag,
                    Err(e) => return Err(e),
                },
                _ => return Err(Except::raise(mu, Condition::Type, "mu:funcall", args)),
            },
            _ => return Err(Except::raise(mu, Condition::Type, "mu:funcall", func)),
        };

        Ok(())
    }

    fn mu_write(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let form = fp.argv[0];
        let escape = fp.argv[1];
        let stream = fp.argv[2];

        fp.value = form;

        match Tag::class_of(mu, stream) {
            Class::Stream => match mu.write(form, !escape.null_(), stream) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            _ => Err(Except::raise(mu, Condition::Type, "mu:write", stream)),
        }
    }

    fn mu_if(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let test = fp.argv[0];
        let true_fn = fp.argv[1];
        let false_fn = fp.argv[2];

        fp.value = match Tag::class_of(mu, true_fn) {
            Class::Function => match Tag::class_of(mu, false_fn) {
                Class::Function => {
                    match mu.funcall(if test.null_() { false_fn } else { true_fn }, Tag::nil()) {
                        Ok(tag) => tag,
                        Err(e) => return Err(e),
                    }
                }
                _ => return Err(Except::raise(mu, Condition::Type, "mu::if", false_fn)),
            },
            _ => return Err(Except::raise(mu, Condition::Type, "mu::if", true_fn)),
        };

        Ok(())
    }

    fn mu_exit(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let rc = fp.argv[0];

        match Tag::class_of(mu, rc) {
            Class::Fixnum => std::process::exit(Fixnum::as_i64(mu, rc) as i32),
            _ => Err(Except::raise(mu, Condition::Type, "mu:exit", rc)),
        }
    }

    fn mu_view(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        fp.value = match Tag::class_of(mu, tag) {
            Class::Char => Char::view(mu, tag),
            Class::Cons => Cons::view(mu, tag),
            Class::Fixnum => Fixnum::view(mu, tag),
            Class::Float => Float::view(mu, tag),
            Class::Function => Function::view(mu, tag),
            Class::Namespace => Namespace::view(mu, tag),
            Class::Null | Class::Symbol | Class::Keyword => Symbol::view(mu, tag),
            Class::Stream => Stream::view(mu, tag),
            Class::Vector => Vector::view(mu, tag),
            _ => return Err(Except::raise(mu, Condition::Type, "mu:view", tag)),
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

        match Tag::class_of(mu, func) {
            Class::Function => {
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
            _ => Err(Except::raise(mu, Condition::Type, "mu:fix", func)),
        }
    }

    // visit this later
    fn mu_fix_env(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];
        let env = fp.argv[1];

        fp.value = fp.argv[1];

        match Tag::class_of(mu, env) {
            Class::Cons => match Tag::class_of(mu, func) {
                Class::Function => {
                    loop {
                        let mut argv = vec![fp.value];

                        for cons in ConsIter::new(mu, env) {
                            argv.push(Cons::car(mu, cons));
                        }

                        let opt_tag = Frame {
                            func,
                            argv,
                            value: Tag::nil(),
                        }
                        .apply(mu, func);

                        match opt_tag {
                            Ok(value) => {
                                if value.eq_(fp.value) {
                                    break;
                                }

                                fp.value = value;
                            }
                            Err(e) => return Err(e),
                        }
                    }

                    Ok(())
                }
                _ => Err(Except::raise(mu, Condition::Type, "mu:fix", func)),
            },

            _ => Err(Except::raise(mu, Condition::Type, "mu:fix", env)),
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
