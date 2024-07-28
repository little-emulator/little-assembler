use super::*;

#[test]
fn pgoffset9_index6_imm5() {
    // pgoffset9 too big
    assert_eq!(assemble_error("BR 512"), ParseError::NumberLiteralTooBig);

    // pgoffset9 negative
    assert_eq!(assemble_error("BR -1"), ParseError::NumberLiteralIsNegative);

    // pgoffset9 from label
    assert_eq!(assemble(".orig 0x3010 label: BR label"), [0x0E, 0x10]);

    // index6 too big
    assert_eq!(
        assemble_error("JMPR R7, 64"),
        ParseError::NumberLiteralTooBig
    );

    // index6 negative
    assert_eq!(
        assemble_error("JMPR R7, -1"),
        ParseError::NumberLiteralIsNegative
    );

    // imm5 value too big
    assert_eq!(
        assemble_error("ADD R0, R0, 16"),
        ParseError::NumberLiteralTooBig
    );
    assert_eq!(
        assemble_error("ADD R0, R0, -17"),
        ParseError::NumberLiteralTooBig
    );
}

#[test]
fn add() {
    assert_eq!(assemble("ADD R0, R0, 5"), [0x10, 0x25]);
    assert_eq!(assemble("ADD R0, R0, R7"), [0x10, 0x07]);

    // Without commas
    assert_eq!(assemble("ADD R0 R0 5"), [0x10, 0x25]);
    assert_eq!(assemble("ADD R0 R0 R7"), [0x10, 0x07]);
}

#[test]
fn and() {
    assert_eq!(assemble("AND R0, R0, 5"), [0x50, 0x25]);
    assert_eq!(assemble("AND R0, R0, R7"), [0x50, 0x07]);

    // Without commas
    assert_eq!(assemble("AND R0 R0 5"), [0x50, 0x25]);
    assert_eq!(assemble("AND R0 R0 R7"), [0x50, 0x07]);
}

#[test]
fn branch() {
    // Branch n
    assert_eq!(assemble("BRn 10"), [0x08, 0x0A]);
    assert_eq!(assemble("BRlt 10"), [0x08, 0x0A]);

    // Branch z
    assert_eq!(assemble("BRz 10"), [0x04, 0x0A]);
    assert_eq!(assemble("BReq 10"), [0x04, 0x0A]);

    // Branch p
    assert_eq!(assemble("BRp 10"), [0x02, 0x0A]);
    assert_eq!(assemble("BRgt 10"), [0x02, 0x0A]);

    // Branch nz
    assert_eq!(assemble("BRnz 10"), [0x0C, 0x0A]);
    assert_eq!(assemble("BRle 10"), [0x0C, 0x0A]);

    // Branch np
    assert_eq!(assemble("BRnp 10"), [0x0A, 0x0A]);
    assert_eq!(assemble("BRne 10"), [0x0A, 0x0A]);

    // Branch zp
    assert_eq!(assemble("BRzp 10"), [0x06, 0x0A]);
    assert_eq!(assemble("BRge 10"), [0x06, 0x0A]);

    // Branch
    assert_eq!(assemble("BRnzp 10"), [0x0E, 0x0A]);
    assert_eq!(assemble("BR 10"), [0x0E, 0x0A]);

    // NOP
    assert_eq!(assemble("BRnop 10"), [0x00, 0x0A]);
    assert_eq!(assemble("NOP 10"), [0x00, 0x0A]);
}

#[test]
fn jump() {
    assert_eq!(assemble("JMP 10"), [0x40, 0x0A]);
    assert_eq!(assemble("JSR 10"), [0x48, 0x0A]);

    // Label
    assert_eq!(assemble(".orig 0x3010 label: JMP label"), [0x40, 0x10]);

    // Jump through Register
    assert_eq!(assemble("JMPR R7, 10"), [0xC1, 0xCA]);
    assert_eq!(assemble("JSRR R7, 10"), [0xC9, 0xCA]);

    // Jump through Register (Without commas)
    assert_eq!(assemble("JMPR R7 10"), [0xC1, 0xCA]);
    assert_eq!(assemble("JSRR R7 10"), [0xC9, 0xCA]);
}

#[test]
fn load() {
    assert_eq!(assemble("LD R5, 10"), [0x2A, 0x0A]);
    assert_eq!(assemble("LDI R7, 10"), [0xAE, 0x0A]);
    assert_eq!(assemble("LDR R6, R2, 8"), [0x6C, 0x88]);
    assert_eq!(assemble("LEA R3, 25"), [0xE6, 0x19]);

    // Without commas
    assert_eq!(assemble("LD R5 10"), [0x2A, 0x0A]);
    assert_eq!(assemble("LDI R7 10"), [0xAE, 0x0A]);
    assert_eq!(assemble("LDR R6 R2 8"), [0x6C, 0x88]);
    assert_eq!(assemble("LEA R3 25"), [0xE6, 0x19]);
}

#[test]
fn not() {
    assert_eq!(assemble("NOT R5, R6"), [0x9B, 0xBF]);

    // Without commas
    assert_eq!(assemble("NOT R5 R6"), [0x9B, 0xBF]);
}

#[test]
fn r#return() {
    assert_eq!(assemble("RET"), [0xD0, 0]);
    assert_eq!(assemble("RTI"), [0x80, 0]);
}

#[test]
fn store() {
    assert_eq!(assemble("ST R5, 10"), [0x3A, 0x0A]);
    assert_eq!(assemble("STI R7, 10"), [0xBE, 0x0A]);
    assert_eq!(assemble("STR R6, R2, 8"), [0x7C, 0x88]);

    // Without commas
    assert_eq!(assemble("ST R5 10"), [0x3A, 0x0A]);
    assert_eq!(assemble("STI R7 10"), [0xBE, 0x0A]);
    assert_eq!(assemble("STR R6 R2 8"), [0x7C, 0x88]);
}

#[test]
fn trap() {
    assert_eq!(assemble("TRAP 10"), [0xF0, 0x0A]);

    // Predefined syscalls
    assert_eq!(assemble("GETC"), [0xF0, 0x20]);
    assert_eq!(assemble("OUT"), [0xF0, 0x21]);
    assert_eq!(assemble("PUTS"), [0xF0, 0x22]);
    assert_eq!(assemble("IN"), [0xF0, 0x23]);
    assert_eq!(assemble("PUTSP"), [0xF0, 0x24]);
    assert_eq!(assemble("HALT"), [0xF0, 0x25]);
}
