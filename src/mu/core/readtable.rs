//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu reader readtable
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum SyntaxType {
    Constituent,
    Whitespace,
    Macro,
    Tmacro,
    Escape,
    Mescape,
}

pub fn map_char_syntax(ch: char) -> Option<&'static SyntaxType> {
    SYNTAX_MAP.get(&ch)
}

lazy_static! {
    static ref SYNTAX_MAP: HashMap::<char, SyntaxType> = HashMap::<char, SyntaxType>::from([
        ('0', SyntaxType::Constituent),
        ('1', SyntaxType::Constituent),
        ('2', SyntaxType::Constituent),
        ('3', SyntaxType::Constituent),
        ('4', SyntaxType::Constituent),
        ('5', SyntaxType::Constituent),
        ('6', SyntaxType::Constituent),
        ('7', SyntaxType::Constituent),
        ('8', SyntaxType::Constituent),
        ('9', SyntaxType::Constituent),
        (':', SyntaxType::Constituent),
        ('<', SyntaxType::Constituent),
        ('>', SyntaxType::Constituent),
        ('=', SyntaxType::Constituent),
        ('?', SyntaxType::Constituent),
        ('!', SyntaxType::Constituent),
        ('@', SyntaxType::Constituent),
        ('\t', SyntaxType::Whitespace),
        ('\n', SyntaxType::Whitespace),
        ('\r', SyntaxType::Whitespace),
        (' ', SyntaxType::Whitespace),
        (';', SyntaxType::Tmacro),
        ('"', SyntaxType::Tmacro),
        ('#', SyntaxType::Macro),
        ('\'', SyntaxType::Tmacro),
        ('(', SyntaxType::Tmacro),
        (')', SyntaxType::Tmacro),
        ('`', SyntaxType::Tmacro),
        (',', SyntaxType::Tmacro),
        ('\\', SyntaxType::Escape),
        ('|', SyntaxType::Mescape),
        ('A', SyntaxType::Constituent),
        ('B', SyntaxType::Constituent),
        ('C', SyntaxType::Constituent),
        ('D', SyntaxType::Constituent),
        ('E', SyntaxType::Constituent),
        ('F', SyntaxType::Constituent),
        ('G', SyntaxType::Constituent),
        ('H', SyntaxType::Constituent),
        ('I', SyntaxType::Constituent),
        ('J', SyntaxType::Constituent),
        ('K', SyntaxType::Constituent),
        ('L', SyntaxType::Constituent),
        ('M', SyntaxType::Constituent),
        ('N', SyntaxType::Constituent),
        ('O', SyntaxType::Constituent),
        ('P', SyntaxType::Constituent),
        ('Q', SyntaxType::Constituent),
        ('R', SyntaxType::Constituent),
        ('S', SyntaxType::Constituent),
        ('T', SyntaxType::Constituent),
        ('V', SyntaxType::Constituent),
        ('W', SyntaxType::Constituent),
        ('X', SyntaxType::Constituent),
        ('Y', SyntaxType::Constituent),
        ('Z', SyntaxType::Constituent),
        ('[', SyntaxType::Constituent),
        (']', SyntaxType::Constituent),
        ('$', SyntaxType::Constituent),
        ('*', SyntaxType::Constituent),
        ('{', SyntaxType::Constituent),
        ('}', SyntaxType::Constituent),
        ('+', SyntaxType::Constituent),
        ('-', SyntaxType::Constituent),
        ('/', SyntaxType::Constituent),
        ('~', SyntaxType::Constituent),
        ('.', SyntaxType::Constituent),
        ('%', SyntaxType::Constituent),
        ('&', SyntaxType::Constituent),
        ('^', SyntaxType::Constituent),
        ('_', SyntaxType::Constituent),
        ('a', SyntaxType::Constituent),
        ('b', SyntaxType::Constituent),
        ('c', SyntaxType::Constituent),
        ('d', SyntaxType::Constituent),
        ('e', SyntaxType::Constituent),
        ('f', SyntaxType::Constituent),
        ('g', SyntaxType::Constituent),
        ('h', SyntaxType::Constituent),
        ('i', SyntaxType::Constituent),
        ('j', SyntaxType::Constituent),
        ('k', SyntaxType::Constituent),
        ('l', SyntaxType::Constituent),
        ('m', SyntaxType::Constituent),
        ('n', SyntaxType::Constituent),
        ('o', SyntaxType::Constituent),
        ('p', SyntaxType::Constituent),
        ('q', SyntaxType::Constituent),
        ('r', SyntaxType::Constituent),
        ('s', SyntaxType::Constituent),
        ('t', SyntaxType::Constituent),
        ('u', SyntaxType::Constituent),
        ('v', SyntaxType::Constituent),
        ('w', SyntaxType::Constituent),
        ('x', SyntaxType::Constituent),
        ('y', SyntaxType::Constituent),
        ('z', SyntaxType::Constituent),
    ]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
