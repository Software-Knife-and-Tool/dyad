//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! compile:
//!     function calls
//!     special forms
use {
    crate::{
        classes::{
            cons::{Cons, ConsIter, Core as _, Properties as _},
            fixnum::Fixnum,
            function::Function,
            namespace::{Core as _, Namespace, Scope},
            symbol::{Core as _, Symbol},
        },
        core::{
            classes::{Class, Tag},
            exception,
            exception::{Condition, Except, Result},
            mu::Mu,
        },
    },
    std::cell::{Ref, RefMut},
};

// special forms
type SpecFn = fn(&Mu, Tag) -> exception::Result<Tag>;
type SpecMap = (Tag, SpecFn);

lazy_static! {
    static ref SPECMAP: Vec<SpecMap> = vec![
        (Symbol::keyword("if"), compile_if),
        (Symbol::keyword("lambda"), compile_lambda),
        (Symbol::keyword("quote"), compile_quote),
    ];
}

fn compile_if(mu: &Mu, args: Tag) -> exception::Result<Tag> {
    if Cons::length(mu, args) != 3 {
        return Err(Except::raise(
            mu,
            Condition::Syntax,
            "compile::compile_quote",
            args,
        ));
    }

    let lambda = Symbol::keyword("lambda");

    let if_vec = vec![
        Namespace::intern(mu, mu.mu_ns, Scope::Intern, "if".to_string(), Tag::nil()),
        match Cons::nth(mu, 0, args) {
            Some(t) => t,
            None => panic!("internal: if argument inconsistency"),
        },
        Cons::list(
            mu,
            &[
                lambda,
                Tag::nil(),
                match Cons::nth(mu, 1, args) {
                    Some(t) => t,
                    None => panic!("internal: if argument inconsistency"),
                },
            ],
        ),
        Cons::list(
            mu,
            &[
                lambda,
                Tag::nil(),
                match Cons::nth(mu, 2, args) {
                    Some(t) => t,
                    None => panic!("internal: if argument inconsistency"),
                },
            ],
        ),
    ];

    compile(mu, Cons::list(mu, &if_vec))
}

fn compile_quote(mu: &Mu, args: Tag) -> exception::Result<Tag> {
    if Cons::length(mu, args) != 1 {
        return Err(Except::raise(
            mu,
            Condition::Syntax,
            "compile::compile_quote",
            args,
        ));
    }

    Ok(Cons::new(Symbol::keyword("quote"), args).evict(mu))
}

fn compile_special_form(mu: &Mu, name: Tag, args: Tag) -> exception::Result<Tag> {
    match SPECMAP.iter().copied().find(|spec| name.eq_(spec.0)) {
        Some(spec) => spec.1(mu, args),
        None => Err(Except::raise(
            mu,
            Condition::Syntax,
            "compile::special_form",
            args,
        )),
    }
}

// utilities
fn compile_list(mu: &Mu, body: Tag) -> exception::Result<Tag> {
    let mut body_vec = Vec::new();

    for cons in ConsIter::new(mu, body) {
        match compile(mu, Cons::car(mu, cons)) {
            Ok(expr) => body_vec.push(expr),
            Err(e) => return Err(e),
        }
    }

    Ok(Cons::list(mu, &body_vec))
}

// lexical symbols
fn compile_frame_symbols(mu: &Mu, lambda: Tag) -> exception::Result<Vec<Tag>> {
    let mut symv = Vec::new();

    for cons in ConsIter::new(mu, lambda) {
        let symbol = Cons::car(mu, cons);
        if Tag::class_of(mu, symbol) == Class::Symbol {
            match symv.iter().rev().position(|lex| symbol.eq_(*lex)) {
                Some(_) => {
                    return Err(Except::raise(
                        mu,
                        Condition::Syntax,
                        "compile::compile_frame_symbols",
                        symbol,
                    ))
                }
                _ => symv.push(symbol),
            }
        } else {
            return Err(Except::raise(
                mu,
                Condition::Type,
                "compile::compile_frame_symbols",
                symbol,
            ));
        }
    }

    Ok(symv)
}

