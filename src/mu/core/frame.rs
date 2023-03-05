//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
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
            cons::{Cons, ConsIter},
            fixnum::Fixnum,
            function::Function,
            r#struct::{Core as _, Struct},
            symbol::{Core as _, Symbol},
            vecimage::VectorIter,
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
    pub argv: Vec<u64>,
    pub value: Tag,
}

impl Frame {
    fn to_tag(&self, mu: &Mu) -> Tag {
        let mut vec: Vec<Tag> = vec![self.func];

        for arg in &self.argv {
            vec.push(Tag::from_u64(*arg))
        }

        Struct::new(mu, "frame".to_string(), vec).evict(mu)
    }

    fn from_tag(mu: &Mu, tag: Tag) -> Self {
        match Tag::type_of(mu, tag) {
            Type::Struct => {
                let stype = Struct::stype(mu, tag);
                let frame = Struct::vector(mu, tag);

                let func = Vector::r#ref(mu, frame, 0).unwrap();

                match Tag::type_of(mu, func) {
                    Type::Function => {
                        if !stype.eq_(Symbol::keyword("frame")) {
                            panic!("internal: frame type inconsistency")
                        }

                        let mut args = Vec::new();

                        for arg in VectorIter::new(mu, frame).skip(1) {
                            args.push(Tag::as_u64(&arg))
                        }

                        Frame {
                            func,
                            argv: args,
                            value: Tag::nil(),
                        }
                    }
                    _ => panic!("internal: frame type inconsistency"),
                }
            }
            _ => panic!("internal: frame type inconsistency"),
        }
    }

    // context stack
    #[allow(dead_code)]
    fn context_push(self, mu: &Mu) {
        let mut dynamic_ref: RefMut<Vec<(u64, usize)>> = mu.dynamic.borrow_mut();
        let offset = Self::frame_stack_len(mu, Function::frame_of(mu, self.func)) - 1;

        dynamic_ref.push((self.func.as_u64(), offset));
    }

    #[allow(dead_code)]
    fn context_pop(mu: &Mu) {
        let mut dynamic_ref: RefMut<Vec<(u64, usize)>> = mu.dynamic.borrow_mut();

        dynamic_ref.pop();
    }

    #[allow(dead_code)]
    fn context_ref(mu: &Mu, index: usize) -> (Tag, usize) {
        let dynamic_ref: Ref<Vec<(u64, usize)>> = mu.dynamic.borrow();
        let (func, offset) = dynamic_ref[index];

        (Tag::from_u64(func), offset)
    }

    // frame stacks
    fn frame_stack_push(self, mu: &Mu) {
        let id = Function::frame_of(mu, self.func).as_u64();
        let mut stack_ref: RefMut<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow_mut();

        if let std::collections::hash_map::Entry::Vacant(e) = stack_ref.entry(id) {
            e.insert(RefCell::new(vec![self]));
        } else {
            let mut vec_ref: RefMut<Vec<Frame>> = stack_ref[&id].borrow_mut();
            vec_ref.push(self);
        }
    }

    fn frame_stack_pop(mu: &Mu, id: Tag) {
        let stack_ref: Ref<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow();
        let mut vec_ref: RefMut<Vec<Frame>> = stack_ref[&id.as_u64()].borrow_mut();

        vec_ref.pop();
    }

    #[allow(dead_code)]
    fn frame_stack_ref(mu: &Mu, id: Tag, _offset: usize) -> Frame {
        let _stack_ref: Ref<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow();
        let _vec_ref: Ref<Vec<Frame>> = _stack_ref[&id.as_u64()].borrow();

        Frame {
            func: Tag::nil(),
            argv: Vec::<u64>::new(),
            value: Tag::nil(),
        }
    }

    fn frame_stack_len(mu: &Mu, id: Tag) -> usize {
        let stack_ref: Ref<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow();
        let vec_ref: Ref<Vec<Frame>> = stack_ref[&id.as_u64()].borrow();

        vec_ref.len()
    }

    // frame reference
    fn frame_ref(mu: &Mu, id: u64, offset: usize) -> Option<Tag> {
        let stack_ref: Ref<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow();
        let vec_ref: Ref<Vec<Frame>> = stack_ref[&id].borrow();

        Some(Tag::from_u64(vec_ref[vec_ref.len() - 1].argv[offset]))
    }

    // apply
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

                    match fnc(mu, &mut self) {
                        Ok(_) => Ok(self.value),
                        Err(e) => Err(e),
                    }
                }
                Type::Cons => {
                    let nreqs = Fixnum::as_i64(mu, Function::nreq_of(mu, func)) as usize;
                    let nargs = self.argv.len();

                    if nargs != nreqs {
                        return Err(Exception::raise(mu, Condition::Arity, "frame::apply", func));
                    }

                    let mut value = Tag::nil();

                    self.frame_stack_push(mu);
                    // self.context_push(mu);

                    for cons in ConsIter::new(mu, Function::form_of(mu, func)) {
                        value = match mu.eval(Cons::car(mu, cons)) {
                            Ok(value) => value,
                            Err(e) => return Err(e),
                        };
                    }

                    // Self::context_pop(mu);
                    Self::frame_stack_pop(mu, Function::frame_of(mu, func));

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
}

pub trait MuFunction {
    fn mu_context(_: &Mu, fp: &mut Frame) -> exception::Result<()>;
    fn mu_fr_get(_: &Mu, fp: &mut Frame) -> exception::Result<()>;
    fn mu_fr_pop(_: &Mu, fp: &mut Frame) -> exception::Result<()>;
    fn mu_fr_push(_: &Mu, fp: &mut Frame) -> exception::Result<()>;
    fn mu_fr_ref(mu: &Mu, fp: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Frame {
    fn mu_context(_: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Tag::nil();
        Ok(())
    }

    fn mu_fr_get(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let func = Tag::from_u64(fp.argv[0]);

        fp.value = match Tag::type_of(mu, func) {
            Type::Function => {
                let id = Function::frame_of(mu, func).as_u64();
                let lexical_ref: Ref<HashMap<u64, RefCell<Vec<Frame>>>> = mu.lexical.borrow();
                let vec_ref: Ref<Vec<Frame>> = lexical_ref[&id].borrow();

                vec_ref[0].to_tag(mu)
            }
            _ => return Err(Exception::raise(mu, Condition::Type, "mu:fr-get", func)),
        };

        Ok(())
    }

    fn mu_fr_pop(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Tag::from_u64(fp.argv[0]);

        match Tag::type_of(mu, fp.value) {
            Type::Function => Self::frame_stack_pop(mu, fp.value),
            _ => return Err(Exception::raise(mu, Condition::Type, "mu:fr-pop", fp.value)),
        }

        Ok(())
    }

    fn mu_fr_push(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Tag::from_u64(fp.argv[0]);

        match Tag::type_of(mu, fp.value) {
            Type::Vector => Self::from_tag(mu, fp.value).frame_stack_push(mu),
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
        let frame = Tag::from_u64(fp.argv[0]);
        let offset = Tag::from_u64(fp.argv[1]);

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
