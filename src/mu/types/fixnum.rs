//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu fixnum type
use crate::{
    core::{
        classes::{Tag, Type},
        exception,
        exception::{Condition, Except, Result},
        frame::Frame,
        mu::{Core as _, Mu},
    },
    types::{
        ivector::{TypedVec, VecType},
        vector::Core as _,
    },
};

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Fixnum {
    Direct(u64),
}

impl Fixnum {
    // u64 to tag
    pub fn as_tag(fx: i64) -> Tag {
        // we're implicitly or'ing in the fixnum base tag type 0 here
        Tag::Fixnum(fx << 2)
    }

    // tag as i64
    pub fn as_i64(mu: &Mu, tag: Tag) -> i64 {
        match Tag::type_of(mu, tag) {
            Type::Fixnum => Tag::data(&tag, mu) as i64,
            _ => panic!("internal: fixnum type inconsistency"),
        }
    }
}

pub trait Core {
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> Result<()>;
    fn view(_: &Mu, _: Tag) -> Tag;
}

impl Core for Fixnum {
    fn write(mu: &Mu, tag: Tag, _escape: bool, stream: Tag) -> Result<()> {
        mu.write_string(Self::as_i64(mu, tag).to_string(), stream)
    }

    fn view(mu: &Mu, fx: Tag) -> Tag {
        let vec = TypedVec::<Vec<Tag>> { vec: vec![fx] };

        vec.vec.to_vector().evict(mu)
    }
}

pub trait MuFunction {
    fn mu_fxadd(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_fxsub(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_fxor(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_fxand(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_fxdiv(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_fxlt(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_fxmul(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Fixnum {
    fn mu_fxadd(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        match Tag::type_of(mu, fp.argv[0]) {
            Type::Fixnum => match Tag::type_of(mu, fp.argv[1]) {
                Type::Fixnum => {
                    fp.value = Self::as_tag(
                        Fixnum::as_i64(mu, fp.argv[0]) + Fixnum::as_i64(mu, fp.argv[1]),
                    );
                    Ok(())
                }
                _ => Err(Except::raise(mu, Condition::Type, "mu:fx-add", fp.argv[1])),
            },
            _ => Err(Except::raise(mu, Condition::Type, "mu:fx-add", fp.argv[0])),
        }
    }

    fn mu_fxsub(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        match Tag::type_of(mu, fp.argv[0]) {
            Type::Fixnum => match Tag::type_of(mu, fp.argv[1]) {
                Type::Fixnum => {
                    fp.value = Self::as_tag(
                        Fixnum::as_i64(mu, fp.argv[0]) - Fixnum::as_i64(mu, fp.argv[1]),
                    );
                    Ok(())
                }
                _ => Err(Except::raise(mu, Condition::Type, "mu:fx-sub", fp.argv[1])),
            },
            _ => Err(Except::raise(mu, Condition::Type, "mu:fx-sub", fp.argv[0])),
        }
    }

    fn mu_fxmul(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        match Tag::type_of(mu, fp.argv[0]) {
            Type::Fixnum => match Tag::type_of(mu, fp.argv[1]) {
                Type::Fixnum => {
                    fp.value = Self::as_tag(
                        Fixnum::as_i64(mu, fp.argv[0]) * Fixnum::as_i64(mu, fp.argv[1]),
                    );
                    Ok(())
                }
                _ => Err(Except::raise(mu, Condition::Type, "mu:fx-mul", fp.argv[1])),
            },
            _ => Err(Except::raise(mu, Condition::Type, "mu:fx-mul", fp.argv[0])),
        }
    }

    fn mu_fxlt(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        match Tag::type_of(mu, fp.argv[0]) {
            Type::Fixnum => match Tag::type_of(mu, fp.argv[1]) {
                Type::Fixnum => {
                    fp.value = if Fixnum::as_i64(mu, fp.argv[0]) < Fixnum::as_i64(mu, fp.argv[1]) {
                        Tag::t()
                    } else {
                        Tag::nil()
                    };

                    Ok(())
                }
                _ => Err(Except::raise(mu, Condition::Type, "mu:fx-lt", fp.argv[1])),
            },
            _ => Err(Except::raise(mu, Condition::Type, "mu:fx-lt", fp.argv[0])),
        }
    }

    fn mu_fxdiv(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        match Tag::type_of(mu, fp.argv[0]) {
            Type::Fixnum => match Tag::type_of(mu, fp.argv[1]) {
                Type::Fixnum => {
                    let dividend = Fixnum::as_i64(mu, fp.argv[0]);
                    let divisor = Fixnum::as_i64(mu, fp.argv[1]);

                    if divisor == 0 {
                        return Err(Except::raise(
                            mu,
                            Condition::ZeroDivide,
                            "mu:fx-div",
                            fp.argv[0],
                        ));
                    }

                    fp.value = Self::as_tag(dividend / divisor);
                    Ok(())
                }
                _ => Err(Except::raise(mu, Condition::Type, "mu:fx-div", fp.argv[1])),
            },
            _ => Err(Except::raise(mu, Condition::Type, "mu:fx-div", fp.argv[0])),
        }
    }

    fn mu_fxand(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        match Tag::type_of(mu, fp.argv[0]) {
            Type::Fixnum => match Tag::type_of(mu, fp.argv[1]) {
                Type::Fixnum => {
                    fp.value = Self::as_tag(
                        Fixnum::as_i64(mu, fp.argv[0]) & Fixnum::as_i64(mu, fp.argv[1]),
                    );
                    Ok(())
                }
                _ => Err(Except::raise(mu, Condition::Type, "mu:logand", fp.argv[1])),
            },
            _ => Err(Except::raise(mu, Condition::Type, "mu:logand", fp.argv[0])),
        }
    }

    fn mu_fxor(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        match Tag::type_of(mu, fp.argv[0]) {
            Type::Fixnum => match Tag::type_of(mu, fp.argv[1]) {
                Type::Fixnum => {
                    fp.value = Self::as_tag(
                        Fixnum::as_i64(mu, fp.argv[0]) | Fixnum::as_i64(mu, fp.argv[1]),
                    );
                    Ok(())
                }
                _ => Err(Except::raise(mu, Condition::Type, "mu:logor", fp.argv[1])),
            },
            _ => Err(Except::raise(mu, Condition::Type, "mu:logor", fp.argv[0])),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::fixnum::Fixnum;

    #[test]
    fn as_tag() {
        match Fixnum::as_tag(0) {
            _ => assert_eq!(true, true),
        }
    }
}
