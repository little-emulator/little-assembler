use super::*;

#[test]
fn get_labels() {
    let assembler = Lc2AssemblerBuilder::default().build();

    let (binary, symbol_table) = assembler
        .assemble(
            r"
            .orig 0x3000

            label_1:
            label_2:
              .fill 0x1234

            label_3 .fill 0x5678

            .end
            ",
        )
        .unwrap();

    assert_eq!(binary, [0x30, 0x00, 0x12, 0x34, 0x56, 0x78]);
    assert_eq!(symbol_table.get("label_1"), Some(&0x3000));
    assert_eq!(symbol_table.get("label_2"), Some(&0x3000));
    assert_eq!(symbol_table.get("label_3"), Some(&0x3001));
}

#[test]
fn label_redeclaration() {
    let assembler = Lc2AssemblerBuilder::default().build();

    let error = assembler.assemble(
        r"
            .orig 0x3000

            label:
              .fill 0x1234

            label .fill 0x5678

            .end
            ",
    );

    assert_eq!(error, Err(ParseError::LabelRedeclaration));
}
