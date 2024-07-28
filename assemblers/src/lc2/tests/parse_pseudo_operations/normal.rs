use super::*;

fn assemble(assembly: &str) -> Vec<u8> {
    let (binary, _symbol_table) = Lc2AssemblerBuilder::default()
        .optional_starting_orig(true)
        .optional_end(true)
        .enable_stringzp(true)
        .build()
        .assemble(assembly)
        .unwrap();

    binary
}

#[test]
fn fill() {
    assert_eq!(assemble(".fill 0x1234"), [0x12, 0x34]);
}

#[test]
fn stringz() {
    assert_eq!(
        assemble(".stringz \"Test\""),
        [0, b'T', 0, b'e', 0, b's', 0, b't', 0, 0]
    );
}

#[test]
fn blkw() {
    assert_eq!(
        assemble(".blkw 10, 0xabcd"),
        [0xabcd_u16; 10]
            .iter()
            .flat_map(|x| x.to_be_bytes())
            .collect::<Vec<_>>()
    );
}

#[test]
fn end() {
    assert_eq!(assemble(".end"), []);
}

#[test]
fn stringzp() {
    assert_eq!(
        assemble(".stringzp \"Test\""),
        [b'e', b'T', b't', b's', 0, 0]
    );
}
