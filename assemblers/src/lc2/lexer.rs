use crate::ParseError;
use architectures::lc2::Gpr;
use logos::{Lexer, Logos};

#[derive(Clone, Logos, Debug, PartialEq, Eq)]
#[logos(skip r"[\s]+")]
pub enum Token {
    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[regex(r"(;|//).*", |lex| lex.slice().to_string())]
    Comment(String),

    #[regex(r"[rR][0-7]", parse_register)]
    Register(Gpr),

    #[regex(r"[_a-zA-Z]+", |lex| lex.slice().to_owned())]
    Label(String),

    #[regex(r"(?i)(0?b|%)[01]+", |lex| parse_number(lex, 2))]
    #[regex(r"#?\d+", |lex| parse_number(lex, 10))]
    #[regex(r"(?i)(0?x|\$)[a-f0-9]+", |lex| parse_number(lex, 16))]
    Number(u16),

    // Inspired by https://logos.maciej.codes/examples/json.html
    #[regex(r#""([^"\\]|\\["\\0nrt])*""#, parse_string)]
    String(String),

    #[regex(r"(?i)\.(ORIG|FILL|STRINGZ|BLKW|END)", |lex| PseudoOperation::try_from(lex.slice()).ok())]
    PseudoOperation(PseudoOperation),

    #[regex(r"(?i)ADD|AND|JSRR?|JMPR?|LD[IR]?|LEA|NOT|RET|RTI|ST[IR]?", |lex| Operation::try_from(lex.slice()).ok())]
    #[regex(r"(?i)BRn?z?p?|BR[gl][te]|BR(eq|ne)|(BR)?nop", |lex| Operation::try_from(lex.slice()).ok())]
    #[regex(r"(?i)TRAP|GETC|OUT|PUTSP?|IN|HALT", |lex| Operation::try_from(lex.slice()).ok())]
    Operation(Operation),
}

fn parse_register(lex: &Lexer<Token>) -> Option<Gpr> {
    // Convert the second char of the register into a usize
    let register_number: usize = lex.slice().chars().nth(1)?.to_digit(10)?.try_into().ok()?;

    // Convert the usize into a General Purpose Register
    Gpr::try_from(register_number).ok()
}

fn parse_string(lex: &Lexer<Token>) -> Option<String> {
    // Remove the starting and ending double quotes and replace all the escaped
    // characters
    Some(
        lex.slice()
            .strip_prefix('"')?
            .strip_suffix('"')?
            .replace("\\\"", "\"")
            .replace("\\\\", "\\")
            .replace("\\0", "\0")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t"),
    )
}

fn parse_number(lex: &Lexer<Token>, base: u32) -> Option<u16> {
    // Get the possible prefixes for any given base
    let prefixes: &[char] = match base {
        2 => &['b', '%'],
        10 => &['#'],
        16 => &['x', '$'],
        _ => return None,
    };

    // Remove the prefixes
    let str = lex.slice().to_lowercase().replace(prefixes, "");

    // Convert the string into a u16
    u16::from_str_radix(&str, base).ok()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Add,
    And,
    Branch(bool, bool, bool),
    Jump(bool),
    JumpRegister(bool),
    Load,
    LoadIndirect,
    LoadRegister,
    LoadEffectiveAddress,
    Not,
    Return,
    ReturnInterrupt,
    Store,
    StoreIndirect,
    StoreRegister,
    Trap(Option<u8>),
}

impl TryFrom<&str> for Operation {
    type Error = ParseError;

    fn try_from(token: &str) -> Result<Self, Self::Error> {
        Ok(match token.to_uppercase().as_str() {
            "ADD" => Self::Add,
            "AND" => Self::And,
            "BRN" | "BRLT" => Self::Branch(true, false, false),
            "BRZ" | "BREQ" => Self::Branch(false, true, false),
            "BRP" | "BRGT" => Self::Branch(false, false, true),
            "BRNZ" | "BRLE" => Self::Branch(true, true, false),
            "BRNP" | "BRNE" => Self::Branch(true, false, true),
            "BRZP" | "BRGE" => Self::Branch(false, true, true),
            "BRNZP" | "BR" => Self::Branch(true, true, true),
            "BRNOP" | "NOP" => Self::Branch(false, false, false),
            "JSR" => Self::Jump(true),
            "JMP" => Self::Jump(false),
            "JSRR" => Self::JumpRegister(true),
            "JMPR" => Self::JumpRegister(false),
            "LD" => Self::Load,
            "LDI" => Self::LoadIndirect,
            "LDR" => Self::LoadRegister,
            "LEA" => Self::LoadEffectiveAddress,
            "NOT" => Self::Not,
            "RET" => Self::Return,
            "RTI" => Self::ReturnInterrupt,
            "ST" => Self::Store,
            "STI" => Self::StoreIndirect,
            "STR" => Self::StoreRegister,

            "TRAP" => Self::Trap(None),
            "GETC" => Self::Trap(Some(0x20)),
            "OUT" => Self::Trap(Some(0x21)),
            "PUTS" => Self::Trap(Some(0x22)),
            "IN" => Self::Trap(Some(0x23)),
            "PUTSP" => Self::Trap(Some(0x24)),
            "HALT" => Self::Trap(Some(0x25)),

            _ => return Err(ParseError::NonValidToken),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PseudoOperation {
    Orig,
    Fill,
    Stringz,
    Blkw,
    End,
}

impl TryFrom<&str> for PseudoOperation {
    type Error = ParseError;

    fn try_from(token: &str) -> Result<Self, Self::Error> {
        Ok(match token.to_uppercase().as_str() {
            ".ORIG" => Self::Orig,
            ".FILL" => Self::Fill,
            ".STRINGZ" => Self::Stringz,
            ".BLKW" => Self::Blkw,
            ".END" => Self::End,

            _ => return Err(ParseError::NonValidToken),
        })
    }
}