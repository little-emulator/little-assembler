use super::{ParseResult, Token};
use crate::ParseError;
use logos::Lexer;
use std::collections::HashMap;

pub trait TokenHelpers {
    fn peek_next_token(&mut self) -> ParseResult<&Token>;
    fn next_token(&mut self) -> ParseResult<Token>;
    fn next_token_skip(&mut self, skip: Token) -> ParseResult<Token>;
    fn get_register(&mut self) -> ParseResult<u16>;
    fn get_register_skip_comma(&mut self) -> ParseResult<u16>;
    fn get_index6(&mut self) -> ParseResult<u16>;
    fn get_pgoffset9(
        &mut self,
        symbol_table: Option<(&HashMap<String, u16>, u16)>,
        skip_comma: bool,
    ) -> ParseResult<u16>;
}

impl TokenHelpers for std::iter::Peekable<Lexer<'_, Token>> {
    /// Return a reference to the next token without consuming it
    fn peek_next_token(&mut self) -> ParseResult<&Token> {
        match self.peek() {
            Some(Ok(token)) => Ok(token),
            Some(Err(_error)) => Err(ParseError::NonValidToken),
            None => Err(ParseError::NoMoreTokens),
        }
    }

    /// Consume the next token
    fn next_token(&mut self) -> ParseResult<Token> {
        match self.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(_error)) => Err(ParseError::NonValidToken),
            None => Err(ParseError::NoMoreTokens),
        }
    }

    /// Consume the next token, skipping the `skip` Token if it's found
    fn next_token_skip(&mut self, skip: Token) -> ParseResult<Token> {
        match self.next_token() {
            Ok(token) if token == skip => self.next_token(),
            x => x,
        }
    }

    /// Consume the next register, returning it's 16 bit representation
    fn get_register(&mut self) -> ParseResult<u16> {
        match self.next_token()? {
            Token::Register(x) => Ok(u16::from(u8::from(x) & 0b111)),
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    /// Consume the next register, returning it's 16 bit representation and
    /// skipping a comma if it's found
    fn get_register_skip_comma(&mut self) -> ParseResult<u16> {
        match self.next_token_skip(Token::Comma)? {
            Token::Register(x) => Ok(u16::from(u8::from(x) & 0b111)),
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    /// Return the next 6 bit positive integer
    fn get_index6(&mut self) -> ParseResult<u16> {
        // Get the next 6 bit number
        Ok(match self.next_token_skip(Token::Comma)? {
            // If the next token is a number...
            Token::Number(number) => {
                // Check if it's negative
                if number >> 15 == 1 {
                    return Err(ParseError::NumberLiteralIsNegative);
                }
                // Check if it's bigger than 6 bit
                else if number >> 6 != 0 {
                    return Err(ParseError::NumberLiteralTooBig);
                }

                number
            }

            _ => return Err(ParseError::UnexpectedToken),
        })
    }

    /// Return the next 9 bit positive integer or the lower 9 bits of the next
    /// label
    fn get_pgoffset9(
        &mut self,
        symbol_table: Option<(&HashMap<String, u16>, u16)>,
        skip_comma: bool,
    ) -> ParseResult<u16> {
        // Get the next token
        let token = if skip_comma {
            self.next_token_skip(Token::Comma)?
        } else {
            self.next_token()?
        };

        // Check if it is a number or a label
        Ok(match token {
            // If it is a number...
            Token::Number(number) => {
                // Check if it's negative
                if number >> 15 == 1 {
                    return Err(ParseError::NumberLiteralIsNegative);
                }
                // Check if it's bigger than 9 bit
                else if number >> 9 != 0 {
                    return Err(ParseError::NumberLiteralTooBig);
                }

                number
            }

            // If it is a label...
            Token::Label(label) => {
                // Check if we are in the assembly phase. If the symbol table is not
                // available return 0
                let Some((symbol_table, address)) = symbol_table else {
                    return Ok(0);
                };

                // Check if the label is present in the symbol table
                match symbol_table.get(&label) {
                    // If it's not present return an error
                    None => return Err(ParseError::LabelNotDeclared),

                    // Else if it's present...
                    Some(label_address) => {
                        // Check if the label is on the same memory page as the
                        // instruction
                        if label_address & 0xfe00 != address & 0xfe00 {
                            return Err(ParseError::LabelNotOnSamePage);
                        }

                        // Return the last 9 bits of the label address
                        label_address & 0x1ff
                    }
                }
            }

            _ => return Err(ParseError::UnexpectedToken),
        })
    }
}
