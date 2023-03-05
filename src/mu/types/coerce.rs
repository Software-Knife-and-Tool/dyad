//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu coercion
#![allow(unused_imports)]

use crate::{
    core::{
        classes::{Tag, Type},
        exception,
        exception::{Condition, Exception},
        frame::Frame,
        mu::{Core as _, Mu},
    },
    image,
    types::{
        char::{Char, Core as _},
        cons::{Cons, Core as _},
        fixnum::Fixnum,
        symbol::{Core as _, Symbol},
    },
};

trait Core {
    fn coerce(_: &Mu, _: Tag, _: Type) -> Option<Tag>;
}

impl Core for Mu {
    fn coerce(mu: &Mu, src: Tag, to_type: Type) -> Option<Tag> {
        match to_type {
            Type::Char => match Tag::type_of(mu, src) {
                Type::Fixnum => Some(Char::as_tag(
                    char::from_u32(Fixnum::as_i64(mu, src) as u32).unwrap(),
                )),
                _ => None,
            },
            Type::Fixnum => match Tag::type_of(mu, src) {
                Type::Char => Some(Fixnum::as_tag(Char::as_char(mu, src) as i64)),
                _ => None,
            },
            Type::Float => Some(Tag::nil()),
            _ => None,
        }
    }
}

pub trait MuFunction {
    fn mu_coerce(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Mu {
    fn mu_coerce(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let src = fp.argv[0];
        let to_key = fp.argv[1];

        fp.value = match Tag::key_type(to_key) {
            Some(to_type) => match Self::coerce(mu, src, to_type) {
                Some(tag) => tag,
                None => return Err(Exception::raise(mu, Condition::Type, "mu:coerce", to_key)),
            },
            None => return Err(Exception::raise(mu, Condition::Type, "mu:coerce", to_key)),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn coerce() {
        assert_eq!(2 + 2, 4);
    }
}
