//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu reader
use crate::{
    classes::{
        char::Char,
        cons::{Cons, Core as _},
        fixnum::Fixnum,
        float::Float,
        namespace::{Core as _, Namespace, Scope},
        stream::{Core as _, Stream},
        symbol::{Core as _, Symbol, UNBOUND},
        vector::{Core as _, Vector},
    },
    core::{
        classes::{Class, DirectClass, Tag},
        exception,
        exception::{Condition, Except},
        mu::Mu,
        readtable::{map_char_syntax, SyntaxType},
    },
};

lazy_static! {
    pub static ref EOL: Tag = Tag::to_direct(0, 0, DirectClass::Keyword);
}

//
// read functions return:
//
//     Ok(Some(())) if the function succeeded,
//     Ok(None) if end of file
//     Err if stream or syntax error
//     errors propagate out of read()
//

pub trait Read {
    fn read(_: &Mu, _: Tag, _: bool, _: Tag, _: bool) -> exception::Result<Tag>;
    fn read_atom(_: &Mu, _: char, _: Tag) -> exception::Result<Tag>;
    fn read_block_comment(_: &Mu, _: Tag) -> exception::Result<Option<()>>;
    fn read_comment(_: &Mu, _: Tag) -> exception::Result<Option<()>>;
    fn read_ws(_: &Mu, _: Tag) -> exception::Result<Option<()>>;
    fn sharp_macro(_: &Mu, _: Tag) -> exception::Result<Option<Tag>>;
    fn read_token(_: &Mu, _: Tag) -> exception::Result<Option<String>>;
}

impl Read for Mu {
    //
    // read whitespace:
    //
    //    leave non-ws char at the head of the stream
    //    return None on end of file (not an error)
    //    return Err exception for stream error
    //    return Ok(Some(())) for ws consumed
    //
    fn read_ws(mu: &Mu, stream: Tag) -> exception::Result<Option<()>> {
        loop {
            match Stream::read_char(mu, stream) {
                Ok(Some(ch)) => {
                    if let Some(stype) = map_char_syntax(ch) {
                        match stype {
                            SyntaxType::Whitespace => (),
                            _ => {
                                Stream::unread_char(mu, stream, ch).unwrap();
                                break;
                            }
                        }
                    }
                }
                Ok(None) => return Ok(None),
                Err(e) => return Err(e),
            }
        }

        Ok(Some(()))
    }

    // read comment til end of line:
    //
    //     return Err exception for stream error
    //     return Ok(Some(())) for comment consumed
    //
    fn read_comment(mu: &Mu, stream: Tag) -> exception::Result<Option<()>> {
        loop {
            match Stream::read_char(mu, stream) {
                Ok(Some(ch)) => {
                    if ch == '\n' {
                        break;
                    }
                }
                Ok(None) => {
                    return Err(Except::raise(
                        mu,
                        Condition::Eof,
                        "read::read_comment",
                        stream,
                    ))
                }
                Err(e) => return Err(e),
            }
        }

        Ok(Some(()))
    }

