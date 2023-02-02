//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! function call frame
//!    Frame
//!    apply
//!    frame_push
//!    frame_pop
//!    frame_ref
use {
    crate::{
        classes::{
            cons::{Cons, ConsIter, Properties as _},
            fixnum::Fixnum,
            function::{Function, Properties as _},
            symbol::{Core as _, Properties as _, Symbol},
        },
        core::{
            classes::{Class, Tag},
            exception,
            exception::{Condition, Except},
            mu::{Core as _, Mu},
            namespace::Core as _,
        },
    },
    std::{
        cell::{Ref, RefCell, RefMut},
        collections::HashMap,
    },
};

pub struct Frame {
    pub func: Tag,
    pub argv: Vec<Tag>,
    pub value: Tag,
}

impl Frame {
    pub fn apply(mut self, mu: &Mu, func: Tag) -> exception::Result<Tag> {
        match Tag::class_of(mu, func) {
            Class::Symbol => {
                if Symbol::is_unbound(mu, func) {
                    Err(Except::raise(mu, Condition::Unbound, "frame::apply", func))
                } else {
                    self.apply(mu, Symbol::value_of(mu, func))
                }
            }
            Class::Function => match Tag::class_of(mu, Function::func_of(mu, func)) {
                Class::Null => Ok(Tag::nil()),
                Class::Fixnum => {
                    let nreqs = Fixnum::as_i64(mu, Function::nreq_of(mu, func)) as usize;
                    let nargs = self.argv.len();

                    if nargs != nreqs {
                        return Err(Except::raise(mu, Condition::Arity, "frame::apply", func));
                    }

                    let fn_off = Fixnum::as_i64(mu, Function::func_of(mu, func)) as usize;
                    let (_, _, _, fnc) = Mu::functionmap(fn_off);

                    // Self::stack_push(mu, self);

                    match fnc(mu, &mut self) {
                        Ok(_) => Ok(self.value),
                        Err(e) => Err(e),
                    }

                    // Self::stack_pop(mu);
                }
                Class::Cons => {
                    let nreqs = Fixnum::as_i64(mu, Function::nreq_of(mu, func)) as usize;
                    let nargs = self.argv.len();

                    if nargs != nreqs {
                        return Err(Except::raise(mu, Condition::Arity, "frame::apply", func));
                    }

                    let mut value = Tag::nil();

                    self.lexical_push(mu);
                    // Self::stack_push(mu, self);

                    for cons in ConsIter::new(mu, Function::func_of(mu, func)) {
                        value = match mu.eval(Cons::car(mu, cons)) {
                            Ok(value) => value,
                            Err(e) => return Err(e),
                        };
                    }

                    // Self::stack_pop(mu);
                    Self::lexical_pop(mu, Function::frame_of(mu, func));

                    Ok(value)
                }
                _ => Err(Except::raise(
                    mu,
                    Condition::Type,
                    "frame::apply::car",
                    func,
                )),
            },
            _ => Err(Except::raise(mu, Condition::Type, "frame::apply", func)),
        }
    }

    pub fn lexical_push(self, mu: &Mu) {
        let id = Function::frame_of(mu, self.func).as_u64();
        let mut lexical_ref: RefMut<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow_mut();

        if let std::collections::hash_map::Entry::Vacant(e) = lexical_ref.entry(id) {
            e.insert(RefCell::new(vec![self]));
        } else {
            let mut vec_ref: RefMut<Vec<Frame>> = lexical_ref[&id].borrow_mut();
            vec_ref.push(self);
        }
    }

    pub fn lexical_pop(mu: &Mu, frame: Tag) {
        let lexical_ref: Ref<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow();
        let mut vec_ref: RefMut<Vec<Frame>> = lexical_ref[&frame.as_u64()].borrow_mut();

        vec_ref.pop();
    }

    pub fn dynamic_push(mu: &Mu, frame: Frame) {
        let mut dynamic_ref: RefMut<Vec<Frame>> = mu.dynamic.borrow_mut();

        dynamic_ref.push(frame);
    }

    pub fn dynamic_pop(mu: &Mu) {
        let mut dynamic_ref: RefMut<Vec<Frame>> = mu.dynamic.borrow_mut();

        dynamic_ref.pop();
    }

    pub fn frame_ref(mu: &Mu, id: u64, offset: usize) -> Option<Tag> {
        let lexical_ref: Ref<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow();
        let vec_ref: Ref<Vec<Frame>> = lexical_ref[&id].borrow();

        Some(vec_ref[vec_ref.len() - 1].argv[offset])
    }
}

pub trait MuFunction {
    fn mu_context(_: &Mu, fp: &mut Frame) -> exception::Result<()>;
    fn mu_fr_pop(_: &Mu, fp: &mut Frame) -> exception::Result<()>;
    fn mu_fr_push(_: &Mu, fp: &mut Frame) -> exception::Result<()>;
    fn mu_fr_ref(mu: &Mu, fp: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Frame {
    fn mu_context(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Tag::nil();
        Ok(())
    }

    fn mu_fr_pop(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Tag::nil();
        Ok(())
    }

    fn mu_fr_push(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Tag::nil();
        Ok(())
    }

    fn mu_fr_ref(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let frame = fp.argv[0];
        let offset = fp.argv[1];

        match Tag::class_of(mu, frame) {
            Class::Fixnum => match Tag::class_of(mu, offset) {
                Class::Fixnum => match Frame::frame_ref(
                    mu,
                    Fixnum::as_i64(mu, frame) as u64,
                    Fixnum::as_i64(mu, offset) as usize,
                ) {
                    Some(tag) => {
                        fp.value = tag;
                        Ok(())
                    }
                    None => Err(Except::raise(mu, Condition::Type, "mu:lex-ref", frame)),
                },
                _ => Err(Except::raise(mu, Condition::Type, "mu:lex-ref", offset)),
            },
            _ => Err(Except::raise(mu, Condition::Type, "mu:lex-ref", frame)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn frame() {
        assert_eq!(2 + 2, 4);
    }
}
