//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu coercion
#![allow(unused_imports)]

use crate::{
    classes::{
        char::{Char, Core as _},
        cons::{Cons, Core as _},
        fixnum::Fixnum,
        symbol::{Core as _, Symbol},
    },
    core::{
        classes::{Class, Tag},
        exception,
        exception::{Condition, Except},
        frame::Frame,
        mu::{Core as _, Mu},
    },
    image,
};

trait Core {
    fn coerce(_: &Mu, _: Tag, _: Class) -> Option<Tag>;
}

impl Core for Mu {
    fn coerce(mu: &Mu, src: Tag, to_type: Class) -> Option<Tag> {
        match to_type {
            Class::Char => match Tag::class_of(mu, src) {
                Class::Fixnum => Some(Char::as_tag(
                    char::from_u32(Fixnum::as_i64(mu, src) as u32).unwrap(),
                )),
                _ => None,
            },
            Class::Fixnum => match Tag::class_of(mu, src) {
                Class::Char => Some(Fixnum::as_tag(Char::as_char(mu, src) as i64)),
                _ => None,
            },
            Class::Float => Some(Tag::nil()),
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
                None => return Err(Except::raise(mu, Condition::Type, "mu:coerce", to_key)),
            },
            None => return Err(Except::raise(mu, Condition::Type, "mu:coerce", to_key)),
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
