use super::*;

#[test]
fn multiple_orig() {
    let assembler = Lc2AssemblerBuilder::default()
        .multiple_origs(true)
        .build()
        .unwrap();

    let (binary, symbol_table) = assembler
        .assemble(
            r"
            .orig 0x3000
            .orig 0x3002
            .end
            ",
        )
        .unwrap();

    assert_eq!(binary, {
        let mut output = vec![0x30, 0x00]; // Start Address
        output.extend([0u8; 0x2 * 2]); // Difference between two `.orig`
        output
    });
    assert!(symbol_table.is_empty());
}

#[test]
fn no_orig() {
    let assembler = Lc2AssemblerBuilder::default()
        .optional_starting_orig(true)
        .build()
        .unwrap();

    let (binary, symbol_table) = assembler.assemble(".end").unwrap();

    assert_eq!(binary, []);
    assert!(symbol_table.is_empty());
}

#[test]
fn orig_not_first() {
    let assembler = Lc2AssemblerBuilder::default()
        .optional_starting_orig(true)
        .multiple_origs(true)
        .build()
        .unwrap();

    let (binary, symbol_table) = assembler
        .assemble(
            r"
            .fill 0x1234
            .orig 0x0004
            .end
            ",
        )
        .unwrap();

    assert_eq!(binary, {
        let mut output = vec![0x12, 0x34]; // `.fill` output
        output.extend([0u8; 0x3 * 2]); // Difference between the `.fill` and
                                       // the `.orig`
        output
    });
    assert!(symbol_table.is_empty());
}

#[test]
fn no_end() {
    let assembler = Lc2AssemblerBuilder::default()
        .optional_end(true)
        .build()
        .unwrap();

    let (binary, symbol_table) = assembler.assemble(".orig 0x3000").unwrap();

    assert_eq!(binary, [0x30, 0x00]);
    assert!(symbol_table.is_empty());
}

#[test]
fn end_not_last() {
    let assembler = Lc2AssemblerBuilder::default()
        .nothing_after_end(false)
        .build()
        .unwrap();

    let (binary, symbol_table) = assembler
        .assemble(
            r"
            .orig 0x3000
            .end
            .fill 0x1234
            ",
        )
        .unwrap();

    assert_eq!(binary, [0x30, 0x00]);
    assert!(symbol_table.is_empty());
}
