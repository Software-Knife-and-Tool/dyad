//
// symbol
//
use {
    crate::{
        classes::{
            indirect_vector::{TypedVec, VecType},
            namespace::{Namespace, Properties as _, Scope},
            vector::{Core as _, Vector},
        },
        core::{
            classes::DirectClass,
            classes::{Class, Tag, TagType, TagU64},
            exception,
            exception::{Condition, Except},
            frame::Frame,
            mu::{Core as _, Mu},
        },
        image,
    },
    std::{
        cell::{Ref, RefMut},
        str,
    },
};

pub enum Symbol {
    Keyword(Tag),
    Symbol(Image),
}

pub struct Image {
    namespace: Tag,
    scope: Tag,
    name: Tag,
    value: Tag,
}

lazy_static! {
    pub static ref UNBOUND: Tag = Tag::to_direct(0, 0, DirectClass::Keyword);
}

impl Symbol {
    pub fn new(mu: &Mu, namespace: Tag, scope: Scope, name: &str, value: Tag) -> Self {
        let str = name.as_bytes();
        let len = str.len();

        match str[0] as char {
            ':' => {
                if len > Tag::DIRECT_STR_MAX + 1 || len == 1 {
                    panic!("internal: keyword format inconsistency")
                }

                let str = name[1..].to_string();
                let mut data: [u8; 8] = 0u64.to_le_bytes();
                for (src, dst) in str.as_bytes().iter().zip(data.iter_mut()) {
                    *dst = *src
                }
                Symbol::Keyword(Tag::to_direct(
                    u64::from_le_bytes(data),
                    (len - 1) as u8,
                    DirectClass::Keyword,
                ))
            }
            _ => Symbol::Symbol(Image {
                namespace,
                scope: match scope {
                    Scope::Extern => Symbol::keyword("extern"),
                    Scope::Intern => Symbol::keyword("intern"),
                },
                name: Vector::from_string(name).evict(mu),
                value,
            }),
        }
    }

    pub fn as_image(mu: &Mu, tag: Tag) -> Image {
        let heap_ref: Ref<image::heap::Heap> = mu.heap.borrow();
        match Tag::class_of(mu, tag) {
            Class::Symbol => match tag {
                Tag::Indirect(main) => Image {
                    namespace: Tag::from_slice(
                        heap_ref.of_length(main.offset() as usize, 8).unwrap(),
                    ),
                    scope: Tag::from_slice(
                        heap_ref.of_length(main.offset() as usize + 8, 8).unwrap(),
                    ),
                    name: Tag::from_slice(
                        heap_ref.of_length(main.offset() as usize + 16, 8).unwrap(),
                    ),
                    value: Tag::from_slice(
                        heap_ref.of_length(main.offset() as usize + 24, 8).unwrap(),
                    ),
                },
                _ => panic!("internal: tag format inconsistency"),
            },
            _ => panic!("internal: symbol type required"),
        }
    }
}

pub trait Properties {
    fn namespace_of(_: &Mu, _: Tag) -> Tag;
    fn scope_of(_: &Mu, _: Tag) -> Tag;
    fn name_of(_: &Mu, _: Tag) -> Tag;
    fn value_of(_: &Mu, _: Tag) -> Tag;
}

impl Properties for Symbol {
    fn namespace_of(mu: &Mu, symbol: Tag) -> Tag {
        match Tag::class_of(mu, symbol) {
            Class::Keyword => Tag::nil(),
            Class::Symbol => match symbol {
                Tag::Indirect(_) => Self::as_image(mu, symbol).namespace,
                _ => panic!("internal: tag format inconsistency"),
            },
            _ => panic!("internal: symbol type required"),
        }
    }

    fn scope_of(mu: &Mu, symbol: Tag) -> Tag {
        match Tag::class_of(mu, symbol) {
            Class::Keyword => match symbol {
                Tag::Direct(_) => Symbol::keyword("extern"),
                _ => panic!("internal: tag format inconsistency"),
            },
            Class::Symbol => match symbol {
                Tag::Indirect(_) => Self::as_image(mu, symbol).scope,
                _ => panic!("internal: tag format inconsistency"),
            },
            _ => panic!("internal: symbol type required"),
        }
    }

    fn name_of(mu: &Mu, symbol: Tag) -> Tag {
        match Tag::class_of(mu, symbol) {
            Class::Keyword => match symbol {
                Tag::Direct(dir) => Tag::to_direct(dir.data(), dir.length(), DirectClass::Byte),
                _ => panic!("internal: tag format inconsistency"),
            },
            Class::Symbol => match symbol {
                Tag::Indirect(_) => Self::as_image(mu, symbol).name,
                _ => panic!("internal: tag format inconsistency"),
            },
            _ => panic!("internal: symbol type required"),
        }
    }

    fn value_of(mu: &Mu, symbol: Tag) -> Tag {
        match Tag::class_of(mu, symbol) {
            Class::Keyword => symbol,
            Class::Symbol => match symbol {
                Tag::Indirect(_) => Self::as_image(mu, symbol).value,
                _ => panic!("internal: symbol type inconsistency"),
            },
            _ => panic!("internal: symbol type required"),
        }
    }
}

pub trait Core {
    fn evict(&self, _: &Mu) -> Tag;
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn keyword(_: &str) -> Tag;
    fn is_unbound(_: &Mu, _: Tag) -> bool;
    fn view(_: &Mu, _: Tag) -> Tag;
}

