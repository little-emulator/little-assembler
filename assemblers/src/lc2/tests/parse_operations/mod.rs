use super::*;

mod normal;

fn assemble_error(assembly: &str) -> ParseError {
    Lc2AssemblerBuilder::default()
        .optional_starting_orig(true)
        .optional_end(true)
        .build()
        .unwrap()
        .assemble(assembly)
        .unwrap_err()
}

fn assemble(assembly: &str) -> Vec<u8> {
    let (binary, _symbol_table) = Lc2AssemblerBuilder::default()
        .optional_starting_orig(true)
        .optional_end(true)
        .build()
        .unwrap()
        .assemble(assembly)
        .unwrap();

    binary
}
