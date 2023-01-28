//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu function type
use {
    crate::{
        classes::{
            fixnum::Fixnum,
            indirect_vector::{TypedVec, VecType},
            vector::Core as _,
        },
        core::{
            classes::{Class, Tag, TagType, TagU64},
            exception,
            mu::{Core as _, Mu},
            namespaces::Core as _,
        },
        image,
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
            .with_offset(heap_ref.alloc(image, Class::Function as u8) as u64)
            .with_tag(TagType::Function);

        Tag::Indirect(ind)
    }

    pub fn from_tag(mu: &Mu, tag: Tag) -> Self {
        match Tag::class_of(mu, tag) {
            Class::Function => match tag {
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
        match Tag::class_of(mu, func) {
            Class::Function => match func {
                Tag::Indirect(_) => Self::from_tag(mu, func).nreq,
                _ => panic!("internal: function type inconsistency"),
            },
            _ => panic!("internal: function type inconsistency"),
        }
    }

    fn func_of(mu: &Mu, func: Tag) -> Tag {
        match Tag::class_of(mu, func) {
            Class::Function => match func {
                Tag::Indirect(_) => Self::from_tag(mu, func).func,
                _ => panic!("internal: function type inconsistency"),
            },
            _ => panic!("internal: function type inconsistency"),
        }
    }

    fn frame_of(mu: &Mu, func: Tag) -> Tag {
        match Tag::class_of(mu, func) {
            Class::Function => match func {
                Tag::Indirect(_) => Self::from_tag(mu, func).frame,
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
        match Tag::class_of(mu, func) {
            Class::Function => {
                match mu.write_string("#<function: ".to_string(), stream) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                let form = Function::func_of(mu, func);
                match Tag::class_of(mu, form) {
                    Class::Null | Class::Cons => match mu.write_string(
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
                    Class::Fixnum => {
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

#[cfg(test)]
mod tests {
    use crate::classes::function::Function;
    use crate::core::classes::Tag;

    #[test]
    fn as_tag() {
        match Function::new(Tag::nil(), Tag::nil(), Tag::nil()) {
            _ => assert_eq!(true, true),
        }
    }
}
