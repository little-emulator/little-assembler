use super::*;
use std::collections::HashMap;

fn assemble(assembly: &str) -> Result<(Vec<u8>, HashMap<String, u16>), ParseError> {
    Lc2AssemblerBuilder::default()
        .optional_starting_orig(true)
        .optional_end(true)
        .enable_stringzp(true)
        .build()
        .unwrap()
        .assemble(assembly)
}

#[test]
fn fill() {
    assert_eq!(
        assemble(".fill \"Unexpected\" 0x1234"),
        Err(ParseError::UnexpectedToken)
    );
}

#[test]
fn stringz() {
    assert_eq!(
        assemble(".stringz 0x1234 \"Test\""),
        Err(ParseError::UnexpectedToken)
    );
}

#[test]
fn blkw_1() {
    assert_eq!(
        assemble(".blkw \"Unexpected\" 10, 0xabcd"),
        Err(ParseError::UnexpectedToken)
    );
}

#[test]
fn blkw_2() {
    assert_eq!(
        assemble(".blkw 10, \"Unexpected\" 0xabcd"),
        Err(ParseError::UnexpectedToken)
    );
}

#[test]
fn stringzp() {
    assert_eq!(
        assemble(".stringzp 0x1234"),
        Err(ParseError::UnexpectedToken)
    );
}

#[test]
fn stringzp_not_enabled() {
    let assembler = Lc2AssemblerBuilder::default()
        .optional_starting_orig(true)
        .optional_end(true)
        .build()
        .unwrap();

    assert_eq!(
        assembler.assemble(".stringzp \"Test\""),
        Err(ParseError::NonValidToken)
    );
}
