//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu function type
use {
    crate::{
        core::{
            classes::{Tag, TagType, TagU64, Type},
            exception,
            exception::{Condition, Except},
            frame::Frame,
            mu::{Core as _, Mu},
            namespace::Core as _,
        },
        image,
        types::{
            cons::{Cons, Properties as _},
            fixnum::Fixnum,
            indirect_vector::{TypedVec, VecType},
            symbol::{Core as _, Symbol},
            vector::Core as _,
        },
    },
    std::cell::RefMut,
};

#[derive(Copy, Clone)]
pub struct Function {
    func: Tag,  // if a fixnum, it's native, otherwise a body list
    nreq: Tag,  // fixnum nrequired arguments
    frame: Tag, // frame id
}

impl Function {
    pub fn new(func: Tag, nreq: Tag, frame: Tag) -> Self {
        Function { func, nreq, frame }
    }

    pub fn evict(&self, mu: &Mu) -> Tag {
        let image: &[[u8; 8]] = &[
            self.func.as_slice(),
            self.nreq.as_slice(),
            self.frame.as_slice(),
        ];

        let mut heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
        let ind = TagU64::new()
            .with_offset(heap_ref.alloc(image, Type::Function as u8) as u64)
            .with_tag(TagType::Function);

        Tag::Indirect(ind)
    }

    pub fn to_image(mu: &Mu, tag: Tag) -> Self {
        match Tag::type_of(mu, tag) {
            Type::Function => match tag {
                Tag::Indirect(main) => {
                    let heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
                    Function {
                        func: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize, 8).unwrap(),
                        ),
                        nreq: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 8, 8).unwrap(),
                        ),
                        frame: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 16, 8).unwrap(),
                        ),
                    }
                }
                _ => panic!("internal: function type inconsistency"),
            },
            _ => panic!("internal: function type inconsistency"),
        }
    }
}

pub trait Properties {
    fn nreq_of(_: &Mu, _: Tag) -> Tag;
    fn func_of(_: &Mu, _: Tag) -> Tag;
    fn frame_of(_: &Mu, _: Tag) -> Tag;
}

impl Properties for Function {
    fn nreq_of(mu: &Mu, func: Tag) -> Tag {
        match Tag::type_of(mu, func) {
            Type::Function => match func {
                Tag::Indirect(_) => Self::to_image(mu, func).nreq,
                _ => panic!("internal: function type inconsistency"),
            },
            _ => panic!("internal: function type inconsistency"),
        }
    }

    fn func_of(mu: &Mu, func: Tag) -> Tag {
        match Tag::type_of(mu, func) {
            Type::Function => match func {
                Tag::Indirect(_) => Self::to_image(mu, func).func,
                _ => panic!("internal: function type inconsistency"),
            },
            _ => panic!("internal: function type inconsistency"),
        }
    }

    fn frame_of(mu: &Mu, func: Tag) -> Tag {
        match Tag::type_of(mu, func) {
            Type::Function => match func {
                Tag::Indirect(_) => Self::to_image(mu, func).frame,
                _ => panic!("internal: function type inconsistency"),
            },
            _ => panic!("internal: function type inconsistency"),
        }
    }
}

pub trait Core {
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn view(_: &Mu, _: Tag) -> Tag;
}

impl Core for Function {
    fn view(mu: &Mu, func: Tag) -> Tag {
        let vec = TypedVec::<Vec<Tag>> {
            vec: vec![
                Self::nreq_of(mu, func),
                Self::func_of(mu, func),
                Self::frame_of(mu, func),
            ],
        };

        vec.vec.to_vector().evict(mu)
    }

    fn write(mu: &Mu, func: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        match Tag::type_of(mu, func) {
            Type::Function => {
                match mu.write_string("#<function: ".to_string(), stream) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                let form = Function::func_of(mu, func);
                match Tag::type_of(mu, form) {
                    Type::Null | Type::Cons => match mu.write_string(
                        format!(
                            ":lambda [req:{}, tag:{:x}]",
                            Fixnum::as_i64(mu, Function::nreq_of(mu, func)),
                            func.as_u64(),
                        ),
                        stream,
                    ) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    },
                    Type::Fixnum => {
                        let (name, _, nreqs, _) =
                            Mu::functionmap(Fixnum::as_i64(mu, form) as usize);
                        match mu.write_string(format!(":native [req:{nreqs}] mu:{name}"), stream) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    _ => panic!("internal: function type inconsistency"),
                }
                mu.write_string(">".to_string(), stream)
            }
            _ => panic!("internal: function type inconsistency"),
        }
    }
}

pub trait MuFunction {
    fn mu_fn_prop(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Function {
    fn mu_fn_prop(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let prop_key = fp.argv[0];
        let func = fp.argv[1];

        match Tag::type_of(mu, func) {
            Type::Function => (),
            _ => return Err(Except::raise(mu, Condition::Type, "mu:fn-prop", func)),
        }

        fp.value = if prop_key.eq_(Symbol::keyword("lambda")) {
            let fnc = Self::func_of(mu, func);
            match Tag::type_of(mu, fnc) {
                Type::Cons => Cons::car(mu, fnc),
                _ => return Err(Except::raise(mu, Condition::Type, "mu:fn-prop", fnc)),
            }
        } else if prop_key.eq_(Symbol::keyword("frame")) {
            Self::frame_of(mu, func)
        } else if prop_key.eq_(Symbol::keyword("form")) {
            let fnc = Self::func_of(mu, func);
            match Tag::type_of(mu, fnc) {
                Type::Cons => Cons::cdr(mu, fnc),
                _ => func,
            }
        } else {
            return Err(Except::raise(mu, Condition::Type, "mu:fn-prop", prop_key));
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::classes::Tag;
    use crate::types::function::Function;

    #[test]
    fn as_tag() {
        match Function::new(Tag::nil(), Tag::nil(), Tag::nil()) {
            _ => assert_eq!(true, true),
        }
    }
}
