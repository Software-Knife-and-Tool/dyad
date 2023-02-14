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
        core::{
            classes::{Tag, Type},
            exception,
            exception::{Condition, Exception},
            mu::{Core as _, Mu},
            namespace::Core as _,
        },
        types::{
            cons::{Cons, ConsIter, Properties as _},
            fixnum::Fixnum,
            function::{Function, Properties as _},
            ivector::{TypedVec, VecType, VectorIter},
            symbol::{Core as _, Properties as _, Symbol},
            vector::{Core as _, Vector},
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
        match Tag::type_of(mu, func) {
            Type::Symbol => {
                if Symbol::is_unbound(mu, func) {
                    Err(Exception::raise(
                        mu,
                        Condition::Unbound,
                        "frame::apply",
                        func,
                    ))
                } else {
                    self.apply(mu, Symbol::value_of(mu, func))
                }
            }
            Type::Function => match Tag::type_of(mu, Function::form_of(mu, func)) {
                Type::Null => Ok(Tag::nil()),
                Type::Fixnum => {
                    let nreqs = Fixnum::as_i64(mu, Function::nreq_of(mu, func)) as usize;
                    let nargs = self.argv.len();

                    if nargs != nreqs {
                        return Err(Exception::raise(mu, Condition::Arity, "frame::apply", func));
                    }

                    let fn_off = Fixnum::as_i64(mu, Function::form_of(mu, func)) as usize;
                    let (_, _, _, fnc) = Mu::map_core(fn_off);

                    // Self::stack_push(mu, self);

                    match fnc(mu, &mut self) {
                        Ok(_) => Ok(self.value),
                        Err(e) => Err(e),
                    }

                    // Self::stack_pop(mu);
                }
                Type::Cons => {
                    let nreqs = Fixnum::as_i64(mu, Function::nreq_of(mu, func)) as usize;
                    let nargs = self.argv.len();

                    if nargs != nreqs {
                        return Err(Exception::raise(mu, Condition::Arity, "frame::apply", func));
                    }

                    let mut value = Tag::nil();

                    // Self::dynamic_push(mu, (self.func, &self.argv));
                    self.lexical_push(mu);

                    for cons in ConsIter::new(mu, Function::form_of(mu, func)) {
                        value = match mu.eval(Cons::car(mu, cons)) {
                            Ok(value) => value,
                            Err(e) => return Err(e),
                        };
                    }

                    Self::lexical_pop(mu, Function::frame_of(mu, func));
                    // Self::dynamic_pop(mu);

                    Ok(value)
                }
                _ => Err(Exception::raise(
                    mu,
                    Condition::Type,
                    "frame::apply::car",
                    func,
                )),
            },
            _ => Err(Exception::raise(mu, Condition::Type, "frame::apply", func)),
        }
    }

    fn to_vector(&self, mu: &Mu) -> Tag {
        let mut vec: Vec<Tag> = vec![Symbol::keyword("frame"), self.func];

        for arg in &self.argv {
            vec.push(*arg)
        }

        TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu)
    }

    fn from_vector(mu: &Mu, vec: Tag) -> Self {
        match Tag::type_of(mu, vec) {
            Type::Vector => {
                let vtype = Vector::r#ref(mu, vec, 0).unwrap();
                let func = Vector::r#ref(mu, vec, 1).unwrap();

                match Tag::type_of(mu, func) {
                    Type::Function => {
                        if !vtype.eq_(Tag::t()) {
                            panic!("internal: vector type inconsistency")
                        }

                        let mut args = Vec::new();

                        for arg in VectorIter::new(mu, vec).skip(2) {
                            args.push(arg)
                        }

                        Frame {
                            func,
                            argv: args,
                            value: Tag::nil(),
                        }
                    }
                    _ => panic!("internal: vector type inconsistency"),
                }
            }
            _ => panic!("internal: frame type inconsistency"),
        }
    }

    fn lexical_push(self, mu: &Mu) {
        let id = Function::frame_of(mu, self.func).as_u64();
        let mut lexical_ref: RefMut<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow_mut();

        if let std::collections::hash_map::Entry::Vacant(e) = lexical_ref.entry(id) {
            e.insert(RefCell::new(vec![self]));
        } else {
            let mut vec_ref: RefMut<Vec<Frame>> = lexical_ref[&id].borrow_mut();
            vec_ref.push(self);
        }
    }

    fn lexical_pop(mu: &Mu, frame: Tag) {
        let lexical_ref: Ref<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow();
        let mut vec_ref: RefMut<Vec<Frame>> = lexical_ref[&frame.as_u64()].borrow_mut();

        vec_ref.pop();
    }

    #[allow(dead_code)]
    fn dynamic_push(mu: &Mu, frame: (Tag, Vec<Tag>)) {
        let mut dynamic_ref: RefMut<Vec<(Tag, Vec<Tag>)>> = mu.dynamic.borrow_mut();

        dynamic_ref.push(frame);
    }

    #[allow(dead_code)]
    fn dynamic_pop(mu: &Mu) {
        let mut dynamic_ref: RefMut<Vec<(Tag, Vec<Tag>)>> = mu.dynamic.borrow_mut();

        dynamic_ref.pop();
    }

    fn frame_ref(mu: &Mu, id: u64, offset: usize) -> Option<Tag> {
        let lexical_ref: Ref<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow();
        let vec_ref: Ref<Vec<Frame>> = lexical_ref[&id].borrow();

        Some(vec_ref[vec_ref.len() - 1].argv[offset])
    }
}

