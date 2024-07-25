use super::*;

#[test]
fn normal() {
    let assembler = Lc2AssemblerBuilder::default().build().unwrap();

    let (binary, symbol_table) = assembler
        .assemble(
            r"
            .orig 0x3000
            .end
            ",
        )
        .unwrap();

    assert_eq!(binary, [0x30, 0x00]);
    assert!(symbol_table.is_empty());
}

#[test]
fn multiple_orig_error() {
    let assembler = Lc2AssemblerBuilder::default().build().unwrap();

    let error = assembler.assemble(
        r"
        .orig 0x3000
        .orig 0x6000
        .end
        ",
    );

    assert_eq!(error, Err(ParseError::TooManyOrig));
}

#[test]
fn no_orig_error() {
    let assembler = Lc2AssemblerBuilder::default().build().unwrap();

    let error = assembler.assemble(".end");

    assert_eq!(error, Err(ParseError::NoOrig));
}

#[test]
fn comments_before_orig() {
    let assembler = Lc2AssemblerBuilder::default().build().unwrap();

    let _ = assembler
        .assemble(
            r"
            ; This is a comment
            .orig 0x3000
            .end
            ",
        )
        .unwrap();
}

#[test]
fn orig_not_first_error() {
    let assembler = Lc2AssemblerBuilder::default().build().unwrap();

    let error = assembler.assemble(
        r"
        .fill 0x1234
        .orig 0x3000
        .end
        ",
    );

    assert_eq!(error, Err(ParseError::OrigNotFirst));
}

#[test]
fn no_end_error() {
    let assembler = Lc2AssemblerBuilder::default().build().unwrap();

    let error = assembler.assemble(".orig 0x3000");

    assert_eq!(error, Err(ParseError::NoEnd));
}

#[test]
fn end_not_last_error() {
    let assembler = Lc2AssemblerBuilder::default().build().unwrap();

    let error = assembler.assemble(
        r"
        .orig 0x3000
        .end
        .fill 0x1234
        ",
    );

    assert_eq!(error, Err(ParseError::EndNotLast));
}