    // read block comment
    //
    //     leave non-ws char at the head of the stream
    //     return Err exception for stream error
    //     return Ok(Some(())) for comment consumed
    //
    fn read_block_comment(mu: &Mu, stream: Tag) -> exception::Result<Option<()>> {
        loop {
            match Stream::read_char(mu, stream) {
                Ok(Some(ch)) => {
                    if ch == '|' {
                        match Stream::read_char(mu, stream) {
                            Ok(Some(ch)) => {
                                if ch == '#' {
                                    break;
                                }
                            }
                            Ok(None) => {
                                return Err(Except::raise(
                                    mu,
                                    Condition::Eof,
                                    "read::read_block_comment",
                                    stream,
                                ))
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
                Ok(None) => {
                    return Err(Except::raise(
                        mu,
                        Condition::Eof,
                        "read::read_block_comment",
                        stream,
                    ))
                }
                Err(e) => return Err(e),
            }
        }

        Ok(Some(()))
    }

    // read token
    //
    //     return Err exception for stream error
    //     return Ok(Some(String))
    //
    fn read_token(mu: &Mu, stream: Tag) -> exception::Result<Option<String>> {
        let mut token = String::new();

        loop {
            match Stream::read_char(mu, stream) {
                Ok(Some(ch)) => match map_char_syntax(ch) {
                    Some(stype) => match stype {
                        SyntaxType::Constituent => token.push(ch),
                        SyntaxType::Whitespace | SyntaxType::Tmacro => {
                            Stream::unread_char(mu, stream, ch).unwrap();
                            break;
                        }
                        _ => {
                            return Err(Except::raise(
                                mu,
                                Condition::Range,
                                "read::read_token",
                                stream,
                            ))
                        }
                    },
                    None => {
                        return Err(Except::raise(
                            mu,
                            Condition::Range,
                            "read::read_token",
                            stream,
                        ))
                    }
                },
                Ok(None) => {
                    break;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(Some(token))
    }

    // read symbol or literal:
    //
    //      leave non-ws char at the head of the stream
    //      return Some(tag) for successful read
    //      return Err exception for stream I/O error or unexpected eof
    //
    fn read_atom(mu: &Mu, ch: char, stream: Tag) -> exception::Result<Tag> {
        let mut token = String::new();

        token.push(ch);
        loop {
            match Stream::read_char(mu, stream) {
                Ok(Some(ch)) => match map_char_syntax(ch) {
                    Some(stype) => match stype {
                        SyntaxType::Constituent => token.push(ch),
                        SyntaxType::Whitespace | SyntaxType::Tmacro => {
                            Stream::unread_char(mu, stream, ch).unwrap();
                            break;
                        }
                        _ => {
                            return Err(Except::raise(
                                mu,
                                Condition::Range,
                                "read::read_atom",
                                stream,
                            ))
                        }
                    },
                    None => {
                        return Err(Except::raise(
                            mu,
                            Condition::Range,
                            "read::read_atom",
                            stream,
                        ))
                    }
                },
                Ok(None) => {
                    break;
                }
                Err(e) => return Err(e),
            }
        }

        match token.parse::<i64>() {
            Ok(fx) => Ok(Fixnum::as_tag(fx)),
            Err(_) => match token.parse::<f32>() {
                Ok(fl) => Ok(Float::as_tag(fl)),
                Err(_) => match token.find(':') {
                    Some(0) => {
                        Ok(Symbol::new(mu, Tag::nil(), Scope::Extern, &token, *UNBOUND).evict(mu))
                    }
                    Some(_) => {
                        let sym: Vec<&str> = token.split(':').collect();
                        match sym.len() {
                            2 => {
                                let ns = sym[0].to_string();
                                let name = sym[1].to_string();

                                match Namespace::map_ns(mu, ns) {
                                    Some(ns) => {
                                        Ok(Namespace::intern(mu, ns, Scope::Extern, name, *UNBOUND))
                                    }
                                    None => Err(Except::raise(
                                        mu,
                                        Condition::Unbound,
                                        "read::read_atom",
                                        Vector::from_string(sym[0]).evict(mu),
                                    )),
                                }
                            }
                            3 => {
                                let ns = sym[0].to_string();
                                let name = sym[2].to_string();

                                match Namespace::map_ns(mu, ns) {
                                    Some(ns) => {
                                        Ok(Namespace::intern(mu, ns, Scope::Intern, name, *UNBOUND))
                                    }
                                    None => Err(Except::raise(
                                        mu,
                                        Condition::Unbound,
                                        "read::read_atom",
                                        Vector::from_string(sym[0]).evict(mu),
                                    )),
                                }
                            }
                            _ => Err(Except::raise(
                                mu,
                                Condition::Syntax,
                                "read::read_atom",
                                Vector::from_string(&token).evict(mu),
                            )),
                        }
                    }
                    None => Ok(Namespace::intern(
                        mu,
                        mu.nil_ns,
                        Scope::Extern,
                        token,
                        *UNBOUND,
                    )),
                },
            },
        }
    }

    // sharp_macro returns:
    //
    //     Err exception if I/O problem or syntax error
    //     Ok(tag) if the read succeeded,
    //
    fn sharp_macro(mu: &Mu, stream: Tag) -> exception::Result<Option<Tag>> {
        match Stream::read_char(mu, stream) {
            Ok(Some(ch)) => match ch {
                ':' => match Stream::read_char(mu, stream) {
                    Ok(Some(ch)) => match Self::read_atom(mu, ch, stream) {
                        Ok(atom) => match Tag::class_of(mu, atom) {
                            Class::Symbol => Ok(Some(atom)),
                            _ => Err(Except::raise(
                                mu,
                                Condition::Type,
                                "read::sharp_macro",
                                stream,
                            )),
                        },
                        Err(e) => Err(e),
                    },
                    Ok(None) => Err(Except::raise(
                        mu,
                        Condition::Eof,
                        "read::sharp_macro",
                        stream,
                    )),
                    Err(e) => Err(e),
                },
                '|' => match Self::read_block_comment(mu, stream) {
                    Ok(Some(())) => Ok(None),
                    Ok(None) => Err(Except::raise(
                        mu,
                        Condition::Eof,
                        "read::sharp_macro",
                        stream,
                    )),
                    Err(e) => Err(e),
                },
                '\\' => match Stream::read_char(mu, stream) {
                    Ok(Some(ch)) => Ok(Some(Char::as_tag(ch))),
                    Ok(None) => Err(Except::raise(
                        mu,
                        Condition::Eof,
                        "read::sharp_macro",
                        stream,
                    )),
                    Err(e) => Err(e),
                },
                '(' => match Vector::read(mu, '(', stream) {
                    Ok(vec) => Ok(Some(vec)),
                    Err(e) => Err(e),
                },
                'x' => match Self::read_token(mu, stream) {
                    Ok(token) => match token {
                        Some(hex) => match i64::from_str_radix(&hex, 16) {
                            Ok(fx) => Ok(Some(Fixnum::as_tag(fx))),
                            Err(_) => Err(Except::raise(
                                mu,
                                Condition::Syntax,
                                "read::sharp_macro",
                                Char::as_tag(ch),
                            )),
                        },
                        None => panic!(),
                    },
                    Err(_) => Err(Except::raise(
                        mu,
                        Condition::Syntax,
                        "read::sharp_macro",
                        Char::as_tag(ch),
                    )),
                },
                _ => Err(Except::raise(
                    mu,
                    Condition::Type,
                    "read::sharp_macro",
                    Char::as_tag(ch),
                )),
            },
            Ok(None) => Err(Except::raise(
                mu,
                Condition::Eof,
                "read::sharp_macro",
                stream,
            )),
            Err(e) => Err(e),
        }
    }

    // read returns:
    //
    //     Err raise exception if I/O problem, syntax error, or end of file and !eofp
    //     Ok(eof_value) if end of file and eofp
    //     Ok(tag) if the read succeeded,
    //
    #[allow(clippy::only_used_in_recursion)]
    fn read(
        mu: &Mu,
        stream: Tag,
        eofp: bool,
        eof_value: Tag,
        recursivep: bool,
    ) -> exception::Result<Tag> {
        match Self::read_ws(mu, stream) {
            Ok(None) => {
                if eofp {
                    return Ok(eof_value);
                } else {
                    return Err(Except::raise(mu, Condition::Eof, "read::read", stream));
                }
            }
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        match Stream::read_char(mu, stream) {
            Ok(None) => {
                if eofp {
                    Ok(eof_value)
                } else {
                    Err(Except::raise(mu, Condition::Eof, "read::read", stream))
                }
            }
            Ok(Some(ch)) => match map_char_syntax(ch) {
                Some(stype) => match stype {
                    SyntaxType::Constituent => Self::read_atom(mu, ch, stream),
                    SyntaxType::Macro => match ch {
                        '#' => match Self::sharp_macro(mu, stream) {
                            Ok(Some(tag)) => Ok(tag),
                            Ok(None) => {
                                <Mu as Read>::read(mu, stream, false, Tag::nil(), recursivep)
                            }
                            Err(e) => Err(e),
                        },
                        _ => Err(Except::raise(
                            mu,
                            Condition::Type,
                            "read::read",
                            Fixnum::as_tag(ch as i64),
                        )),
                    },
                    SyntaxType::Tmacro => match ch {
                        '\'' => match <Mu as Read>::read(mu, stream, false, Tag::nil(), recursivep)
                        {
                            Ok(tag) => Ok(Cons::new(
                                Symbol::keyword("quote"),
                                Cons::new(tag, Tag::nil()).evict(mu),
                            )
                            .evict(mu)),
                            Err(e) => Err(e),
                        },
                        '"' => match Vector::read(mu, '"', stream) {
                            Ok(tag) => Ok(tag),
                            Err(e) => Err(e),
                        },
                        '(' => match Cons::read(mu, stream) {
                            Ok(cons) => Ok(cons),
                            Err(e) => Err(e),
                        },
                        ')' => {
                            if recursivep {
                                Ok(*EOL)
                            } else {
                                Err(Except::raise(mu, Condition::Syntax, "read:read", stream))
                            }
                        }
                        ';' => match <Mu as Read>::read_comment(mu, stream) {
                            Ok(_) => <Mu as Read>::read(mu, stream, eofp, eof_value, recursivep),
                            Err(e) => Err(e),
                        },
                        _ => Err(Except::raise(
                            mu,
                            Condition::Range,
                            "read::read_atom",
                            Char::as_tag(ch),
                        )),
                    },
                    _ => Err(Except::raise(
                        mu,
                        Condition::Read,
                        "read::read_atom",
                        Char::as_tag(ch),
                    )),
                },
                _ => Err(Except::raise(
                    mu,
                    Condition::Read,
                    "read::read_atom",
                    Char::as_tag(ch),
                )),
            },
            Err(e) => Err(e),
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
