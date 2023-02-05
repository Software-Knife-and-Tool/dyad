//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu exceptions:
//!    Condition
//!    Exception
//!    Result<Exception>
//!    print
//!    raise
use {
    crate::{
        core::{
            classes::{Tag, Type},
            frame::Frame,
            mu::{Core as _, Mu},
        },
        types::{
            cons::{Cons, Core as _},
            symbol::{Core as _, Symbol},
        },
    },
    std::fmt,
};

pub type Result<T> = std::result::Result<T, Exception>;

#[derive(Clone)]
pub struct Exception {
    pub condition: Condition,
    pub tag: Tag,
    pub source: String,
}

#[derive(Clone, Debug)]
pub enum Condition {
    Arity,
    Eof,
    Error,
    Open,
    Range,
    Read,
    Stream,
    Syntax,
    Type,
    Unbound,
    Write,
    ZeroDivide,
}

lazy_static! {
    static ref CONDMAP: Vec<(Tag, Condition)> = vec![
        (Symbol::keyword("arity"), Condition::Arity),
        (Symbol::keyword("div0"), Condition::ZeroDivide),
        (Symbol::keyword("eof"), Condition::Eof),
        (Symbol::keyword("error"), Condition::Error),
        (Symbol::keyword("open"), Condition::Open),
        (Symbol::keyword("range"), Condition::Range),
        (Symbol::keyword("read"), Condition::Read),
        (Symbol::keyword("stream"), Condition::Stream),
        (Symbol::keyword("syntax"), Condition::Syntax),
        (Symbol::keyword("type"), Condition::Type),
        (Symbol::keyword("unbound"), Condition::Unbound),
        (Symbol::keyword("write"), Condition::Write),
    ];
}

impl fmt::Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{}", self.condition, self.source)
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{}", self.condition, self.source)
    }
}

pub trait Except {
    fn print(&self, _: &Mu);
    fn raise(_: &Mu, _: Condition, _: &str, _: Tag) -> Self;
}

impl Except for Exception {
    fn print(&self, mu: &Mu) {
        eprintln!();
        eprint!(
            "exception: raised from {1}, {:?} condition on ",
            self.condition, self.source
        );

        match mu.write(self.tag, true, mu.errout) {
            Ok(_) => eprintln!(),
            Err(_) => panic!("internal: can't write error-out"),
        }
    }

    fn raise(mu: &Mu, condition: Condition, src: &str, tag: Tag) -> Self {
        let except = Exception {
            condition,
            source: src.to_string(),
            tag,
        };

        except.print(mu);

        except
    }
}

fn map_condition(mu: &Mu, keyword: Tag) -> Result<Condition> {
    #[allow(clippy::unnecessary_to_owned)]
    let condmap = CONDMAP
        .to_vec()
        .into_iter()
        .find(|cond| keyword.eq_(cond.0));

    match condmap {
        Some(entry) => Ok(entry.1),
        _ => Err(Except::raise(
            mu,
            Condition::Syntax,
            "exception::map_condition",
            keyword,
        )),
    }
}

pub trait MuFunction {
    fn mu_with_ex(mu: &Mu, fp: &mut Frame) -> Result<()>;
    fn mu_raise(mu: &Mu, fp: &mut Frame) -> Result<()>;
}

impl MuFunction for Exception {
    fn mu_raise(mu: &Mu, fp: &mut Frame) -> Result<()> {
        let condition = fp.argv[0];
        let src = fp.argv[1];

        fp.value = src;
        match Tag::type_of(mu, condition) {
            Type::Keyword => match map_condition(mu, condition) {
                Ok(cond) => {
                    Exception::raise(mu, cond, "mu:raise", src);
                    Ok(())
                }
                Err(_) => Err(Except::raise(mu, Condition::Type, "mu:raise", condition)),
            },
            _ => Err(Except::raise(mu, Condition::Type, "mu:raise", condition)),
        }
    }

    fn mu_with_ex(mu: &Mu, fp: &mut Frame) -> Result<()> {
        let handler = fp.argv[0];
        let thunk = fp.argv[1];

        fp.value = match Tag::type_of(mu, thunk) {
            Type::Function => match Tag::type_of(mu, handler) {
                Type::Function => match mu.apply(thunk, Tag::nil()) {
                    Ok(v) => v,
                    Err(e) => match mu.apply(
                        handler,
                        Cons::new(
                            Symbol::keyword("error"),
                            Cons::new(e.tag, Tag::nil()).evict(mu),
                        )
                        .evict(mu),
                    ) {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    },
                },
                _ => return Err(Except::raise(mu, Condition::Type, "mu:with-ex", handler)),
            },
            _ => return Err(Except::raise(mu, Condition::Type, "mu:with-ex", thunk)),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exception() {
        assert!(true)
    }
}
