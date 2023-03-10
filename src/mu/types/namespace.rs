//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu namespace type
use {
    crate::{
        core::{
            classes::{Tag, TagIndirect, TagType, Type},
            exception,
            exception::{Condition, Exception},
            frame::Frame,
            mu::{Core as _, Mu},
        },
        image,
        types::{
            cons::{Cons, Core as _},
            r#struct::Struct,
            symbol::{Core as _, Symbol},
            vector::{Core as _, Vector},
        },
    },
    std::{
        cell::{Ref, RefCell, RefMut},
        collections::HashMap,
        str,
    },
};

#[derive(Copy, Clone, Debug)]
pub enum Scope {
    Intern,
    Extern,
}

pub struct Namespace {
    name: Tag, // string
    #[allow(dead_code)]
    externs: Tag, // list of symbols
    interns: Tag, // list of symbols
    import: Tag, // import namespace
}

impl Namespace {
    pub fn new(mu: &Mu, name: &str, import: Tag) -> Self {
        match Tag::type_of(mu, import) {
            Type::Null | Type::Namespace => Namespace {
                name: Vector::from_string(name).evict(mu),
                externs: Tag::nil(),
                interns: Tag::nil(),
                import,
            },
            _ => panic!(),
        }
    }

    pub fn evict(&self, mu: &Mu) -> Tag {
        let image: &[[u8; 8]] = &[
            self.name.as_slice(),
            self.externs.as_slice(),
            self.interns.as_slice(),
            self.import.as_slice(),
        ];

        let mut heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
        Tag::Indirect(
            TagIndirect::new()
                .with_offset(heap_ref.alloc(image, Type::Namespace as u8) as u64)
                .with_tag(TagType::Heap),
        )
    }

