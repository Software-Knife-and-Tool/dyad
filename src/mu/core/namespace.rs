//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu namespace symbols
use crate::{
    core::{
        classes::{MuFunction as _, Tag},
        exception::{Exception, MuFunction as _},
        frame::{Frame, MuFunction as _},
        functions::MuFunction as _,
        image::MuFunction as _,
        mu::{Mu, MuFunctionType},
    },
    types::{
        coerce::MuFunction as _,
        cons::{Cons, MuFunction as _},
        fixnum::{Fixnum, MuFunction as _},
        float::{Float, MuFunction as _},
        function::Function,
        namespace::{Core as _, MuFunction as _, Namespace, Scope},
        r#struct::{MuFunction as _, Struct},
        stream::{MuFunction as _, Stream},
        symbol::{MuFunction as _, Symbol},
        vector::{MuFunction as _, Vector},
    },
};

// mu function dispatch table
lazy_static! {
    static ref FUNCTIONMAP: Vec<<Mu as Core>::FunctionDesc> = vec![
        // conses and lists
        ("append", Scope::Extern, 2, Cons::mu_append),
        ("car", Scope::Extern, 1, Cons::mu_car),
        ("cdr", Scope::Extern, 1, Cons::mu_cdr),
        ("cons", Scope::Extern, 2, Cons::mu_cons),
        ("length", Scope::Extern, 1, Cons::mu_length),
        ("nth", Scope::Extern, 2, Cons::mu_nth),
        ("nthcdr", Scope::Extern, 2, Cons::mu_nthcdr),
        // mu
        ("apply", Scope::Extern, 2, Mu::mu_apply),
        ("compile", Scope::Extern, 1, Mu::mu_compile),
        ("eval", Scope::Extern, 1, Mu::mu_eval),
        ("exit", Scope::Intern, 1, Mu::mu_exit),
        ("fix", Scope::Extern, 2, Mu::mu_fix),
        ("hp-info", Scope::Extern, 0, Mu::mu_hp_info),
        ("tag-of", Scope::Extern, 1, Mu::mu_tag_of),
        ("view", Scope::Extern, 1, Mu::mu_view),
        // exceptions
        ("with-ex", Scope::Extern, 2, Exception::mu_with_ex),
        ("raise", Scope::Extern, 2, Exception::mu_raise),
        // frames
        ("context", Scope::Intern, 0, Frame::mu_context),
        ("fr-get", Scope::Extern, 1, Frame::mu_fr_get),
        ("fr-pop", Scope::Extern, 1, Frame::mu_fr_pop),
        ("fr-push", Scope::Extern, 1, Frame::mu_fr_push),
        // types
        ("eq", Scope::Extern, 2, Tag::mu_eq),
        ("type-of", Scope::Extern, 1, Tag::mu_typeof),
        ("coerce", Scope::Extern, 2, Mu::mu_coerce),
        // fixnums
        ("fx-add", Scope::Extern, 2, Fixnum::mu_fxadd),
        ("fx-sub", Scope::Extern, 2, Fixnum::mu_fxsub),
        ("fx-lt", Scope::Extern, 2, Fixnum::mu_fxlt),
        ("fx-mul", Scope::Extern, 2, Fixnum::mu_fxmul),
        ("fx-div", Scope::Extern, 2, Fixnum::mu_fxdiv),
        ("logand", Scope::Extern, 2, Fixnum::mu_fxand),
        ("logor", Scope::Extern, 2, Fixnum::mu_fxor),
        // floats
        ("fl-add", Scope::Extern, 2, Float::mu_fladd),
        ("fl-sub", Scope::Extern, 2, Float::mu_flsub),
        ("fl-lt", Scope::Extern, 2, Float::mu_fllt),
        ("fl-mul", Scope::Extern, 2, Float::mu_flmul),
        ("fl-div", Scope::Extern, 2, Float::mu_fldiv),
        // namespaces
        ("intern", Scope::Extern, 4, Namespace::mu_intern),
        ("make-ns", Scope::Extern, 2, Namespace::mu_make_ns),
        ("map-ns", Scope::Extern, 1, Namespace::mu_map_ns),
        ("ns-ext", Scope::Extern, 1, Namespace::mu_ns_externs),
        ("ns-imp", Scope::Extern, 1, Namespace::mu_ns_import),
        ("ns-int", Scope::Extern, 1, Namespace::mu_ns_interns),
        ("ns-map", Scope::Extern, 3, Namespace::mu_ns_map),
        ("ns-name", Scope::Extern, 1, Namespace::mu_ns_name),
        // read/write
        ("read", Scope::Extern, 3, Stream::mu_read),
        ("write", Scope::Extern, 3, Stream::mu_write),
        // symbols
        ("boundp", Scope::Extern, 1, Symbol::mu_boundp),
        ("keyp", Scope::Extern, 1, Symbol::mu_keywordp),
        ("keyword", Scope::Extern, 1, Symbol::mu_keyword),
        ("symbol", Scope::Extern, 1, Symbol::mu_symbol),
        ("sy-name", Scope::Extern, 1, Symbol::mu_name),
        ("sy-ns", Scope::Extern, 1, Symbol::mu_ns),
        ("sy-val", Scope::Extern, 1, Symbol::mu_value),
        // simple vectors
        ("vector", Scope::Extern, 2, Vector::mu_make_vector),
        ("sv-len", Scope::Extern, 1, Vector::mu_length),
        ("sv-ref", Scope::Extern, 2, Vector::mu_svref),
        ("sv-type", Scope::Extern, 1, Vector::mu_type),
        // structs
        ("struct", Scope::Extern, 2, Struct::mu_make_struct),
        ("st-type", Scope::Extern, 1, Struct::mu_struct_type),
        ("st-vec", Scope::Extern, 1, Struct::mu_struct_vector),
        // streams
        ("close", Scope::Extern, 1, Stream::mu_close),
        ("eof", Scope::Extern, 1, Stream::mu_eof),
        ("get-str", Scope::Extern, 1, Stream::mu_get_string),
        ("open", Scope::Extern, 3, Stream::mu_open),
        ("openp", Scope::Extern, 1, Stream::mu_openp),
        ("rd-byte", Scope::Extern, 3, Stream::mu_read_byte),
        ("rd-char", Scope::Extern, 3, Stream::mu_read_char),
        ("un-char", Scope::Extern, 2, Stream::mu_unread_char),
        ("wr-byte", Scope::Extern, 2, Stream::mu_write_byte),
        ("wr-char", Scope::Extern, 2, Stream::mu_write_char),
        // interns
        ("if", Scope::Intern, 3, Mu::mu_if),
        ("fr-ref", Scope::Intern, 2, Frame::mu_fr_ref),
    ];
}

