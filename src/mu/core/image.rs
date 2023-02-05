//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu image
use {
    crate::{
        core::{
            classes::{Tag, Type},
            exception,
            exception::{Condition, Except},
            frame::Frame,
            mu::Mu,
        },
        image,
        types::{
            cons::{Cons, Core as _},
            fixnum::Fixnum,
            symbol::{Core as _, Symbol},
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
        (Symbol::keyword("t"), Type::T),
        (Symbol::keyword("vector"), Type::Vector),
    ];
}

pub trait Core {
    fn to_type(_: Tag) -> Option<Type>;
    fn hp_info(_: &Mu) -> (usize, usize);
    fn hp_type(_: &Mu, _: Type) -> (u8, usize, usize, usize, usize);
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

    fn hp_type(mu: &Mu, htype: Type) -> (u8, usize, usize, usize, usize) {
        let heap_ref: Ref<image::heap::Heap> = mu.heap.borrow();

        #[allow(clippy::type_complexity)]
        let alloc_ref: Ref<Vec<(u8, usize, usize, usize, usize)>> = heap_ref.alloc_map.borrow();

        alloc_ref[htype as usize]
    }
}

pub trait MuFunction {
    fn mu_hp_info(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_hp_type(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Mu {
    fn mu_hp_info(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let hp = Self::hp_info(mu);

        fp.value = Cons::list(
            mu,
            &[
                Cons::new(Symbol::keyword("pagesz"), Fixnum::as_tag(hp.0 as i64)).evict(mu),
                Cons::new(Symbol::keyword("pages"), Fixnum::as_tag(hp.1 as i64)).evict(mu),
            ],
        );

        Ok(())
    }

    fn mu_hp_type(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let type_key = fp.argv[0];
        let of_key = fp.argv[1];

        match Tag::type_of(mu, type_key) {
            Type::Keyword => match Self::to_type(type_key) {
                Some(htype) => match Tag::type_of(mu, of_key) {
                    Type::Keyword => {
                        let type_info = Self::hp_type(mu, htype);

                        fp.value = if of_key.eq_(Symbol::keyword("alloc")) {
                            Fixnum::as_tag(type_info.1 as i64)
                        } else if of_key.eq_(Symbol::keyword("in-use")) {
                            Fixnum::as_tag(type_info.2 as i64)
                        } else if of_key.eq_(Symbol::keyword("free")) {
                            Fixnum::as_tag(type_info.3 as i64)
                        } else if of_key.eq_(Symbol::keyword("size")) {
                            Fixnum::as_tag(type_info.4 as i64)
                        } else {
                            return Err(Except::raise(mu, Condition::Type, "mu:hp-type", of_key));
                        };

                        Ok(())
                    }
                    _ => Err(Except::raise(mu, Condition::Type, "mu:hp-type", of_key)),
                },
                None => Err(Except::raise(mu, Condition::Range, "mu:hp-type", type_key)),
            },
            _ => Err(Except::raise(mu, Condition::Type, "mu:hp-type", type_key)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn image() {
        assert_eq!(2 + 2, 4);
    }
}
