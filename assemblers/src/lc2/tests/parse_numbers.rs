use crate::lc2::lexer::Token;
use logos::Logos;

#[test]
fn decimal() {
    assert_eq!(Token::lexer("5").next().unwrap(), Ok(Token::Number(5)));
    assert_eq!(Token::lexer("#5").next().unwrap(), Ok(Token::Number(5)));
    assert_eq!(
        Token::lexer("-5").next().unwrap(),
        Ok(Token::Number(!5 + 1))
    );
    assert_eq!(
        Token::lexer("#-5").next().unwrap(),
        Ok(Token::Number(!5 + 1))
    );

    // Zero
    assert_eq!(Token::lexer("0").next().unwrap(), Ok(Token::Number(0)));
    assert_eq!(Token::lexer("#0").next().unwrap(), Ok(Token::Number(0)));
    assert_eq!(Token::lexer("-0").next().unwrap(), Ok(Token::Number(0)));
    assert_eq!(Token::lexer("#-0").next().unwrap(), Ok(Token::Number(0)));

    // u16::MAX
    assert_eq!(
        Token::lexer("65535").next().unwrap(),
        Ok(Token::Number(65535))
    );
    assert_eq!(
        Token::lexer("#65535").next().unwrap(),
        Ok(Token::Number(65535))
    );
    assert_eq!(Token::lexer("-65535").next().unwrap(), Ok(Token::Number(1)));
    assert_eq!(
        Token::lexer("#-65535").next().unwrap(),
        Ok(Token::Number(1))
    );

    // u16::MAX + 1
    assert_eq!(Token::lexer("65536").next().unwrap(), Err(()));
    assert_eq!(Token::lexer("#65536").next().unwrap(), Err(()));
    assert_eq!(Token::lexer("-65536").next().unwrap(), Err(()));
    assert_eq!(Token::lexer("#-65536").next().unwrap(), Err(()));
}

#[test]
fn binary() {
    assert_eq!(
        Token::lexer("0b1010").next().unwrap(),
        Ok(Token::Number(10))
    );
    assert_eq!(Token::lexer("b1010").next().unwrap(), Ok(Token::Number(10)));
    assert_eq!(Token::lexer("%1010").next().unwrap(), Ok(Token::Number(10)));

    // Zero
    assert_eq!(Token::lexer("0b0").next().unwrap(), Ok(Token::Number(0)));
    assert_eq!(Token::lexer("b0").next().unwrap(), Ok(Token::Number(0)));
    assert_eq!(Token::lexer("%0").next().unwrap(), Ok(Token::Number(0)));

    // u16::MAX
    assert_eq!(
        Token::lexer("0b1111111111111111").next().unwrap(),
        Ok(Token::Number(65535))
    );
    assert_eq!(
        Token::lexer("b1111111111111111").next().unwrap(),
        Ok(Token::Number(65535))
    );
    assert_eq!(
        Token::lexer("%1111111111111111").next().unwrap(),
        Ok(Token::Number(65535))
    );

    // u16::MAX + 1
    assert_eq!(Token::lexer("0b10000000000000000").next().unwrap(), Err(()));
    assert_eq!(Token::lexer("b10000000000000000").next().unwrap(), Err(()));
    assert_eq!(Token::lexer("%10000000000000000").next().unwrap(), Err(()));
}

#[test]
fn hexadecimal() {
    assert_eq!(Token::lexer("0x10").next().unwrap(), Ok(Token::Number(16)));
    assert_eq!(Token::lexer("x10").next().unwrap(), Ok(Token::Number(16)));
    assert_eq!(Token::lexer("$10").next().unwrap(), Ok(Token::Number(16)));

    // Zero
    assert_eq!(Token::lexer("0x0").next().unwrap(), Ok(Token::Number(0)));
    assert_eq!(Token::lexer("x0").next().unwrap(), Ok(Token::Number(0)));
    assert_eq!(Token::lexer("$0").next().unwrap(), Ok(Token::Number(0)));

    // u16::MAX
    assert_eq!(
        Token::lexer("0xffff").next().unwrap(),
        Ok(Token::Number(65535))
    );
    assert_eq!(
        Token::lexer("xffff").next().unwrap(),
        Ok(Token::Number(65535))
    );
    assert_eq!(
        Token::lexer("$ffff").next().unwrap(),
        Ok(Token::Number(65535))
    );

    // u16::MAX + 1
    assert_eq!(Token::lexer("0x10000").next().unwrap(), Err(()));
    assert_eq!(Token::lexer("x10000").next().unwrap(), Err(()));
    assert_eq!(Token::lexer("$10000").next().unwrap(), Err(()));
}
