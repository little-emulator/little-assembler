use super::lexer::Token;
use crate::ParseError;
use logos::Lexer;
use std::{collections::HashMap, iter::Peekable};

pub fn build_symbol_table(
    _lexer: &mut Peekable<Lexer<'_, Token>>,
) -> Result<HashMap<String, u16>, ParseError> {
    todo!();
}

pub fn assemble(
    _lexer: &mut Peekable<Lexer<'_, Token>>,
    _symbol_table: &HashMap<String, u16>,
) -> Result<Vec<u8>, ParseError> {
    todo!();
}
