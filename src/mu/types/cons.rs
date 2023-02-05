//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu cons class

use {
    crate::{
        core::{
            classes::{DirectType, Type},
            classes::{Tag, TagType, TagU64},
            exception,
            exception::{Condition, Except},
            frame::Frame,
            mu::{Core as _, Mu},
            read::{Read, EOL},
        },
        image,
        types::{
            fixnum::Fixnum,
            indirect_vector::{TypedVec, VecType},
            symbol::{Properties as _, Symbol},
            vector::Core as _,
        },
    },
    std::cell::RefMut,
};

#[derive(Copy, Clone)]
pub struct Cons {
    car: Tag,
    cdr: Tag,
}

impl Cons {
    pub fn new(car: Tag, cdr: Tag) -> Self {
        Cons { car, cdr }
    }

    pub fn to_image(mu: &Mu, tag: Tag) -> Self {
        match Tag::type_of(mu, tag) {
            Type::Cons => match tag {
                Tag::Indirect(main) => {
                    let heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
                    Cons {
                        car: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize, 8).unwrap(),
                        ),
                        cdr: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 8, 8).unwrap(),
                        ),
                    }
                }
                _ => panic!("internal: tag format inconsistency"),
            },
            _ => panic!("internal: cons type required"),
        }
    }
}

/// properties
pub trait Properties {
    fn car(_: &Mu, _: Tag) -> Tag;
    fn cdr(_: &Mu, _: Tag) -> Tag;
}

impl Properties for Cons {
    fn car(mu: &Mu, cons: Tag) -> Tag {
        match Tag::type_of(mu, cons) {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Indirect(_) => Self::to_image(mu, cons).car,
                _ => panic!("internal: tag format inconsistency"),
            },
            _ => panic!("internal: cons tag required"),
        }
    }

    fn cdr(mu: &Mu, cons: Tag) -> Tag {
        match Tag::type_of(mu, cons) {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Indirect(_) => Self::to_image(mu, cons).cdr,
                _ => panic!("internal: tag format inconsistency"),
            },
            _ => panic!("internal: cons tag required"),
        }
    }
}

/// core operations
pub trait Core {
    fn evict(&self, _: &Mu) -> Tag;
    fn append(_: &Mu, _: Tag, _: Tag) -> Tag;
    fn length(_: &Mu, _: Tag) -> usize;
    fn list(_: &Mu, _: &[Tag]) -> Tag;
    fn nth(_: &Mu, _: usize, _: Tag) -> Option<Tag>;
    fn nthcdr(_: &Mu, _: usize, _: Tag) -> Option<Tag>;

    fn read(_: &Mu, _: Tag) -> exception::Result<Tag>;
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn view(_: &Mu, _: Tag) -> Tag;
}

impl Core for Cons {
    fn view(mu: &Mu, cons: Tag) -> Tag {
        let vec = TypedVec::<Vec<Tag>> {
            vec: vec![Self::car(mu, cons), Self::cdr(mu, cons)],
        };

        vec.vec.to_vector().evict(mu)
    }

    fn evict(&self, mu: &Mu) -> Tag {
        let image: &[[u8; 8]] = &[self.car.as_slice(), self.cdr.as_slice()];

        let mut heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
        let ind = TagU64::new()
            .with_offset(heap_ref.alloc(image, Type::Cons as u8) as u64)
            .with_tag(TagType::Cons);

        Tag::Indirect(ind)
    }

