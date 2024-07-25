mod token_operations;

use super::{
    lexer::{Operation, PseudoOperation, Token},
    Lc2Assembler,
};
use crate::ParseError;
use logos::Lexer;
use std::{collections::HashMap, iter::Peekable};
use token_operations::{ParseResult, TokenOperations};

/// The main purpose of this function is to scan the whole assembly in search
/// of labels declarations, saving them into a `HashMap` alongside the address
/// they're pointing to.
///
/// Other than that, it validates the assembly, asserting that:
///   - The `.orig` pseudo-operation is the first directive (unless
///     `options.optional_starting_orig` is set to `true`);
///   - There is only one `.orig` directive (unless `options.multiple_origs` is
///     set to `true`);
///   - If there are multiple `.orig` directives, they are in order;
///   - If `options.enable_stringzp` is set to `true`, then it enables the
///     pseudo-operation `.stringzp` to create a null-terminated packed string;
///   - The `.end` pseudo-operation is the last directive (unless
///     `options.optional_end` is set to `true`);
///   - There aren't any more tokens after the `.end` directive (unless
///     `options.nothing_after_end` is set to `false`);
///   - The binary doesn't exceed the maximum size.
///
/// The `assemble()` function assumes that those check are done
pub fn build_symbol_table(
    options: &Lc2Assembler,
    lexer: &mut Peekable<Lexer<'_, Token>>,
) -> ParseResult<HashMap<String, u16>> {
    // Create an empty symbol table
    let mut symbol_table = HashMap::new();

    // Get the start_address
    log::trace!("Getting the start address...");
    let mut address = lexer.parse_start_address(options.optional_starting_orig)?;
    log::debug!("Start address is {:#06x}!", address);

    // For every token...
    let mut end = false;
    while let Some(token) = lexer.next() {
        let token = token.map_err(|()| ParseError::NonValidToken)?;
        log::debug!("Got a new token: {:?}!", token);

        // Get the lenght of the instruction
        let instruction_lenght: u16 = match token {
            // Skip the comments
            Token::Comment(_) => continue,

            // Add the labels declaration into the symbol table
            Token::Label(label) => {
                log::info!(
                    "Adding the label \"{}\" at address {:#06x} to the symbol table...",
                    label,
                    address
                );

                // Return an error if the label was already defined
                if symbol_table.insert(label, address).is_some() {
                    return Err(ParseError::LabelRedeclaration);
                }

                // Skip the trailing colon, if there is one, and skip to the
                // next token
                lexer.skip_token(Token::Colon)?;
                continue;
            }

            // In the LC2 architecture every instruction is exactly 1 word, so
            // add 1 to the address
            Token::Operation(x) => {
                lexer.parse_operation(x, None)?;
                1
            }

            // If there is another `.orig` directive and the
            // `options.multiple_origs` is set then jump to the new address
            Token::PseudoOperation(PseudoOperation::Orig) => {
                if !options.multiple_origs {
                    return Err(ParseError::TooManyOrig);
                }

                lexer.parse_pseudo_operation(PseudoOperation::Orig)?[0]
                    .checked_sub(address)
                    .ok_or(ParseError::OutOfOrderOrigs)?
            }

            // If there is a `.end` directive, exit from the loop
            #[allow(unused_assignments)]
            Token::PseudoOperation(PseudoOperation::End) => {
                end = true;
                break;
            }

            // If the `.stringzp` directive is not enabled return an error
            Token::PseudoOperation(PseudoOperation::Stringzp) if !options.enable_stringzp => {
                return Err(ParseError::NonValidToken);
            }

            // Get pseudo-operations lenght
            Token::PseudoOperation(x) => u16::try_from(lexer.parse_pseudo_operation(x)?.len())
                .map_err(|_| ParseError::BinaryTooBig)?,

            _ => return Err(ParseError::UnexpectedToken),
        };

        log::trace!(
            "Incrementing the addess by {} cell{}...",
            instruction_lenght,
            if instruction_lenght == 1 { "" } else { "s" }
        );

        // Update the address
        address = address
            .checked_add(instruction_lenght)
            .ok_or(ParseError::BinaryTooBig)?;

        log::trace!("The new address is {:#06x}!", address);
    }

    // If there wasn't any `.end` directive and the `options.optional_end` isn't
    // set then return an error
    if !end && !options.optional_end {
        return Err(ParseError::NoEnd);
    }

    // If there was an `.end` directive but it was't the last token and the
    // `options.nothing_after_end` isn't set then return an error
    if !options.optional_end && lexer.next().is_some() && options.nothing_after_end {
        return Err(ParseError::EndNotLast);
    }

    // Return the symbol table
    Ok(symbol_table)
}

/// This function takes the assembly and the symbol table and converts them into
/// the final binary.
///
/// It doesn't perform any kind of check except the essential ones, so the
/// `build_symbol_table()` function needs to be run first
pub fn assemble(
    options: &Lc2Assembler,
    lexer: &mut Peekable<Lexer<'_, Token>>,
    symbol_table: &HashMap<String, u16>,
) -> ParseResult<Vec<u8>> {
    // Get the start_address
    log::trace!("Getting the start address...");
    let mut address = lexer.parse_start_address(options.optional_starting_orig)?;
    log::debug!("Start address is {:#06x}!", address);

    // Create a new binary and put the start address into it if
    // `options.prepend_start_address` is set
    let mut binary = Vec::new();
    if !options.optional_starting_orig && options.prepend_start_address {
        log::debug!("Putting the start_address into the binary!");
        binary.push(address);
    }

    // For every token...
    while let Some(token) = lexer.next() {
        let token = token.map_err(|()| ParseError::NonValidToken)?;
        log::debug!("Got a new token: {:?}!", token);

        // Get the binary representation of the instruction
        let instruction: Vec<u16> = match token {
            // Skip comments and labels
            Token::Comment(_) => continue,
            Token::Label(_) => {
                lexer.skip_token(Token::Colon)?;
                continue;
            }

            // Parse operations
            Token::Operation(x) => lexer.parse_operation(x, Some((symbol_table, address)))?,

            // If there is a new `.orig` directive, add new empty cells until
            // the new address is reached
            Token::PseudoOperation(PseudoOperation::Orig) => {
                let new_address = lexer.parse_pseudo_operation(PseudoOperation::Orig)?[0];
                vec![0; usize::from(new_address - address)]
            }

            // Parse pseudo-operations. Exit the loop if it encounters a `.end`
            // directive
            Token::PseudoOperation(PseudoOperation::End) => break,
            Token::PseudoOperation(x) => lexer.parse_pseudo_operation(x)?,

            _ => return Err(ParseError::UnexpectedToken),
        };

        log::trace!(
            "Incrementing the binary by {} cell{}...",
            instruction.len(),
            if instruction.len() == 1 { "" } else { "s" }
        );

        // Update the address and extend the binary
        address += u16::try_from(instruction.len()).map_err(|_| ParseError::BinaryTooBig)?;
        binary.extend(instruction);

        log::trace!("The new binary is {} bytes long!", binary.len() * 2);
    }

    // Convert the vector of words into a vector of bytes and return it
    Ok(binary.iter().flat_map(|x| x.to_be_bytes()).collect())
}
