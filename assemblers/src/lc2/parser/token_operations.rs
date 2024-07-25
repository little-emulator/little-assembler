use super::{Operation, PseudoOperation, Token};
use crate::ParseError;
use logos::Lexer;
use std::{collections::HashMap, iter::Peekable};

pub type ParseResult<T> = std::result::Result<T, ParseError>;

pub trait TokenOperations {
    type Address;
    type Data;

    fn peek_next_token(&mut self) -> ParseResult<&Token>;
    fn next_token(&mut self) -> ParseResult<Token>;
    fn next_token_skip(&mut self, skip: Token) -> ParseResult<Token>;
    fn skip_token(&mut self, skip: Token) -> ParseResult<()>;
    fn parse_start_address(&mut self, orig_optional: bool) -> ParseResult<u16>;
    fn parse_pseudo_operation(
        &mut self,
        pseudo_operation: PseudoOperation,
    ) -> ParseResult<Vec<Self::Data>>;
    fn parse_operation(
        &mut self,
        operation: Operation,
        symbol_table: Option<(&HashMap<String, u16>, Self::Address)>,
    ) -> ParseResult<Vec<Self::Data>>;
}

impl TokenOperations for Peekable<Lexer<'_, Token>> {
    type Address = u16;
    type Data = u16;

    /// Return a reference to the next token without consuming it
    fn peek_next_token(&mut self) -> ParseResult<&Token> {
        match self.peek() {
            Some(Ok(token)) => Ok(token),
            Some(Err(_error)) => Err(ParseError::NonValidToken),
            None => Err(ParseError::NoMoreTokens),
        }
    }

    /// Return a the next token
    fn next_token(&mut self) -> ParseResult<Token> {
        match self.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(_error)) => Err(ParseError::NonValidToken),
            None => Err(ParseError::NoMoreTokens),
        }
    }

    /// Return a the next token, skipping the `skip` Token if it's found
    fn next_token_skip(&mut self, skip: Token) -> ParseResult<Token> {
        match self.next_token() {
            Ok(token) if token == skip => self.next_token(),
            x => x,
        }
    }

    /// Skips the `skip` Token if it's found
    fn skip_token(&mut self, skip: Token) -> ParseResult<()> {
        if self.peek_next_token()? == &skip {
            self.next_token()?;
        }
        Ok(())
    }

    /// Get the start address from the first `.orig` directive. If 
    /// `orig_optional` is set and the first directive isn't an `.orig` return
    /// `0`
    fn parse_start_address(&mut self, orig_optional: bool) -> ParseResult<u16> {
        Ok(match self.peek_next_token()? {
            Token::PseudoOperation(PseudoOperation::Orig) => {
                self.next_token()?;
                self.parse_pseudo_operation(PseudoOperation::Orig)?[0]
            }
            _ if orig_optional => 0,
            _ => return Err(ParseError::NoOrig),
        })
    }

    /// Consume a pseudo-operation, returning the binary representation
    fn parse_pseudo_operation(
        &mut self,
        pseudo_operation: PseudoOperation,
    ) -> ParseResult<Vec<Self::Data>> {
        match pseudo_operation {
            // For the `.orig` and the `.fill` directives return the number
            // immediatly after them
            PseudoOperation::Orig | PseudoOperation::Fill => {
                let Token::Number(start) = self.next_token()? else {
                    return Err(ParseError::UnexpectedToken);
                };

                Ok(vec![start])
            }

            // For the `.stringz` directive returns the next string followed by
            // a null byte
            PseudoOperation::Stringz => {
                let Token::String(mut string) = self.next_token()? else {
                    return Err(ParseError::UnexpectedToken);
                };

                string.push('\0');
                Ok(string.bytes().map(u16::from).collect())
            }

            // For the `.blkw` directive returns a the second number repeated
            // for the first number
            PseudoOperation::Blkw => {
                // Get how many times to repeat the next word
                let Token::Number(times) = self.next_token()? else {
                    return Err(ParseError::UnexpectedToken);
                };

                // Get the word to repeat, skipping the comma if there is one
                let Token::Number(word) = self.next_token_skip(Token::Comma)? else {
                    return Err(ParseError::UnexpectedToken);
                };

                Ok(vec![word; usize::from(times)])
            }

            // The `.end` directive doesn't have a binary representation
            PseudoOperation::End => Ok(Vec::new()),

            // For the `.stringzp` custom directive returns the next string
            // followed by a null byte in a packed form.
            //
            // A packed string is a string in which a single word of memory
            // contains two characters: the first one in the lower byte and the
            // second one in the high byte.
            PseudoOperation::Stringzp => {
                let Token::String(mut string) = self.next_token()? else {
                    return Err(ParseError::UnexpectedToken);
                };

                string.push('\0');
                Ok(string
                    .into_bytes()
                    .chunks(2)
                    .map(|bytes| {
                        // Put the low byte into the data word
                        let mut data = u16::from(bytes[0]);

                        // Put the high byte into the data word, if it exists
                        if bytes.len() == 2 {
                            data |= u16::from(bytes[1]) << 8;
                        }

                        // Return the packed characters
                        data
                    })
                    .collect())
            }
        }
    }

    /// Consume an operation, returning the binary representation
    fn parse_operation(
        &mut self,
        _operation: Operation,
        _symbol_table: Option<(&HashMap<String, u16>, Self::Address)>,
    ) -> ParseResult<Vec<Self::Data>> {
        todo!()
    }
}