impl Core for Symbol {
    fn view(mu: &Mu, symbol: Tag) -> Tag {
        let vec = TypedVec::<Vec<Tag>> {
            vec: vec![
                Self::namespace_of(mu, symbol),
                Self::scope_of(mu, symbol),
                Self::name_of(mu, symbol),
                if Self::is_unbound(mu, symbol) {
                    Symbol::keyword("UNBOUND")
                } else {
                    Self::value_of(mu, symbol)
                },
            ],
        };

        vec.vec.to_vector().evict(mu)
    }

    fn evict(&self, mu: &Mu) -> Tag {
        match self {
            Symbol::Keyword(tag) => *tag,
            Symbol::Symbol(image) => {
                let slices: &[[u8; 8]] = &[
                    image.namespace.as_slice(),
                    image.scope.as_slice(),
                    image.name.as_slice(),
                    image.value.as_slice(),
                ];

                let mut heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
                Tag::Indirect(
                    TagU64::new()
                        .with_offset(heap_ref.alloc(slices, Class::Symbol as u8) as u64)
                        .with_tag(TagType::Symbol),
                )
            }
        }
    }

    fn keyword(name: &str) -> Tag {
        let str = name.as_bytes();
        let len = str.len();

        if len > Tag::DIRECT_STR_MAX || len == 0 {
            panic!("internal: keyword format inconsistency")
        }

        let str = name.to_string();
        let mut data: [u8; 8] = 0u64.to_le_bytes();
        for (src, dst) in str.as_bytes().iter().zip(data.iter_mut()) {
            *dst = *src
        }
        Tag::to_direct(u64::from_le_bytes(data), len as u8, DirectClass::Keyword)
    }

    fn write(mu: &Mu, symbol: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        match Tag::class_of(mu, symbol) {
            Class::Null | Class::Keyword => match str::from_utf8(&symbol.data(mu).to_le_bytes()) {
                Ok(s) => mu.write_string(format!(":{s}"), stream),
                Err(_) => panic!("internal: symbol content inconsistency"),
            },
            Class::Symbol => {
                let name = Self::name_of(mu, symbol);

                if escape {
                    let ns = Self::namespace_of(mu, symbol);

                    if !ns.eq_(mu.nil_ns) {
                        match mu.write(Namespace::name_of(mu, ns), false, stream) {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }

                        let scope = Symbol::scope_of(mu, symbol);
                        if scope.eq_(Symbol::keyword("extern")) {
                            match mu.write_string(":".to_string(), stream) {
                                Ok(_) => (),
                                Err(e) => return Err(e),
                            }
                        } else if scope.eq_(Symbol::keyword("intern")) {
                            match mu.write_string("::".to_string(), stream) {
                                Ok(_) => (),
                                Err(e) => return Err(e),
                            }
                        } else {
                            panic!("internal: symbol scope type inconsistency")
                        }
                    }
                }
                mu.write(name, false, stream)
            }
            _ => panic!("internal: symbol type inconsistency"),
        }
    }

    fn is_unbound(mu: &Mu, symbol: Tag) -> bool {
        Self::value_of(mu, symbol).eq_(*UNBOUND)
    }
}

pub trait MuFunction {
    fn mu_name(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_value(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_boundp(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_symbol(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_keyword(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_keywordp(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Symbol {
    fn mu_name(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match Tag::class_of(mu, symbol) {
            Class::Keyword | Class::Symbol => Symbol::name_of(mu, symbol),
            _ => return Err(Except::raise(mu, Condition::Type, "mu:sy-name", symbol)),
        };

        Ok(())
    }

    fn mu_ns(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match Tag::class_of(mu, symbol) {
            Class::Symbol => Symbol::namespace_of(mu, symbol),
            Class::Keyword => Self::keyword("keyword"),
            _ => return Err(Except::raise(mu, Condition::Type, "mu:sy-ns", symbol)),
        };

        Ok(())
    }

    fn mu_value(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match Tag::class_of(mu, symbol) {
            Class::Symbol => {
                if Symbol::is_unbound(mu, symbol) {
                    return Err(Except::raise(mu, Condition::Type, "mu:sy-value", symbol));
                } else {
                    Symbol::value_of(mu, symbol)
                }
            }
            Class::Keyword => symbol,
            _ => return Err(Except::raise(mu, Condition::Type, "mu:sy-ns", symbol)),
        };

        Ok(())
    }

    fn mu_boundp(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        fp.value = match Tag::class_of(mu, symbol) {
            Class::Keyword => symbol,
            Class::Symbol => {
                if Self::is_unbound(mu, symbol) {
                    Tag::nil()
                } else {
                    symbol
                }
            }
            _ => return Err(Except::raise(mu, Condition::Type, "mu:unboundp", symbol)),
        };

        Ok(())
    }

    fn mu_keywordp(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        match Tag::class_of(mu, symbol) {
            Class::Keyword => {
                fp.value = symbol;
                Ok(())
            }
            Class::Symbol => {
                fp.value = Tag::nil();
                Ok(())
            }
            _ => Err(Except::raise(mu, Condition::Type, "mu:keyp", symbol)),
        }
    }

    fn mu_keyword(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        match Tag::class_of(mu, symbol) {
            Class::Keyword => {
                fp.value = symbol;
                Ok(())
            }
            Class::Vector => {
                let str = Vector::as_string(mu, symbol);
                fp.value = Self::keyword(&str);
                Ok(())
            }
            _ => Err(Except::raise(mu, Condition::Type, "mu:make-kw", symbol)),
        }
    }

    fn mu_symbol(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let symbol = fp.argv[0];

        match Tag::class_of(mu, symbol) {
            Class::Vector => {
                let str = Vector::as_string(mu, symbol);
                fp.value = Self::new(mu, mu.nil_ns, Scope::Extern, &str, *UNBOUND).evict(mu);
                Ok(())
            }
            _ => Err(Except::raise(mu, Condition::Type, "mu:symbol", symbol)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
