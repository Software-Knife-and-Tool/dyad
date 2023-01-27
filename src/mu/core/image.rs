//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu image
use {
    crate::{
        classes::{
            cons::{Cons, Core as _},
            fixnum::Fixnum,
            // float::Float,
            symbol::{Core as _, Symbol},
        },
        core::{
            classes::{Class, Tag},
            exception,
            exception::{Condition, Except},
            frame::Frame,
            mu::Mu,
        },
        image,
    },
    std::cell::Ref,
};

lazy_static! {
    static ref INFOTYPEMAP: Vec<(Tag, Class)> = vec![
        (Symbol::keyword("cons"), Class::Cons),
        (Symbol::keyword("func"), Class::Function),
        (Symbol::keyword("nil"), Class::Null),
        (Symbol::keyword("ns"), Class::Namespace),
        (Symbol::keyword("stream"), Class::Stream),
        (Symbol::keyword("symbol"), Class::Symbol),
        (Symbol::keyword("t"), Class::T),
        (Symbol::keyword("vector"), Class::Vector),
    ];
}

fn to_type(keyword: Tag) -> Option<Class> {
    INFOTYPEMAP
        .iter()
        .copied()
        .find(|tab| keyword.eq_(tab.0))
        .map(|tab| tab.1)
}

pub trait MuFunction {
    fn mu_hp_info(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_hp_type(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Mu {
    fn mu_hp_info(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let hp: (usize, usize);

        {
            let heap_ref: Ref<image::heap::Heap> = mu.heap.borrow();

            hp = (heap_ref.page_size, heap_ref.npages);
        }

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

        match Tag::class_of(mu, type_key) {
            Class::Keyword => match to_type(type_key) {
                Some(htype) => match Tag::class_of(mu, of_key) {
                    Class::Keyword => {
                        let type_info;

                        {
                            let heap_ref: Ref<image::heap::Heap> = mu.heap.borrow();
                            #[allow(clippy::type_complexity)]
                            let alloc_ref: Ref<
                                Vec<(u8, usize, usize, usize, usize)>,
                            > = heap_ref.alloc_map.borrow();

                            type_info = alloc_ref[htype as usize];
                        }

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