pub trait MuFunction {
    fn mu_context(_: &Mu, fp: &mut Frame) -> exception::Result<()>;
    fn mu_fr_lexv(_: &Mu, fp: &mut Frame) -> exception::Result<()>;
    fn mu_fr_pop(_: &Mu, fp: &mut Frame) -> exception::Result<()>;
    fn mu_fr_push(_: &Mu, fp: &mut Frame) -> exception::Result<()>;
    fn mu_fr_ref(mu: &Mu, fp: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Frame {
    fn mu_context(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Tag::nil();
        Ok(())
    }

    fn mu_fr_lexv(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let func = fp.argv[0];

        fp.value = match Tag::type_of(mu, func) {
            Type::Function => {
                let id = Function::frame_of(mu, func).as_u64();
                let lexical_ref: Ref<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow();
                let vec_ref: Ref<Vec<Frame>> = lexical_ref[&id].borrow();

                vec_ref[0].to_vector(mu)
            }
            _ => return Err(Exception::raise(mu, Condition::Type, "mu:fr-lexv", func)),
        };

        Ok(())
    }

    fn mu_fr_pop(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = fp.argv[0];

        match Tag::type_of(mu, fp.value) {
            Type::Function => Self::lexical_pop(mu, fp.value),
            _ => return Err(Exception::raise(mu, Condition::Type, "mu:fr-pop", fp.value)),
        }

        Ok(())
    }

    fn mu_fr_push(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = fp.argv[0];

        match Tag::type_of(mu, fp.value) {
            Type::Vector => Self::from_vector(mu, fp.value).lexical_push(mu),
            _ => {
                return Err(Exception::raise(
                    mu,
                    Condition::Type,
                    "mu:fr-push",
                    fp.value,
                ))
            }
        }

        Ok(())
    }

    fn mu_fr_ref(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let frame = fp.argv[0];
        let offset = fp.argv[1];

        match Tag::type_of(mu, frame) {
            Type::Fixnum => match Tag::type_of(mu, offset) {
                Type::Fixnum => match Frame::frame_ref(
                    mu,
                    Fixnum::as_i64(mu, frame) as u64,
                    Fixnum::as_i64(mu, offset) as usize,
                ) {
                    Some(tag) => {
                        fp.value = tag;
                        Ok(())
                    }
                    None => Err(Exception::raise(mu, Condition::Type, "mu:lex-ref", frame)),
                },
                _ => Err(Exception::raise(mu, Condition::Type, "mu:lex-ref", offset)),
            },
            _ => Err(Exception::raise(mu, Condition::Type, "mu:lex-ref", frame)),
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
