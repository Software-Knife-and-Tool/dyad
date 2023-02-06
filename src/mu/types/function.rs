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
    lambda: Tag, // lambda list
    nreq: Tag,   // fixnum # of required arguments
    form: Tag,   // cons body or fixnum native table offset
    frame: Tag,  // frame id
}

impl Function {
    pub fn new(lambda: Tag, nreq: Tag, form: Tag, frame: Tag) -> Self {
        Function {
            lambda,
            nreq,
            form,
            frame,
        }
    }

    pub fn evict(&self, mu: &Mu) -> Tag {
        let image: &[[u8; 8]] = &[
            self.lambda.as_slice(),
            self.nreq.as_slice(),
            self.form.as_slice(),
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
                        lambda: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize, 8).unwrap(),
                        ),
                        nreq: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 8, 8).unwrap(),
                        ),
                        form: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 16, 8).unwrap(),
                        ),
                        frame: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 24, 8).unwrap(),
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
    fn form_of(_: &Mu, _: Tag) -> Tag;
    fn frame_of(_: &Mu, _: Tag) -> Tag;
    fn lambda_of(_: &Mu, _: Tag) -> Tag;
    fn nreq_of(_: &Mu, _: Tag) -> Tag;
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

    fn lambda_of(mu: &Mu, func: Tag) -> Tag {
        match Tag::type_of(mu, func) {
            Type::Function => match func {
                Tag::Indirect(_) => Self::to_image(mu, func).lambda,
                _ => panic!("internal: function type inconsistency"),
            },
            _ => panic!("internal: function type inconsistency"),
        }
    }

    fn form_of(mu: &Mu, func: Tag) -> Tag {
        match Tag::type_of(mu, func) {
            Type::Function => match func {
                Tag::Indirect(_) => Self::to_image(mu, func).form,
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
                Self::lambda_of(mu, func),
                Self::nreq_of(mu, func),
                Self::form_of(mu, func),
                Self::frame_of(mu, func),
            ],
        };

        vec.vec.to_vector().evict(mu)
    }

    fn write(mu: &Mu, func: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        match Tag::type_of(mu, func) {
            Type::Function => {
                let nreq = Fixnum::as_i64(mu, Function::nreq_of(mu, func));
                let form = Function::form_of(mu, func);

                let desc = match Tag::type_of(mu, form) {
                    Type::Cons => ("lambda".to_string(), form.as_u64().to_string()),
                    Type::Fixnum => {
                        let (name, _, _, _) = Mu::map_core(Fixnum::as_i64(mu, form) as usize);
                        ("native".to_string(), name.to_string())
                    }
                    _ => panic!("internal: function type inconsistency"),
                };

                mu.write_string(
                    format!("#<:function :{} [req:{nreq}, tag:{}]>", desc.0, desc.1),
                    stream,
                )
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
            Self::lambda_of(mu, func)
        } else if prop_key.eq_(Symbol::keyword("nreq")) {
            Self::nreq_of(mu, func)
        } else if prop_key.eq_(Symbol::keyword("form")) {
            Self::form_of(mu, func)
        } else if prop_key.eq_(Symbol::keyword("frame")) {
            Self::frame_of(mu, func)
        } else {
            return Err(Except::raise(mu, Condition::Type, "mu:fn-prop", prop_key));
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::classes::Tag;
    use crate::types::fixnum::Fixnum;
    use crate::types::function::Function;

    #[test]
    fn as_tag() {
        match Function::new(Tag::nil(), Fixnum::as_tag(0), Tag::nil(), Tag::nil()) {
            _ => assert_eq!(true, true),
        }
    }
}
