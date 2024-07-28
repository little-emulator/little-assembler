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

    #[error("The assembly should contain an \".orig\" directive")]
    NoOrig,
    #[error("\".orig\" should be the first directive")]
    OrigNotFirst,
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
    #[error("The number literal must be positive")]
    NumberLiteralIsNegative,

    #[error("The label was declarated more than once")]
    LabelRedeclaration,
    #[error("The label was used but not declared")]
    LabelNotDeclared,
    #[error("The label was found but is not on the same memory page")]
    LabelNotOnSamePage,
}
