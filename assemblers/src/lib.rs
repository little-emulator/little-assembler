pub mod lc2;

use std::collections::HashMap;

#[allow(clippy::type_complexity)]
pub trait Assembler {
    type Address;

    /// # Errors
    ///
    /// This method returns an error if the assembly is not valid
    fn assemble(
        &self,
        assembly: &str,
    ) -> Result<(Vec<u8>, HashMap<String, Self::Address>), ParseError>;
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("Token is not valid")]
    NonValidToken,
    #[error("I was expecting more tokens")]
    NoMoreTokens,
    #[error("I was not expecting that token")]
    UnexpectedToken,

    #[error("The assembly should start with an \".orig\" directive")]
    NoOrig,
    #[error("The \".orig\" directives should be in order")]
    OutOfOrderOrigs,
    #[error("There should be only an \".orig\" directive")]
    TooManyOrig,

    #[error("The assembly should end with an \".end\" directive")]
    NoEnd,
    #[error("The \".end\" should be the last directive")]
    EndNotLast,

    #[error("The binary size exceeds the 65536 word limit")]
    BinaryTooBig,

    #[error("The number literal is too big")]
    NumberLiteralTooBig,

    #[error("The label was declarated more than once")]
    LabelRedeclaration,
}
