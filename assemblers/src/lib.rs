pub mod lc2;

use std::collections::HashMap;

#[allow(clippy::type_complexity)]
pub trait Assembler {
    type Address;

    /// # Errors
    ///
    /// This method errors if the provided assembly is not valid
    fn assemble(
        &self,
        assembly: &str,
    ) -> Result<(Vec<u8>, HashMap<String, Self::Address>), ParseError>;
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Token is not valid")]
    NonValidToken,
    #[error("I was expecting more Tokens")]
    NoMoreTokens,
    #[error("I was not expecting that Token")]
    UnexpectedToken,
    #[error("The assembly should start with an \".orig\" directive")]
    NoOrig,
    #[error("The assembly should end with an \".end\" directive")]
    NoEnd,
    #[error("There should be only an \".orig\" directive")]
    TooManyOrig,
    #[error("The \".end\" should be the last directive")]
    EndNotLast,
    #[error("The Number Literal is too big")]
    NumberLiteralTooBig,
    #[error("The label was declarated more than once")]
    LabelRedeclaration,
}
