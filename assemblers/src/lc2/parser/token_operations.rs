use super::{token_helpers::TokenHelpers, Operation, PseudoOperation, Token};
use crate::ParseError;
use logos::Lexer;
use std::collections::HashMap;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

pub trait TokenOperations {
    type Address;
    type Data;

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

impl TokenOperations for std::iter::Peekable<Lexer<'_, Token>> {
    type Address = u16;
    type Data = u16;

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
        // Loop to ignore start comments
        loop {
            match self.peek_next_token()? {
                // Ignore comments
                Token::Comment(_) => {
                    self.next_token()?;
                    continue;
                }

                // Get the address from the first `.orig` directive
                Token::PseudoOperation(PseudoOperation::Orig) => {
                    self.next_token()?;
                    return Ok(self.parse_pseudo_operation(PseudoOperation::Orig)?[0]);
                }

                // If the first token after the comments isn't a `.orig`
                // directive return an error, but if `orig_optional` is `true`
                // then return 0
                _ if orig_optional => return Ok(0),

                // If there is at least one `.orig` directive in the assembly
                // then return an `OrigNotFirst` error, else return a `NoOrig`
                // error
                _ => {
                    if self.any(|x| x == Ok(Token::PseudoOperation(PseudoOperation::Orig))) {
                        return Err(ParseError::OrigNotFirst);
                    }

                    return Err(ParseError::NoOrig);
                }
            }
        }
    }

    /// Consume a pseudo-operation, returning the binary representation
    fn parse_pseudo_operation(
        &mut self,
        pseudo_operation: PseudoOperation,
    ) -> ParseResult<Vec<Self::Data>> {
        Ok(match pseudo_operation {
            // For the `.orig` and the `.fill` directives return the number
            // immediatly after them
            PseudoOperation::Orig | PseudoOperation::Fill => {
                let Token::Number(start) = self.next_token()? else {
                    return Err(ParseError::UnexpectedToken);
                };

                vec![start]
            }

            // For the `.stringz` directive returns the next string followed by
            // a null byte
            PseudoOperation::Stringz => {
                let Token::String(mut string) = self.next_token()? else {
                    return Err(ParseError::UnexpectedToken);
                };

                string.push('\0');
                string.bytes().map(u16::from).collect()
            }

            // For the `.blkw` directive returns a the second number repeated
            // for the first number
            PseudoOperation::Blkw => {
                // Get how many times to repeat the next word
                let Token::Number(times) = self.next_token()? else {
                    return Err(ParseError::UnexpectedToken);
                };

                // Get the word to repeat...
                let word = match self.peek_next_token() {
                    // If the next token is a number, consume it and get it as
                    // the word to repeat
                    Ok(Token::Number(word)) => {
                        // Get the number
                        let word = *word;

                        // Consume it
                        self.next_token()?;

                        // Return it
                        word
                    }

                    // If the next token is a comma followed by a number, get it
                    // as the word to repeat
                    Ok(Token::Comma) => {
                        // Consume the comma
                        self.next_token()?;

                        // Get the next number
                        let Token::Number(word) = self.next_token()? else {
                            return Err(ParseError::UnexpectedToken);
                        };

                        // Return it
                        word
                    }

                    // Else fill the words with a null word
                    _ => 0,
                };

                vec![word; usize::from(times)]
            }

            // The `.end` directive doesn't have a binary representation
            PseudoOperation::End => Vec::new(),

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
                string
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
                    .collect()
            }
        })
    }

    /// Consume an operation, returning the binary representation
    fn parse_operation(
        &mut self,
        operation: Operation,
        symbol_table: Option<(&HashMap<String, u16>, Self::Address)>,
    ) -> ParseResult<Vec<Self::Data>> {
        Ok(match operation {
            Operation::Add | Operation::And => {
                // Get the opcode
                let opcode = if operation == Operation::And {
                    0b0101
                } else {
                    0b0001
                };

                let dest = self.get_register()?;
                let src1 = self.get_register_skip_comma()?;

                // Get the next register or a signed 5 bit number
                let src2 = match self.next_token_skip(Token::Comma)? {
                    // If the next token is a register than return it
                    Token::Register(x) => u16::from(u8::from(x) & 0b111),

                    // If the next token is a number...
                    Token::Number(number) => {
                        // Return an error if the number is bigger than 4 bits
                        // (sign excluded)
                        if number & 0xfff0 != 0 && number & 0xfff0 != 0xfff0 {
                            return Err(ParseError::NumberLiteralTooBig);
                        }

                        // Return the first 5 bits of the number with a 1
                        // before, to indicate that it's an immediate value
                        number & 0b11111 | 0b10_0000
                    }

                    _ => return Err(ParseError::UnexpectedToken),
                };

                vec![(opcode << 12) | (dest << 9) | (src1 << 6) | src2]
            }

            Operation::Branch(n, z, p) => {
                vec![
                    (u16::from(n) << 11)
                        | (u16::from(z) << 10)
                        | (u16::from(p) << 9)
                        | self.get_pgoffset9(symbol_table, false)?,
                ]
            }

            Operation::Jump(link) => {
                vec![
                    (0b0100 << 12)
                        | (u16::from(link) << 11)
                        | self.get_pgoffset9(symbol_table, false)?,
                ]
            }

            Operation::JumpRegister(link) => {
                vec![
                    (0b1100 << 12)
                        | (u16::from(link) << 11)
                        | (self.get_register()? << 6)
                        | self.get_index6()?,
                ]
            }

            Operation::Load
            | Operation::LoadIndirect
            | Operation::LoadEffectiveAddress
            | Operation::Store
            | Operation::StoreIndirect => {
                // Get the opcode
                let opcode = match operation {
                    Operation::Load => 0b0010,
                    Operation::LoadIndirect => 0b1010,
                    Operation::LoadEffectiveAddress => 0b1110,
                    Operation::Store => 0b0011,
                    Operation::StoreIndirect => 0b1011,
                    _ => unreachable!(),
                };

                vec![
                    (opcode << 12)
                        | (self.get_register()? << 9)
                        | self.get_pgoffset9(symbol_table, true)?,
                ]
            }

            Operation::LoadRegister | Operation::StoreRegister => {
                // Get the opcode
                let opcode = if operation == Operation::LoadRegister {
                    0b0110
                } else {
                    0b0111
                };

                vec![
                    (opcode << 12)
                        | (self.get_register()? << 9)
                        | (self.get_register_skip_comma()? << 6)
                        | self.get_index6()?,
                ]
            }

            Operation::Not => {
                vec![
                    (0b1001 << 12)
                        | (self.get_register()? << 9)
                        | (self.get_register_skip_comma()? << 6)
                        | 0b11_1111,
                ]
            }

            Operation::Return => vec![0b1101_000000000000],
            Operation::ReturnInterrupt => vec![0b1000_000000000000],

            Operation::Trap(index) => {
                // If the index is set, use it, else get the next 8 bit number
                let index: u8 = match index {
                    Some(x) => x,
                    None => match self.next_token()? {
                        Token::Number(x) => {
                            u8::try_from(x).map_err(|_| ParseError::NumberLiteralTooBig)?
                        }

                        _ => return Err(ParseError::UnexpectedToken),
                    },
                };

                vec![(0b1111 << 12) | u16::from(index)]
            }
        })
    }
}
