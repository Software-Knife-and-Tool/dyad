//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu classes
#![allow(unused_braces)]
#![allow(clippy::identity_op)]
use {
    crate::{
        core::{exception, frame::Frame, mu::Mu},
        image,
        types::symbol::{Core as _, Symbol},
    },
    modular_bitfield::specifiers::{B3, B56, B61},
    num_enum::TryFromPrimitive,
    std::{cell::Ref, fmt},
};

// tag storage classes
#[derive(Copy, Clone)]
pub enum Tag {
    Fixnum(i64),
    Direct(Direct),
    Indirect(TagU64),
}

// classes
#[derive(Eq, PartialEq, Copy, Clone, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Type {
    Byte,
    Char,
    Cons,
    Fixnum,
    Float,
    Function,
    Keyword,
    Namespace,
    Null,
    Stream,
    Symbol,
    T,
    Vector,
}

// chosen to give fixnums 62 bits
#[derive(BitfieldSpecifier, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TagType {
    EFixnum = 0,
    Float = 1,
    Symbol = 2,
    Function = 3,
    OFixnum = 4,
    Cons = 5,
    Direct = 6,
    Heap = 7,
}

// little-endian tag format
#[derive(Copy, Clone)]
#[bitfield]
#[repr(u64)]
pub struct TagU64 {
    #[bits = 3]
    pub tag: TagType,
    pub offset: B61,
}

#[derive(BitfieldSpecifier, Copy, Clone)]
pub enum DirectType {
    Char = 0,
    Byte = 1,
    Keyword = 2,
    Float = 3,
}

// little endian direct tag format
#[derive(Copy, Clone)]
#[bitfield]
#[repr(u64)]
pub struct Direct {
    #[bits = 3]
    pub tag: TagType,
    #[bits = 2]
    pub dtype: DirectType,
    pub length: B3,
    pub data: B56,
}

lazy_static! {
    pub static ref T: Tag = Tag::to_direct('t' as u64, 1, DirectType::Keyword);
    pub static ref NIL: Tag = Tag::to_direct(
        (('l' as u64) << 16) | (('i' as u64) << 8) | ('n' as u64),
        3,
        DirectType::Keyword
    );
    pub static ref TYPEKEYMAP: Vec::<(Type, Tag)> = vec![
        (Type::Byte, Symbol::keyword("byte")),
        (Type::Char, Symbol::keyword("char")),
        (Type::Cons, Symbol::keyword("cons")),
        (Type::Fixnum, Symbol::keyword("fixnum")),
        (Type::Float, Symbol::keyword("float")),
        (Type::Function, Symbol::keyword("func")),
        (Type::Keyword, Symbol::keyword("keyword")),
        (Type::Namespace, Symbol::keyword("ns")),
        (Type::Null, Symbol::keyword("null")),
        (Type::Stream, Symbol::keyword("stream")),
        (Type::Symbol, Symbol::keyword("symbol")),
        (Type::T, Symbol::keyword("t")),
        (Type::Vector, Symbol::keyword("vector")),
    ];
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}: ", self.as_u64()).unwrap();
        match self {
            Tag::Fixnum(as_i64) => write!(f, "is a fixnum {}", as_i64 >> 2),
            Tag::Direct(direct) => write!(f, "is a direct: type {:?}", direct.dtype() as u8),
            Tag::Indirect(main) => write!(f, "is a main: type {:?}", main.tag()),
        }
    }
}

impl Tag {
    pub const DIRECT_STR_MAX: usize = 7;

    pub fn data(&self, mu: &Mu) -> u64 {
        let heap_ref: Ref<image::heap::Heap> = mu.heap.borrow();
        match self {
            Tag::Fixnum(fx) => (*fx >> 2) as u64,
            Tag::Direct(tag) => tag.data(),
            Tag::Indirect(heap) => match heap_ref.info(heap.offset() as usize) {
                Some(info) => match Type::try_from(info.tag_type()) {
                    Ok(etype) => etype as u64,
                    Err(_) => panic!("internal: tag format inconsistency"),
                },
                None => panic!("internal: tag info inconsistency"),
            },
        }
    }

