//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu struct type
use {
    crate::{
        core::{
            classes::{Tag, TagType, TagU64, Type},
            exception,
            exception::{Condition, Except},
            frame::Frame,
            mu::{Core as _, Mu},
        },
        image,
        types::{
            cons::{Cons, ConsIter, Core as _, Properties as _},
            ivector::{TypedVec, VecType, VectorIter},
            stream::{Core as _, Stream},
            symbol::{Core as _, Symbol},
            vector::Core as _,
        },
    },
    std::cell::{Ref, RefMut},
};

// a struct is a vector with an arbitrary type
pub struct Struct {
    stype: Tag,
    vector: Tag,
}

impl Struct {
    pub fn to_image(mu: &Mu, tag: Tag) -> Self {
        match Tag::type_of(mu, tag) {
            Type::Struct => match tag {
                Tag::Indirect(image) => {
                    let heap_ref: Ref<image::heap::Heap> = mu.heap.borrow();
                    Struct {
                        stype: Tag::from_slice(
                            heap_ref.of_length(image.offset() as usize, 8).unwrap(),
                        ),
                        vector: Tag::from_slice(
                            heap_ref.of_length(image.offset() as usize + 8, 8).unwrap(),
                        ),
                    }
                }
                _ => panic!("internal: tag format consistency"),
            },
            _ => panic!("internal: struct type required"),
        }
    }

    pub fn to_tag(mu: &Mu, stype: Tag, vec: Vec<Tag>) -> Tag {
        match Tag::type_of(mu, stype) {
            Type::Keyword => {
                let vector = TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu);
                Struct { stype, vector }.evict(mu)
            }
            _ => panic!("internal: struct type inconsistency"),
        }
    }
}

// core
pub trait Core<'a> {
    fn read(_: &Mu, _: Tag) -> exception::Result<Tag>;
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;

    fn evict(&self, _: &Mu) -> Tag;
    fn view(_: &Mu, _: Tag) -> Tag;
}

impl<'a> Core<'a> for Struct {
    fn view(mu: &Mu, tag: Tag) -> Tag {
        let image = Self::to_image(mu, tag);

        Self::to_tag(
            mu,
            Symbol::keyword("struct"),
            vec![image.stype, image.vector],
        )
    }

    fn write(mu: &Mu, tag: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        match tag {
            Tag::Indirect(_) => {
                match mu.write_string("#S(".to_string(), stream) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                match mu.write(Self::to_image(mu, tag).stype, true, stream) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }

                for tag in VectorIter::new(mu, Self::to_image(mu, tag).vector) {
                    match mu.write_string(" ".to_string(), stream) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }

                    match mu.write(tag, false, stream) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }

                mu.write_string(")".to_string(), stream)
            }
            _ => panic!("internal: struct type inconsistency"),
        }
    }

    fn read(mu: &Mu, stream: Tag) -> exception::Result<Tag> {
        match Stream::read_char(mu, stream) {
            Ok(Some(ch)) => match ch {
                '(' => {
                    let vec_list = match Cons::read(mu, stream) {
                        Ok(list) => {
                            if list.null_() {
                                return Err(Except::raise(
                                    mu,
                                    Condition::Type,
                                    "struct::read",
                                    Tag::nil(),
                                ));
                            }
                            list
                        }
                        Err(_) => {
                            return Err(Except::raise(
                                mu,
                                Condition::Syntax,
                                "struct::read",
                                stream,
                            ));
                        }
                    };

                    let stype = Cons::car(mu, vec_list);
                    match Tag::type_of(mu, stype) {
                        Type::Keyword => {
                            let mut vec = Vec::new();
                            for cons in ConsIter::new(mu, Cons::cdr(mu, vec_list)) {
                                vec.push(Cons::car(mu, cons));
                            }

                            Ok(Self::to_tag(mu, stype, vec))
                        }
                        _ => Err(Except::raise(mu, Condition::Type, "struct::read", stype)),
                    }
                }
                _ => Err(Except::raise(mu, Condition::Eof, "struct::read", stream)),
            },
            Ok(None) => Err(Except::raise(mu, Condition::Eof, "struct::read", stream)),
            Err(e) => Err(e),
        }
    }

    fn evict(&self, mu: &Mu) -> Tag {
        let image: &[[u8; 8]] = &[self.stype.as_slice(), self.vector.as_slice()];

        let mut heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
        Tag::Indirect(
            TagU64::new()
                .with_offset(heap_ref.alloc(image, Type::Struct as u8) as u64)
                .with_tag(TagType::Heap),
        )
    }
}

// mu functions
pub trait MuFunction {
    fn mu_struct_type(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_struct_vector(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_make_struct(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Struct {
    fn mu_struct_type(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        match Tag::type_of(mu, tag) {
            Type::Struct => {
                let image = Self::to_image(mu, tag);

                fp.value = image.stype;
                Ok(())
            }
            _ => Err(Except::raise(mu, Condition::Type, "mu:st-type", tag)),
        }
    }

    fn mu_struct_vector(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let tag = fp.argv[0];

        match Tag::type_of(mu, tag) {
            Type::Struct => {
                let image = Self::to_image(mu, tag);

                fp.value = image.vector;
                Ok(())
            }
            _ => Err(Except::raise(mu, Condition::Type, "mu:st-vec", tag)),
        }
    }

    fn mu_make_struct(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stype = fp.argv[0];
        let list = fp.argv[1];

        fp.value = match Tag::type_of(mu, stype) {
            Type::Keyword => {
                let mut vec = Vec::new();
                for cons in ConsIter::new(mu, list) {
                    vec.push(Cons::car(mu, cons));
                }

                let vector = TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu);

                Struct { stype, vector }.evict(mu)
            }
            _ => {
                return Err(Except::raise(mu, Condition::Type, "mu:struct", stype));
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
