//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu float type
use crate::{
    core::{
        classes::DirectType,
        classes::{Tag, Type},
        exception,
        exception::{Condition, Exception},
        frame::Frame,
        mu::{Core as _, Mu},
    },
    types::{
        r#struct::Struct,
        symbol::{Core as _, Symbol},
    },
};

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Float {
    Direct(u64),
}

impl Float {
    pub fn as_tag(fl: f32) -> Tag {
        let bytes = fl.to_le_bytes();
        Tag::to_direct(u32::from_le_bytes(bytes) as u64, 0, DirectType::Float)
    }

    pub fn as_f32(mu: &Mu, tag: Tag) -> f32 {
        match Tag::type_of(mu, tag) {
            Type::Float => {
                let data = tag.data(mu).to_le_bytes();
                let mut fl = 0.0f32.to_le_bytes();

                for (dst, src) in fl.iter_mut().zip(data.iter()) {
                    *dst = *src
                }
                f32::from_le_bytes(fl)
            }
            _ => panic!("internal: float type inconsistency"),
        }
    }
}

pub trait Core {
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn view(_: &Mu, _: Tag) -> Tag;
}

impl Core for Float {
    fn view(mu: &Mu, fl: Tag) -> Tag {
        Struct::to_tag(mu, Symbol::keyword("float"), vec![fl])
    }

    fn write(mu: &Mu, tag: Tag, _escape: bool, stream: Tag) -> exception::Result<()> {
        mu.write_string(format!("{:.4}", Self::as_f32(mu, tag)), stream)
    }
}

pub trait MuFunction {
    fn mu_fladd(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_flsub(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_flmul(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_fllt(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_fldiv(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Float {
    fn mu_fladd(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = Tag::from_u64(fp.argv[0]);
        let fl1 = Tag::from_u64(fp.argv[1]);

        match Tag::type_of(mu, fl0) {
            Type::Float => match Tag::type_of(mu, fl1) {
                Type::Float => {
                    fp.value = Self::as_tag(Self::as_f32(mu, fl0) + Self::as_f32(mu, fl1));
                    Ok(())
                }
                _ => Err(Exception::raise(mu, Condition::Type, "mu:fl-add", fl1)),
            },
            _ => Err(Exception::raise(mu, Condition::Type, "mu:fl-add", fl0)),
        }
    }

    fn mu_flsub(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = Tag::from_u64(fp.argv[0]);
        let fl1 = Tag::from_u64(fp.argv[1]);

        match Tag::type_of(mu, fl0) {
            Type::Float => match Tag::type_of(mu, fl1) {
                Type::Float => {
                    fp.value = Self::as_tag(Self::as_f32(mu, fl0) - Self::as_f32(mu, fl1));
                    Ok(())
                }
                _ => Err(Exception::raise(mu, Condition::Type, "mu:fl-sub", fl1)),
            },
            _ => Err(Exception::raise(mu, Condition::Type, "mu:fl-sub", fl0)),
        }
    }

    fn mu_flmul(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = Tag::from_u64(fp.argv[0]);
        let fl1 = Tag::from_u64(fp.argv[1]);

        match Tag::type_of(mu, fl0) {
            Type::Float => match Tag::type_of(mu, fl1) {
                Type::Float => {
                    fp.value = Self::as_tag(Self::as_f32(mu, fl0) * Self::as_f32(mu, fl1));
                    Ok(())
                }
                _ => Err(Exception::raise(mu, Condition::Type, "mu:fl-mul", fl1)),
            },
            _ => Err(Exception::raise(mu, Condition::Type, "mu:fl-mul", fl0)),
        }
    }

    fn mu_fllt(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = Tag::from_u64(fp.argv[0]);
        let fl1 = Tag::from_u64(fp.argv[1]);

        match Tag::type_of(mu, fl0) {
            Type::Float => match Tag::type_of(mu, fl1) {
                Type::Float => {
                    fp.value = if Self::as_f32(mu, fl0) < Self::as_f32(mu, fl1) {
                        Tag::t()
                    } else {
                        Tag::nil()
                    };

                    Ok(())
                }
                _ => Err(Exception::raise(mu, Condition::Type, "mu:fl-lt", fl1)),
            },
            _ => Err(Exception::raise(mu, Condition::Type, "mu:fl-lt", fl0)),
        }
    }

    fn mu_fldiv(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = Tag::from_u64(fp.argv[0]);
        let fl1 = Tag::from_u64(fp.argv[1]);

        match Tag::type_of(mu, fl0) {
            Type::Float => match Tag::type_of(mu, fl1) {
                Type::Float => {
                    fp.value = Self::as_tag(Self::as_f32(mu, fl0) / Self::as_f32(mu, fl1));
                    Ok(())
                }
                _ => Err(Exception::raise(mu, Condition::Type, "mu:fl-div", fl1)),
            },
            _ => Err(Exception::raise(mu, Condition::Type, "mu:fl-div", fl0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::float::Float;

    #[test]
    fn as_tag() {
        match Float::as_tag(1.0) {
            _ => assert_eq!(true, true),
        }
    }
}