fn compile_lexical(mu: &Mu, symbol: Tag) -> Result<Tag> {
    let lexenv_ref: Ref<Vec<(Tag, Vec<Tag>)>> = mu.compile.borrow();

    for frame in lexenv_ref.iter().rev() {
        let (tag, symbols) = frame;

        if let Some(nth) = symbols.iter().position(|lex| symbol.eq_(*lex)) {
            let lex_ref = vec![
                Namespace::intern(
                    mu,
                    mu.mu_ns,
                    Scope::Intern,
                    "fr-ref".to_string(),
                    Tag::nil(),
                ),
                Fixnum::as_tag(tag.as_u64() as i64),
                Fixnum::as_tag(nth as i64),
            ];

            match compile(mu, Cons::list(mu, &lex_ref)) {
                Ok(lexref) => return Ok(lexref),
                Err(e) => return Err(e),
            }
        }
    }

    Ok(symbol)
}

fn compile_lambda(mu: &Mu, args: Tag) -> exception::Result<Tag> {
    let (lambda, body) = match Tag::class_of(mu, args) {
        Class::Cons => {
            let lambda = Cons::car(mu, args);

            match Tag::class_of(mu, lambda) {
                Class::Null | Class::Cons => (lambda, Cons::cdr(mu, args)),
                _ => {
                    return Err(Except::raise(
                        mu,
                        Condition::Type,
                        "compile::compile_lambda",
                        args,
                    ))
                }
            }
        }
        _ => {
            return Err(Except::raise(
                mu,
                Condition::Syntax,
                "compile::compile_lambda",
                args,
            ))
        }
    };

    let frame_tag = Symbol::new(mu, Tag::nil(), Scope::Extern, "lambda", Tag::nil()).evict(mu);

    match compile_frame_symbols(mu, lambda) {
        Ok(lexicals) => {
            let mut lexenv_ref: RefMut<Vec<(Tag, Vec<Tag>)>> = mu.compile.borrow_mut();
            lexenv_ref.push((frame_tag, lexicals));
        }
        Err(e) => return Err(e),
    };

    match compile_list(mu, body) {
        Ok(func) => Ok(Function::new(
            func,
            Fixnum::as_tag(Cons::length(mu, lambda) as i64),
            frame_tag,
        )
        .evict(mu)),
        Err(e) => Err(e),
    }
}

pub fn compile(mu: &Mu, expr: Tag) -> exception::Result<Tag> {
    match Tag::class_of(mu, expr) {
        Class::Symbol => compile_lexical(mu, expr),
        Class::Cons => {
            let func = Cons::car(mu, expr);
            let args = Cons::cdr(mu, expr);
            match Tag::class_of(mu, func) {
                Class::Keyword => match compile_special_form(mu, func, args) {
                    Ok(form) => Ok(form),
                    Err(e) => Err(e),
                },
                Class::Symbol | Class::Function => match compile_list(mu, args) {
                    Ok(arglist) => Ok(Cons::new(func, arglist).evict(mu)),
                    Err(e) => Err(e),
                },
                Class::Cons => match compile_list(mu, args) {
                    Ok(arglist) => match compile(mu, func) {
                        Ok(fnc) => match Tag::class_of(mu, fnc) {
                            Class::Function => Ok(Cons::new(fnc, arglist).evict(mu)),
                            _ => Err(Except::raise(mu, Condition::Type, "compile::compile", func)),
                        },
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                },
                _ => Err(Except::raise(mu, Condition::Type, "compile::compile", func)),
            }
        }
        _ => Ok(expr),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        classes::{Class, Tag},
        compile,
        mu::{Core, Mu},
    };

    #[test]
    fn compile_test() {
        let mu: &Mu = &Core::new("".to_string());

        match compile::compile(mu, Tag::nil()) {
            Ok(form) => match Tag::class_of(mu, form) {
                Class::Null => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
        match compile::compile_list(mu, Tag::nil()) {
            Ok(form) => match Tag::class_of(mu, form) {
                Class::Null => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }
}
