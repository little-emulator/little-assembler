#[cfg(test)]
mod tests;

mod lexer;
mod parser;

use crate::ParseError;
use logos::Logos;
use std::collections::HashMap;

#[allow(clippy::module_name_repetitions)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, derive_builder::Builder)]
#[builder(build_fn(private, name = "fallible_build"))]
pub struct Lc2Assembler {
    #[builder(default = "false")]
    optional_starting_orig: bool,

    #[builder(default = "false")]
    multiple_origs: bool,

    #[builder(default = "false")]
    optional_end: bool,

    #[builder(default = "true")]
    nothing_after_end: bool,

    #[builder(default = "false")]
    enable_stringzp: bool,

    #[builder(default = "true")]
    prepend_start_address: bool,
}

// https://github.com/colin-kiegel/rust-derive-builder/issues/56#issuecomment-1043671602
impl Lc2AssemblerBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// # Panics
    ///
    /// This method panics if if any fields have been added to `Lc2Assembler`
    /// that lack defaults
    #[must_use]
    pub fn build(&mut self) -> Lc2Assembler {
        self.fallible_build()
            .expect("All required fields set at initialization")
    }
}

impl crate::Assembler for Lc2Assembler {
    type Address = u16;

    fn assemble(
        &self,
        assembly: &str,
    ) -> Result<(Vec<u8>, HashMap<String, Self::Address>), ParseError> {
        // Lexer
        log::info!(target: "lc2_assembler", "Tokenizing the assembly...");
        let mut lexer = lexer::Token::lexer(assembly).peekable();

        // Parser
        log::info!(target: "lc2_assembler", "Creating the symbol table...");
        let symbol_table = parser::build_symbol_table(self, &mut lexer.clone())?;
        log::info!(target: "lc2_assembler", "Assembling the binary...");
        let binary = parser::assemble(self, &mut lexer, &symbol_table)?;

        Ok((binary, symbol_table))
    }
}