    fn read(mu: &Mu, stream: Tag) -> exception::Result<Tag> {
        let dot = Tag::to_direct('.' as u64, 1, DirectType::Byte);

        match <Mu as Read>::read(mu, stream, false, Tag::nil(), true) {
            Ok(car) => {
                if EOL.eq_(car) {
                    Ok(Tag::nil())
                } else {
                    match Tag::type_of(mu, car) {
                        Type::Symbol if dot.eq_(Symbol::name_of(mu, car)) => {
                            match <Mu as Read>::read(mu, stream, false, Tag::nil(), true) {
                                Ok(cdr) if EOL.eq_(cdr) => Ok(Tag::nil()),
                                Ok(cdr) => {
                                    match <Mu as Read>::read(mu, stream, false, Tag::nil(), true) {
                                        Ok(eol) if EOL.eq_(eol) => Ok(cdr),
                                        Ok(_) => {
                                            Err(Except::raise(mu, Condition::Eof, "mu:car", stream))
                                        }
                                        Err(e) => Err(e),
                                    }
                                }
                                Err(e) => Err(e),
                            }
                        }
                        _ => match Self::read(mu, stream) {
                            Ok(cdr) => Ok(Cons::new(car, cdr).evict(mu)),
                            Err(e) => Err(e),
                        },
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    fn write(mu: &Mu, cons: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        let car = Self::car(mu, cons);

        mu.write_string("(".to_string(), stream).unwrap();
        mu.write(car, escape, stream).unwrap();

        let mut tail = Self::cdr(mu, cons);

        // this is ugly, but it might be worse with a for loop
        loop {
            match Tag::type_of(mu, tail) {
                Type::Cons => {
                    mu.write_string(" ".to_string(), stream).unwrap();
                    mu.write(Self::car(mu, tail), escape, stream).unwrap();
                    tail = Self::cdr(mu, tail);
                }
                _ if tail.null_() => break,
                _ => {
                    mu.write_string(" . ".to_string(), stream).unwrap();
                    mu.write(tail, escape, stream).unwrap();
                    break;
                }
            }
        }

        mu.write_string(")".to_string(), stream)
    }

    fn append(mu: &Mu, cons0: Tag, cons1: Tag) -> Tag {
        match Tag::type_of(mu, cons0) {
            Type::Null => cons1,
            Type::Cons => Self::new(
                Self::car(mu, cons0),
                Self::append(mu, Cons::cdr(mu, cons0), cons1),
            )
            .evict(mu),
            _ => panic!("interal: cons type inconsistency"),
        }
    }

    fn length(mu: &Mu, cons: Tag) -> usize {
        match Tag::type_of(mu, cons) {
            Type::Null => 0,
            Type::Cons => ConsIter::new(mu, cons).count(),
            _ => panic!("interal: cons type inconsistency"),
        }
    }

    fn list(mu: &Mu, vec: &[Tag]) -> Tag {
        let mut list = Tag::nil();

        vec.iter()
            .for_each(|tag| list = Self::append(mu, list, Self::new(*tag, Tag::nil()).evict(mu)));

        list
    }

    fn nth(mu: &Mu, n: usize, cons: Tag) -> Option<Tag> {
        let mut nth = n;
        let mut tail = cons;

        match Tag::type_of(mu, cons) {
            Type::Null => Some(Tag::nil()),
            Type::Cons => loop {
                match Tag::type_of(mu, tail) {
                    _ if tail.null_() => return Some(Tag::nil()),
                    Type::Cons => {
                        if nth == 0 {
                            return Some(Self::car(mu, tail));
                        }
                        nth -= 1;
                        tail = Self::cdr(mu, tail)
                    }
                    _ => {
                        print!("nth: not on dotted lists");
                        return None;
                    }
                }
            },
            _ => panic!("internal: cons type required"),
        }
    }

    fn nthcdr(mu: &Mu, n: usize, cons: Tag) -> Option<Tag> {
        let mut nth = n;
        let mut tail = cons;

        match Tag::type_of(mu, cons) {
            Type::Null => Some(Tag::nil()),
            Type::Cons => loop {
                match Tag::type_of(mu, tail) {
                    _ if tail.null_() => return Some(Tag::nil()),
                    Type::Cons => {
                        if nth == 0 {
                            return Some(tail);
                        }
                        nth -= 1;
                        tail = Self::cdr(mu, tail)
                    }
                    _ => {
                        print!("nthcdr: not on dotted lists");
                        return None;
                    }
                }
            },
            _ => panic!("internal: cons type required"),
        }
    }
}

/// mu functions
pub trait MuFunction {
    fn mu_append(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_car(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_cdr(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_cons(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_length(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_nth(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_nthcdr(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Cons {
    fn mu_append(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Self::append(mu, fp.argv[0], fp.argv[1]);

        Ok(())
    }

    fn mu_car(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let list = fp.argv[0];

        fp.value = match Tag::type_of(mu, list) {
            Type::Null => list,
            Type::Cons => Self::car(mu, list),
            _ => return Err(Except::raise(mu, Condition::Type, "mu:car", list)),
        };

        Ok(())
    }

    fn mu_cdr(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let list = fp.argv[0];

        fp.value = match Tag::type_of(mu, list) {
            Type::Null => list,
            Type::Cons => Self::cdr(mu, list),
            _ => return Err(Except::raise(mu, Condition::Type, "mu:cdr", list)),
        };

        Ok(())
    }

    fn mu_cons(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Self::new(fp.argv[0], fp.argv[1]).evict(mu);
        Ok(())
    }

    fn mu_length(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let list = fp.argv[0];

        match Tag::type_of(mu, list) {
            Type::Null => fp.value = Fixnum::as_tag(0),
            Type::Cons => fp.value = Fixnum::as_tag(Cons::length(mu, list) as i64),
            _ => return Err(Except::raise(mu, Condition::Type, "mu:length", list)),
        }

        Ok(())
    }

    fn mu_nth(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        if Tag::type_of(mu, fp.argv[0]) != Type::Fixnum || Fixnum::as_i64(mu, fp.argv[0]) < 0 {
            return Err(Except::raise(mu, Condition::Type, "mu:nth", fp.argv[0]));
        }

        match Tag::type_of(mu, fp.argv[1]) {
            Type::Null => {
                fp.value = Tag::nil();
                Ok(())
            }
            Type::Cons => {
                fp.value =
                    Self::nth(mu, Fixnum::as_i64(mu, fp.argv[0]) as usize, fp.argv[1]).unwrap();
                Ok(())
            }
            _ => Err(Except::raise(mu, Condition::Type, "mu:nth", fp.argv[1])),
        }
    }

    fn mu_nthcdr(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        if Tag::type_of(mu, fp.argv[0]) != Type::Fixnum || Fixnum::as_i64(mu, fp.argv[0]) < 0 {
            return Err(Except::raise(mu, Condition::Type, "mu:nthcdr", fp.argv[0]));
        }

        match Tag::type_of(mu, fp.argv[1]) {
            Type::Null => {
                fp.value = Tag::nil();
                Ok(())
            }
            Type::Cons => {
                fp.value =
                    Self::nthcdr(mu, Fixnum::as_i64(mu, fp.argv[0]) as usize, fp.argv[1]).unwrap();
                Ok(())
            }
            _ => Err(Except::raise(mu, Condition::Type, "mu:nthcdr", fp.argv[1])),
        }
    }
}

/// iterator
pub struct ConsIter<'a> {
    mu: &'a Mu,
    pub cons: Tag,
}

impl<'a> ConsIter<'a> {
    pub fn new(mu: &'a Mu, cons: Tag) -> Self {
        Self { mu, cons }
    }
}

impl<'a> Iterator for ConsIter<'a> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cons.eq_(Tag::nil()) {
            None
        } else {
            let cons = self.cons;
            self.cons = Cons::cdr(self.mu, self.cons);
            Some(cons)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::classes::Tag;
    use crate::types::cons::Cons;

    #[test]
    fn cons() {
        match Cons::new(Tag::nil(), Tag::nil()) {
            _ => assert_eq!(true, true),
        }
    }
}
