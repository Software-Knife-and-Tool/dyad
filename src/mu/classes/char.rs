//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu char type
use crate::{
    classes::{
        indirect_vector::{TypedVec, VecType as _},
        stream::{Core as _, Stream},
        vector::Core as _,
    },
    core::{
        classes::{DirectClass, Tag},
        exception,
        mu::{Core as _, Mu},
    },
};

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Char {
    Direct(u64),
}

impl Char {
    pub fn as_char(mu: &Mu, ch: Tag) -> char {
        ((ch.data(mu) & 0xff) as u8) as char
    }

    pub fn as_tag(ch: char) -> Tag {
        Tag::to_direct(ch as u64, 1, DirectClass::Char)
    }
}

pub trait Core {
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn view(_: &Mu, _: Tag) -> Tag;
}

impl Core for Char {
    fn write(mu: &Mu, chr: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        let ch: u8 = (chr.data(mu) & 0xff) as u8;

        if escape {
            match mu.write_string("#\\".to_string(), stream) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        match Stream::write_char(mu, stream, ch as char) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn view(mu: &Mu, chr: Tag) -> Tag {
        let vec = TypedVec::<Vec<Tag>> { vec: vec![chr] };

        vec.vec.to_vector().evict(mu)
    }
}

#[cfg(test)]
mod tests {
    use crate::classes::char::Char;

    #[test]
    fn as_tag() {
        match Char::as_tag('a') {
            _ => assert_eq!(true, true),
        }
    }
}