    pub fn length(&self) -> u64 {
        match self {
            Tag::Direct(tag) => tag.length() as u64,
            _ => panic!("internal: direct tag required"),
        }
    }

    pub fn tag_of(&self) -> u64 {
        match self {
            Tag::Fixnum(fx) => (fx & 7) as u64,
            Tag::Direct(tag) => tag.dtype() as u64,
            Tag::Indirect(_) => TagType::Heap as u64,
        }
    }

    pub fn as_slice(&self) -> [u8; 8] {
        match self {
            Tag::Fixnum(tag) => tag.to_le_bytes(),
            Tag::Direct(tag) => tag.into_bytes(),
            Tag::Indirect(tag) => tag.into_bytes(),
        }
    }

    pub fn as_u64(&self) -> u64 {
        u64::from_le_bytes(self.as_slice())
    }

    pub fn eq_(&self, tag: Tag) -> bool {
        self.as_u64() == tag.as_u64()
    }

    pub fn null_(&self) -> bool {
        self.eq_(Self::nil())
    }

    pub fn t() -> Tag {
        *T
    }

    pub fn nil() -> Tag {
        *NIL
    }

    pub fn to_direct(data: u64, len: u8, tag: DirectType) -> Tag {
        let dir = Direct::new()
            .with_data(data)
            .with_length(len)
            .with_dtype(tag)
            .with_tag(TagType::Direct);

        Tag::Direct(dir)
    }

    pub fn from_slice(bits: &[u8]) -> Tag {
        let mut data: [u8; 8] = 0u64.to_le_bytes();
        for (src, dst) in bits.iter().zip(data.iter_mut()) {
            *dst = *src
        }

        let as_u64: u64 = u64::from_le_bytes(data);
        match as_u64 & 0x7 {
            0 | 4 => Tag::Fixnum(as_u64 as i64),
            1 | 2 | 3 | 5 | 7 => Tag::Indirect(TagU64::from(as_u64)),
            6 => Tag::Direct(Direct::from(as_u64)),
            _ => panic!("internal: immediate tag required"),
        }
    }

    pub fn type_of(mu: &Mu, tag: Tag) -> Type {
        let heap_ref: Ref<image::heap::Heap> = mu.heap.borrow();

        if tag.null_() {
            Type::Null
        } else {
            match tag {
                Tag::Fixnum(_) => Type::Fixnum,
                Tag::Direct(direct) => match direct.dtype() {
                    DirectType::Char => Type::Char,
                    DirectType::Byte => Type::Vector,
                    DirectType::Keyword => Type::Keyword,
                    DirectType::Float => Type::Float,
                },
                Tag::Indirect(main) => match main.tag() {
                    TagType::Float => Type::Float,
                    TagType::Symbol => Type::Symbol,
                    TagType::Function => Type::Function,
                    TagType::Cons => Type::Cons,
                    TagType::Heap => match heap_ref.info(main.offset() as usize) {
                        Some(info) => match Type::try_from(info.tag_type()) {
                            Ok(etype) => etype,
                            Err(_) => panic!("internal: tag format inconsistency"),
                        },
                        None => panic!("internal: tag info inconsistency"),
                    },
                    _ => panic!("internal: indirect tag type inconsistency"),
                },
            }
        }
    }

    pub fn type_key(ttype: Type) -> Option<Tag> {
        TYPEKEYMAP
            .iter()
            .copied()
            .find(|map| ttype == map.0)
            .map(|map| map.1)
    }

    pub fn key_type(tag: Tag) -> Option<Type> {
        TYPEKEYMAP
            .iter()
            .copied()
            .find(|map| tag.eq_(map.1))
            .map(|map| map.0)
    }
}

pub trait MuFunction {
    fn mu_eq(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_typeof(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Tag {
    fn mu_eq(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        if fp.argv[0].eq_(fp.argv[1]) {
            fp.value = Tag::t();
        } else {
            fp.value = Tag::nil();
        }

        Ok(())
    }

    fn mu_typeof(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match Self::type_key(Self::type_of(mu, fp.argv[0])) {
            Some(type_key) => type_key,
            None => panic!("internal: type_of inconsistency"),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn types() {
        assert_eq!(2 + 2, 4);
    }
}