    pub fn to_image(mu: &Mu, tag: Tag) -> Self {
        match Tag::type_of(mu, tag) {
            Type::Namespace => match tag {
                Tag::Indirect(main) => {
                    let heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
                    Namespace {
                        name: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize, 8).unwrap(),
                        ),
                        externs: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 8, 8).unwrap(),
                        ),
                        interns: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 16, 8).unwrap(),
                        ),
                        import: Tag::from_slice(
                            heap_ref.of_length(main.offset() as usize + 24, 8).unwrap(),
                        ),
                    }
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn add_ns(mu: &Mu, ns: Tag) -> exception::Result<Tag> {
        #[allow(clippy::type_complexity)]
        let mut ns_ref: RefMut<
            HashMap<
                String,
                (
                    Tag,
                    (RefCell<HashMap<String, Tag>>, RefCell<HashMap<String, Tag>>),
                ),
            >,
        > = mu.ns_caches.borrow_mut();

        let ns_name = Vector::as_string(mu, Namespace::name_of(mu, ns));

        if ns_ref.contains_key(&ns_name) {
            return Err(Exception::new(Condition::Type, "add-ns", ns));
        }

        ns_ref.insert(
            ns_name,
            (
                ns,
                (
                    RefCell::new(HashMap::<String, Tag>::new()),
                    RefCell::new(HashMap::<String, Tag>::new()),
                ),
            ),
        );

        Ok(ns)
    }

    pub fn map_ns(mu: &Mu, ns_name: String) -> Option<Tag> {
        #[allow(clippy::type_complexity)]
        let ns_ref: Ref<
            HashMap<
                String,
                (
                    Tag,
                    (RefCell<HashMap<String, Tag>>, RefCell<HashMap<String, Tag>>),
                ),
            >,
        > = mu.ns_caches.borrow();

        if !ns_ref.contains_key(&ns_name) {
            return None;
        }

        let (ns, _) = ns_ref[&ns_name];

        Some(ns)
    }

    pub fn name_of(mu: &Mu, ns: Tag) -> Tag {
        match Tag::type_of(mu, ns) {
            Type::Namespace => match ns {
                Tag::Indirect(_) => Self::to_image(mu, ns).name,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn externs_of(mu: &Mu, ns: Tag) -> Tag {
        match Tag::type_of(mu, ns) {
            Type::Namespace => match ns {
                Tag::Indirect(_) => Self::to_image(mu, ns).externs,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn interns_of(mu: &Mu, ns: Tag) -> Tag {
        match Tag::type_of(mu, ns) {
            Type::Namespace => match ns {
                Tag::Indirect(_) => Self::to_image(mu, ns).interns,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn import_of(mu: &Mu, ns: Tag) -> Tag {
        match Tag::type_of(mu, ns) {
            Type::Namespace => match ns {
                Tag::Indirect(_) => Self::to_image(mu, ns).import,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

pub trait Core {
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn intern(_: &Mu, _: Tag, _: Scope, _: String, _: Tag) -> Tag;
    fn view(_: &Mu, _: Tag) -> Tag;
}

impl Core for Namespace {
    fn view(mu: &Mu, ns: Tag) -> Tag {
        Struct::to_tag(
            mu,
            Symbol::keyword("ns"),
            vec![Self::name_of(mu, ns), Self::import_of(mu, ns)],
        )
    }

    fn write(mu: &Mu, ns: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        match Tag::type_of(mu, ns) {
            Type::Namespace => {
                let name = Self::name_of(mu, ns);
                match mu.write_string("#<namespace: ".to_string(), stream) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                match mu.write(name, true, stream) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                mu.write_string(">".to_string(), stream)
            }
            _ => panic!(),
        }
    }

    fn intern(mu: &Mu, ns: Tag, scope: Scope, name: String, value: Tag) -> Tag {
        match Tag::type_of(mu, ns) {
            Type::Namespace => match ns {
                Tag::Indirect(_) => {
                    #[allow(clippy::type_complexity)]
                    let ns_ref: RefMut<
                        HashMap<
                            String,
                            (
                                Tag,
                                (RefCell<HashMap<String, Tag>>, RefCell<HashMap<String, Tag>>),
                            ),
                        >,
                    > = mu.ns_caches.borrow_mut();

                    let ns_name = Vector::as_string(mu, Namespace::name_of(mu, ns));

                    if !ns_ref.contains_key(&ns_name) {
                        panic!();
                    }

                    let (_, (ext, int)) = &ns_ref[&ns_name];
                    let mut hash = match scope {
                        Scope::Intern => int.borrow_mut(),
                        Scope::Extern => ext.borrow_mut(),
                    };

                    if hash.contains_key(&name) {
                        let symbol = *hash.get(&name).unwrap();

                        if Symbol::is_unbound(mu, symbol) {
                            let image = Symbol::to_image(mu, symbol);

                            let slices: &[[u8; 8]] = &[
                                image.namespace.as_slice(),
                                image.scope.as_slice(),
                                image.name.as_slice(),
                                value.as_slice(),
                            ];

                            let offset = match symbol {
                                Tag::Indirect(heap) => heap.offset(),
                                _ => panic!(),
                            } as usize;

                            let mut heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
                            heap_ref.write_image(slices, offset);
                        }

                        return symbol;
                    }

                    let symbol = Symbol::new(mu, ns, scope, &name, value).evict(mu);
                    hash.insert(name, symbol);

                    let mut image = Self::to_image(mu, ns);
                    match scope {
                        Scope::Intern => image.interns = Cons::new(symbol, image.interns).evict(mu),
                        Scope::Extern => image.externs = Cons::new(symbol, image.externs).evict(mu),
                    };

                    let slices: &[[u8; 8]] = &[
                        image.name.as_slice(),
                        image.externs.as_slice(),
                        image.interns.as_slice(),
                        image.import.as_slice(),
                    ];

                    let offset = match ns {
                        Tag::Indirect(heap) => heap.offset(),
                        _ => panic!(),
                    } as usize;

                    let mut heap_ref: RefMut<image::heap::Heap> = mu.heap.borrow_mut();
                    heap_ref.write_image(slices, offset);

                    symbol
                }
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

pub trait MuFunction {
    fn mu_intern(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_make_ns(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_map_ns(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns_find(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns_import(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns_name(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns_interns(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn mu_ns_externs(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Namespace {
    fn mu_intern(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];
        let scope = fp.argv[1];
        let name = fp.argv[2];
        let value = fp.argv[3];

        let scope_type = match Tag::type_of(mu, scope) {
            Type::Keyword => {
                if scope.eq_(Symbol::keyword("extern")) {
                    Scope::Extern
                } else if scope.eq_(Symbol::keyword("intern")) {
                    Scope::Intern
                } else {
                    return Err(Exception::new(Condition::Type, "mu:intern", scope));
                }
            }
            _ => return Err(Exception::new(Condition::Type, "mu:intern", scope)),
        };

        match Tag::type_of(mu, ns) {
            Type::Namespace => match Tag::type_of(mu, name) {
                Type::Vector => {
                    fp.value =
                        Namespace::intern(mu, ns, scope_type, Vector::as_string(mu, name), value);
                    Ok(())
                }
                _ => Err(Exception::new(Condition::Type, "mu:intern", name)),
            },
            _ => Err(Exception::new(Condition::Type, "mu:intern", ns)),
        }
    }

    fn mu_make_ns(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let name = fp.argv[0];
        let import = fp.argv[1];

        match Tag::type_of(mu, name) {
            Type::Vector => match Tag::type_of(mu, import) {
                Type::Null | Type::Namespace => {
                    fp.value = Self::new(mu, &Vector::as_string(mu, name), import).evict(mu);
                    Self::add_ns(mu, fp.value).unwrap();
                    Ok(())
                }
                _ => Err(Exception::new(Condition::Type, "mu:make_ns", import)),
            },
            _ => Err(Exception::new(Condition::Type, "mu:make_ns", name)),
        }
    }

    fn mu_map_ns(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let ns_name = fp.argv[0];

        match Tag::type_of(mu, ns_name) {
            Type::Vector => match Self::map_ns(mu, Vector::as_string(mu, ns_name)) {
                Some(ns) => {
                    fp.value = ns;
                    Ok(())
                }
                None => Err(Exception::new(Condition::Unbound, "mu:map_ns", ns_name)),
            },
            _ => Err(Exception::new(Condition::Type, "mu:map_ns", ns_name)),
        }
    }

    fn mu_ns_find(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];
        let scope = fp.argv[1];
        let name = fp.argv[2];

        let is_extern = match Tag::type_of(mu, scope) {
            Type::Keyword => {
                if scope.eq_(Symbol::keyword("extern")) {
                    true
                } else if scope.eq_(Symbol::keyword("intern")) {
                    false
                } else {
                    return Err(Exception::new(Condition::Type, "mu:ns-find", scope));
                }
            }
            _ => return Err(Exception::new(Condition::Type, "mu:ns-find", scope)),
        };

        match Tag::type_of(mu, name) {
            Type::Vector => match Tag::type_of(mu, ns) {
                Type::Namespace => {
                    #[allow(clippy::type_complexity)]
                    let ns_ref: RefMut<
                        HashMap<
                            String,
                            (
                                Tag,
                                (RefCell<HashMap<String, Tag>>, RefCell<HashMap<String, Tag>>),
                            ),
                        >,
                    > = mu.ns_caches.borrow_mut();

                    let ns_name = Vector::as_string(mu, Namespace::name_of(mu, ns));
                    let sy_name = Vector::as_string(mu, name);

                    if !ns_ref.contains_key(&ns_name) {
                        panic!();
                    }

                    let (_, (ext, int)) = &ns_ref[&ns_name];
                    let hash = if is_extern {
                        ext.borrow()
                    } else {
                        int.borrow()
                    };

                    fp.value = Tag::nil();
                    if hash.contains_key(&sy_name) {
                        fp.value = hash[&sy_name];
                    }

                    Ok(())
                }
                _ => Err(Exception::new(Condition::Type, "mu:ns-find", ns)),
            },
            _ => Err(Exception::new(Condition::Type, "mu:ns-find", name)),
        }
    }

    fn mu_ns_import(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];

        match Tag::type_of(mu, ns) {
            Type::Namespace => {
                fp.value = Namespace::import_of(mu, ns);
                Ok(())
            }
            _ => Err(Exception::new(Condition::Type, "mu:ns-ump", ns)),
        }
    }

    fn mu_ns_name(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];

        match Tag::type_of(mu, ns) {
            Type::Namespace => {
                fp.value = Namespace::name_of(mu, ns);
                Ok(())
            }
            _ => Err(Exception::new(Condition::Type, "mu:ns-name", ns)),
        }
    }

    fn mu_ns_externs(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];

        match Tag::type_of(mu, ns) {
            Type::Namespace => {
                fp.value = Namespace::externs_of(mu, ns);
                Ok(())
            }
            _ => Err(Exception::new(Condition::Type, "mu:ns-ext", ns)),
        }
    }

    fn mu_ns_interns(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let ns = fp.argv[0];

        match Tag::type_of(mu, ns) {
            Type::Namespace => {
                fp.value = Namespace::interns_of(mu, ns);
                Ok(())
            }
            _ => Err(Exception::new(Condition::Type, "mu:ns-int", ns)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn namespace() {
        assert_eq!(true, true)
    }
}
