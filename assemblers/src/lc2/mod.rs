mod lexer;

use crate::ParseError;
use logos::Logos;
use std::collections::HashMap;

#[derive(Debug, derive_builder::Builder)]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::struct_excessive_bools)]
pub struct Lc2Assembler {
    #[builder(default = "false")]
    optional_end: bool,

    #[builder(default = "true")]
    nothing_after_end: bool,

    #[builder(default = "false")]
    optional_orig: bool,

    #[builder(default = "false")]
    multiple_origs: bool,

    #[builder(default = "false")]
    enable_stringzp: bool,
}

impl crate::Assembler for Lc2Assembler {
    type Address = u16;

    fn assemble(
        &self,
        assembly: &str,
    ) -> Result<(Vec<u8>, HashMap<String, Self::Address>), ParseError> {
        let _lexer = lexer::Token::lexer(assembly);

        todo!()
    }
}