pub trait Core {
    type FunctionDesc;
    fn map_core(_: usize) -> <Mu as Core>::FunctionDesc;
    fn install_mu_symbols(_: &Mu);
}

impl Core for Mu {
    type FunctionDesc = (&'static str, Scope, u16, MuFunctionType);

    fn map_core(index: usize) -> <Mu as Core>::FunctionDesc {
        FUNCTIONMAP[index]
    }

    fn install_mu_symbols(mu: &Mu) {
        Namespace::intern(
            mu,
            mu.mu_ns,
            Scope::Extern,
            "version".to_string(),
            mu.version,
        );
        Namespace::intern(mu, mu.mu_ns, Scope::Extern, "std-in".to_string(), mu.stdin);
        Namespace::intern(
            mu,
            mu.mu_ns,
            Scope::Extern,
            "std-out".to_string(),
            mu.stdout,
        );
        Namespace::intern(
            mu,
            mu.mu_ns,
            Scope::Extern,
            "err-out".to_string(),
            mu.errout,
        );

        for (id, fnmap) in FUNCTIONMAP.iter().enumerate() {
            let (name, scope, nreqs, _) = fnmap;

            let func = Function::new(
                Tag::nil(),
                Fixnum::as_tag(*nreqs as i64),
                Fixnum::as_tag(match id.try_into().unwrap() {
                    Some(n) => n as i64,
                    None => panic!("internal: mu function id inconsistency"),
                }),
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
    fn namespace() {
        assert_eq!(2 + 2, 4);
    }
}
