//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu namespace symbols
use {
    crate::{
        classes::{
            cons::{Cons, MuFunction as _},
            fixnum::{Fixnum, MuFunction as _},
            float::{Float, MuFunction as _},
            function::Function,
            namespace::{Core as _, MuFunction as _, Namespace},
            stream::{MuFunction as _, Stream},
            symbol::{MuFunction as _, Symbol},
            vector::{MuFunction as _, Vector},
        },
        core::{
            classes::{MuFunction as _, Tag},
            coerce::MuFunction as _,
            exception::{Exception, MuFunction as _},
            frame::{Frame, MuFunction as _},
            functions::MuFunction as _,
            image::MuFunction as _,
            mu::{Mu, MuFunctionType},
        },
    },
    std::cell::RefMut,
};

// mu function dispatch table
pub type MuFunctionMap = (&'static str, u16, MuFunctionType);
lazy_static! {
    static ref FNMAP: Vec<MuFunctionMap> = vec![
        // conses and lists
        ("append", 2, Cons::mu_append),
        ("car", 1, Cons::mu_car),
        ("cdr", 1, Cons::mu_cdr),
        ("cons", 2, Cons::mu_cons),
        ("length", 1, Cons::mu_length),
        ("nth", 2, Cons::mu_nth),
        ("nthcdr", 2, Cons::mu_nthcdr),
        // mu
        ("%if", 3, Mu::mu_if),
        ("compile", 1, Mu::mu_compile),
        ("eval", 1, Mu::mu_eval),
        ("exit", 1, Mu::mu_exit),
        ("fix", 2, Mu::mu_fix),
        ("fix*", 2, Mu::mu_fix_env),
        ("funcall", 2, Mu::mu_funcall),
        ("tag-of", 1, Mu::mu_tag_of),
        ("view", 1, Mu::mu_view),
        // exceptions
        ("raise", 2, Exception::mu_raise),
        // frames
        ("fr-get", 1, Frame::mu_fr_get),
        ("fr-pop", 1, Frame::mu_fr_pop),
        ("fr-push", 1, Frame::mu_fr_push),
        ("fr-setv", 3, Frame::mu_fr_setv),
        ("%fr-ref", 2, Frame::mu_fr_ref),
        // types
        ("eq", 2, Tag::mu_eq),
        ("type-of", 1, Tag::mu_typeof),
        ("coerce", 2, Mu::mu_coerce),
        // fixnums
        ("fx-add", 2, Fixnum::mu_fxadd),
        ("fx-sub", 2, Fixnum::mu_fxsub),
        ("fx-lt", 2, Fixnum::mu_fxlt),
        ("fx-mul", 2, Fixnum::mu_fxmul),
        ("fx-div", 2, Fixnum::mu_fxdiv),
        ("logand", 2, Fixnum::mu_fxand),
        ("logor", 2, Fixnum::mu_fxor),
        // floats
        ("fl-add", 2, Float::mu_fladd),
        ("fl-sub", 2, Float::mu_flsub),
        ("fl-lt", 2, Float::mu_fllt),
        ("fl-mul", 2, Float::mu_flmul),
        ("fl-div", 2, Float::mu_fldiv),
        // heap
        ("hp-info", 0, Mu::mu_hp_info),
        ("hp-type", 2, Mu::mu_hp_type),
        // namespaces
        ("intern", 4, Namespace::mu_intern),
        ("make-ns", 2, Namespace::mu_make_ns),
        ("map-ns", 1, Namespace::mu_map_ns),
        ("ns-map", 3, Namespace::mu_ns_map),
        ("ns-imp", 1, Namespace::mu_ns_import),
        ("ns-name", 1, Namespace::mu_ns_name),
        // read/write
        ("read", 3, Stream::mu_read),
        ("write", 3, Stream::mu_write),
        // symbols
        ("boundp", 1, Symbol::mu_boundp),
        ("keyp", 1, Symbol::mu_keywordp),
        ("keyword", 1, Symbol::mu_keyword),
        ("symbol", 1, Symbol::mu_symbol),
        ("sy-name", 1, Symbol::mu_name),
        ("sy-ns", 1, Symbol::mu_ns),
        ("sy-val", 1, Symbol::mu_value),
        // simple vectors
        ("slice", 3, Vector::mu_slice),
        ("make-sv", 2, Vector::mu_make_vector),
        ("sv-len", 1, Vector::mu_length),
        ("sv-ref", 2, Vector::mu_svref),
        ("sv-type", 1, Vector::mu_type),
        // streams
        ("close", 1, Stream::mu_close),
        ("eof", 1, Stream::mu_eof),
        ("get-str", 1, Stream::mu_get_string),
        ("open", 3, Stream::mu_open),
        ("openp", 1, Stream::mu_openp),
        ("rd-byte", 3, Stream::mu_read_byte),
        ("rd-char", 3, Stream::mu_read_char),
        ("un-char", 2, Stream::mu_unread_char),
        ("wr-byte", 2, Stream::mu_write_byte),
        ("wr-char", 2, Stream::mu_write_char),
    ];
}
pub trait Core {
    fn fnmap(_: usize) -> MuFunctionMap;
    fn install_mu_symbols(_: &Mu);
}

impl Core for Mu {
    fn fnmap(index: usize) -> MuFunctionMap {
        FNMAP[index]
    }

    // populate the mu namespace
    fn install_mu_symbols(mu: &Mu) {
        Namespace::intern(mu, mu.mu_ns, true, "version".to_string(), mu.version);
        Namespace::intern(mu, mu.mu_ns, true, "std-in".to_string(), mu.stdin);
        Namespace::intern(mu, mu.mu_ns, true, "std-out".to_string(), mu.stdout);
        Namespace::intern(mu, mu.mu_ns, true, "err-out".to_string(), mu.errout);

        for (fn_id, fnmap) in FNMAP.iter().enumerate() {
            let (fn_name, fn_nreqs, fn_fn) = fnmap;
            let mut fn_ref: RefMut<Vec<MuFunctionType>> = mu.fnmap.borrow_mut();

            let func = Function::new(
                Fixnum::as_tag(match fn_id.try_into().unwrap() {
                    Some(n) => n as i64,
                    None => panic!("internal: mu function id inconsistency"),
                }),
                Fixnum::as_tag(*fn_nreqs as i64),
                Tag::nil(),
            )
            .evict(mu);

            Namespace::intern(mu, mu.mu_ns, true, fn_name.to_string(), func);
            fn_ref.push(*fn_fn);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mu_namespace() {
        assert_eq!(2 + 2, 4);
    }
}
