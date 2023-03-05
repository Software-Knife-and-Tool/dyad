//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu image
use {
    crate::{
        core::{
            classes::{Tag, Type},
            exception,
            frame::Frame,
            mu::Mu,
        },
        image,
        types::{
            fixnum::Fixnum,
            symbol::{Core as _, Symbol},
            vecimage::{TypedVec, VecType},
            vector::Core as _,
        },
    },
    std::cell::Ref,
};

lazy_static! {
    static ref TYPEMAP: Vec<(Tag, Type)> = vec![
        (Symbol::keyword("cons"), Type::Cons),
        (Symbol::keyword("func"), Type::Function),
        (Symbol::keyword("nil"), Type::Null),
        (Symbol::keyword("ns"), Type::Namespace),
        (Symbol::keyword("stream"), Type::Stream),
        (Symbol::keyword("symbol"), Type::Symbol),
        (Symbol::keyword("struct"), Type::Struct),
        (Symbol::keyword("t"), Type::T),
        (Symbol::keyword("vector"), Type::Vector),
    ];
    static ref INFOTYPE: Vec<Tag> = vec![
        Symbol::keyword("cons"),
        Symbol::keyword("func"),
        Symbol::keyword("ns"),
        Symbol::keyword("stream"),
        Symbol::keyword("struct"),
        Symbol::keyword("symbol"),
        Symbol::keyword("vector"),
    ];
}

pub trait Core {
    fn to_type(_: Tag) -> Option<Type>;
    fn hp_info(_: &Mu) -> (usize, usize);
    fn hp_type(_: &Mu, _: Type) -> (u8, usize, usize, usize);
}

impl Core for Mu {
    fn to_type(keyword: Tag) -> Option<Type> {
        TYPEMAP
            .iter()
            .copied()
            .find(|tab| keyword.eq_(tab.0))
            .map(|tab| tab.1)
    }

    fn hp_info(mu: &Mu) -> (usize, usize) {
        let heap_ref: Ref<image::heap::Heap> = mu.heap.borrow();

        (heap_ref.page_size, heap_ref.npages)
    }

    fn hp_type(mu: &Mu, htype: Type) -> (u8, usize, usize, usize) {
        let heap_ref: Ref<image::heap::Heap> = mu.heap.borrow();

        #[allow(clippy::type_complexity)]
        let alloc_ref: Ref<Vec<(u8, usize, usize, usize)>> = heap_ref.alloc_map.borrow();

        alloc_ref[htype as usize]
    }
}

pub trait MuFunction {
    fn mu_hp_info(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Mu {
    fn mu_hp_info(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let (pagesz, npages) = Self::hp_info(mu);

        let mut vec = vec![
            Tag::t(),
            Fixnum::as_tag((pagesz * npages) as i64),
            Fixnum::as_tag(npages as i64),
            Fixnum::as_tag(npages as i64),
        ];

        for htype in INFOTYPE.iter() {
            let (_, size, alloc, in_use) = Self::hp_type(mu, Self::to_type(*htype).unwrap());

            vec.push(*htype);
            vec.push(Fixnum::as_tag(size as i64));
            vec.push(Fixnum::as_tag(alloc as i64));
            vec.push(Fixnum::as_tag(in_use as i64));
        }

        fp.value = TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn image() {
        assert_eq!(2 + 2, 4);
    }
}
