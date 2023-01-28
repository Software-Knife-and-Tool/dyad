//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu namespace symbols
use crate::{
    classes::{
        cons::{Cons, MuFunction as _},
        fixnum::{Fixnum, MuFunction as _},
        float::{Float, MuFunction as _},
        function::Function,
        namespace::{Core as _, MuFunction as _, Namespace, NS_EXTERN, NS_INTERN},
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
        mu::{FunctionDesc, Mu},
    },
};

// mu function dispatch table
lazy_static! {
    static ref FUNCTIONMAP: Vec<FunctionDesc> = vec![
        // conses and lists
        ("append", NS_EXTERN, 2, Cons::mu_append),
        ("car", NS_EXTERN, 1, Cons::mu_car),
        ("cdr", NS_EXTERN, 1, Cons::mu_cdr),
        ("cons", NS_EXTERN, 2, Cons::mu_cons),
        ("length", NS_EXTERN, 1, Cons::mu_length),
        ("nth", NS_EXTERN, 2, Cons::mu_nth),
        ("nthcdr", NS_EXTERN, 2, Cons::mu_nthcdr),
        // mu
        ("compile", NS_EXTERN, 1, Mu::mu_compile),
        ("eval", NS_EXTERN, 1, Mu::mu_eval),
        ("exit", NS_EXTERN, 1, Mu::mu_exit),
        ("fix", NS_EXTERN, 2, Mu::mu_fix),
        ("fix*", NS_EXTERN, 2, Mu::mu_fix_env),
        ("funcall", NS_EXTERN, 2, Mu::mu_funcall),
        ("tag-of", NS_EXTERN, 1, Mu::mu_tag_of),
        ("view", NS_EXTERN, 1, Mu::mu_view),
        // exceptions
        ("raise", NS_EXTERN, 2, Exception::mu_raise),
        // frames
        ("fr-get", NS_EXTERN, 1, Frame::mu_fr_get),
        ("fr-pop", NS_EXTERN, 1, Frame::mu_fr_pop),
        ("fr-push", NS_EXTERN, 1, Frame::mu_fr_push),
        ("fr-setv", NS_EXTERN, 3, Frame::mu_fr_setv),
        // types
        ("eq", NS_EXTERN, 2, Tag::mu_eq),
        ("type-of", NS_EXTERN, 1, Tag::mu_typeof),
        ("coerce", NS_EXTERN, 2, Mu::mu_coerce),
        // fixnums
        ("fx-add", NS_EXTERN, 2, Fixnum::mu_fxadd),
        ("fx-sub", NS_EXTERN, 2, Fixnum::mu_fxsub),
        ("fx-lt", NS_EXTERN, 2, Fixnum::mu_fxlt),
        ("fx-mul", NS_EXTERN, 2, Fixnum::mu_fxmul),
        ("fx-div", NS_EXTERN, 2, Fixnum::mu_fxdiv),
        ("logand", NS_EXTERN, 2, Fixnum::mu_fxand),
        ("logor", NS_EXTERN, 2, Fixnum::mu_fxor),
        // floats
        ("fl-add", NS_EXTERN, 2, Float::mu_fladd),
        ("fl-sub", NS_EXTERN, 2, Float::mu_flsub),
        ("fl-lt", NS_EXTERN, 2, Float::mu_fllt),
        ("fl-mul", NS_EXTERN, 2, Float::mu_flmul),
        ("fl-div", NS_EXTERN, 2, Float::mu_fldiv),
        // heap
        ("hp-info", NS_EXTERN, 0, Mu::mu_hp_info),
        ("hp-type", NS_EXTERN, 2, Mu::mu_hp_type),
        // namespaces
        ("intern", NS_EXTERN, 4, Namespace::mu_intern),
        ("make-ns", NS_EXTERN, 2, Namespace::mu_make_ns),
        ("map-ns", NS_EXTERN, 1, Namespace::mu_map_ns),
        ("ns-map", NS_EXTERN, 3, Namespace::mu_ns_map),
        ("ns-imp", NS_EXTERN, 1, Namespace::mu_ns_import),
        ("ns-name", NS_EXTERN, 1, Namespace::mu_ns_name),
        // read/write
        ("read", NS_EXTERN, 3, Stream::mu_read),
        ("write", NS_EXTERN, 3, Stream::mu_write),
        // symbols
        ("boundp", NS_EXTERN, 1, Symbol::mu_boundp),
        ("keyp", NS_EXTERN, 1, Symbol::mu_keywordp),
        ("keyword", NS_EXTERN, 1, Symbol::mu_keyword),
        ("symbol", NS_EXTERN, 1, Symbol::mu_symbol),
        ("sy-name", NS_EXTERN, 1, Symbol::mu_name),
        ("sy-ns", NS_EXTERN, 1, Symbol::mu_ns),
        ("sy-val", NS_EXTERN, 1, Symbol::mu_value),
        // simple vectors
        ("make-sv", NS_EXTERN, 2, Vector::mu_make_vector),
        ("sv-len", NS_EXTERN, 1, Vector::mu_length),
        ("sv-ref", NS_EXTERN, 2, Vector::mu_svref),
        ("sv-type", NS_EXTERN, 1, Vector::mu_type),
        // streams
        ("close", NS_EXTERN, 1, Stream::mu_close),
        ("eof", NS_EXTERN, 1, Stream::mu_eof),
        ("get-str", NS_EXTERN, 1, Stream::mu_get_string),
        ("open", NS_EXTERN, 3, Stream::mu_open),
        ("openp", NS_EXTERN, 1, Stream::mu_openp),
        ("rd-byte", NS_EXTERN, 3, Stream::mu_read_byte),
        ("rd-char", NS_EXTERN, 3, Stream::mu_read_char),
        ("un-char", NS_EXTERN, 2, Stream::mu_unread_char),
        ("wr-byte", NS_EXTERN, 2, Stream::mu_write_byte),
        ("wr-char", NS_EXTERN, 2, Stream::mu_write_char),
        ("%if", NS_EXTERN, 3, Mu::mu_if),
        ("%fr-ref", NS_EXTERN, 2, Frame::mu_fr_ref),
        ("if", NS_INTERN, 3, Mu::mu_if),
        ("fr-ref", NS_INTERN, 2, Frame::mu_fr_ref),
    ];
}

pub trait Core {
    fn functionmap(_: usize) -> FunctionDesc;
    fn install_mu_symbols(_: &Mu);
}

impl Core for Mu {
    fn functionmap(index: usize) -> FunctionDesc {
        FUNCTIONMAP[index]
    }

    fn install_mu_symbols(mu: &Mu) {
        Namespace::intern(mu, mu.mu_ns, NS_EXTERN, "version".to_string(), mu.version);
        Namespace::intern(mu, mu.mu_ns, NS_EXTERN, "std-in".to_string(), mu.stdin);
        Namespace::intern(mu, mu.mu_ns, NS_EXTERN, "std-out".to_string(), mu.stdout);
        Namespace::intern(mu, mu.mu_ns, NS_EXTERN, "err-out".to_string(), mu.errout);

        for (id, fnmap) in FUNCTIONMAP.iter().enumerate() {
            let (name, scope, nreqs, _) = fnmap;

            let func = Function::new(
                Fixnum::as_tag(match id.try_into().unwrap() {
                    Some(n) => n as i64,
                    None => panic!("internal: mu function id inconsistency"),
                }),
                Fixnum::as_tag(*nreqs as i64),
                Tag::nil(),
            )
            .evict(mu);

            Namespace::intern(mu, mu.mu_ns, *scope, name.to_string(), func);
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
