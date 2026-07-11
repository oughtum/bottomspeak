#![cfg(test)]

use indoc::indoc;
use pretty_assertions::assert_eq;

use super::*;

fn lex(source: &str) -> crate::Result<TokenStream> {
    let mut ctx = SourceContext::new(source, Vec::new(), "<test>")?;

    let mut lexer = Lexer::new(&mut ctx);
    lexer.lex_tokens(true);
    Ok(lexer.tokens())
}

#[test]
fn test_lexing() -> crate::Result<()> {
    let source = indoc! {r#"
        haiii 🥺
            afogjqwefokadj;flkadjsf;lkad :333 >////< >w< 🏳️‍⚧️ [23, 23, 23]
            :333 mreow 🏳️‍⚧️ [] "E"
            adlkadlfdla >//< :33 mrrp 🏳️‍⚧️ [] "\n"
            >.<
    "#};
    let ts = lex(source)?;

    assert_eq!(
        ts.inner(),
        &vec![
            Token::new(
                TokenType::Keysmash {
                    tilde: false,
                    lowercase: true,
                    len: 5
                },
                "haiii",
                0..5
            ),
            Token::new(TokenType::Sub, "🥺", 6..10),
            Token::new(
                TokenType::Keysmash {
                    tilde: false,
                    lowercase: true,
                    len: 14
                },
                "afogjqwefokadj",
                15..29
            ),
            Token::new(
                TokenType::Keysmash {
                    tilde: false,
                    lowercase: true,
                    len: 8
                },
                "flkadjsf",
                30..38
            ),
            Token::new(
                TokenType::Keysmash {
                    tilde: false,
                    lowercase: true,
                    len: 4
                },
                "lkad",
                39..43
            ),
            Token::new(TokenType::ColonThree { add: true, len: 3 }, ":333", 44..48),
            Token::new(
                TokenType::Blush {
                    double: false,
                    len: 2
                },
                ">////<",
                49..55
            ),
            Token::new(TokenType::FlusteredW, ">w<", 56..59),
            Token::new(TokenType::ColonThree { add: true, len: 3 }, ":333", 94..98),
            Token::new(
                TokenType::Print {
                    kind: PrintKind::Normal
                },
                "mreow",
                99..104
            ),
            Token::new(
                TokenType::Keysmash {
                    tilde: false,
                    lowercase: true,
                    len: 11
                },
                "adlkadlfdla",
                133..144
            ),
            Token::new(
                TokenType::Blush {
                    double: false,
                    len: 1
                },
                ">//<",
                145..149
            ),
            Token::new(TokenType::ColonThree { add: true, len: 2 }, ":33", 150..153),
            Token::new(
                TokenType::Print {
                    kind: PrintKind::Normal
                },
                "mrrp",
                154..158
            ),
            Token::new(TokenType::FlusteredDot, ">.<", 188..191),
            Token::new(TokenType::Eof, "\0", 192..192),
        ]
    );

    Ok(())
}
