#![cfg(test)]

use indoc::indoc;
use pretty_assertions::assert_eq;

use super::*;

fn lex(source: &str) -> crate::Result<TokenStream> {
    let mut ctx = SourceContext::new(source, "<test>")?;

    let mut lexer = Lexer::new(&mut ctx);
    lexer.lex_tokens();
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
            Token {
                kind: TokenType::Keysmash {
                    lowercase: true,
                    len: 5
                },
                lexeme: "haiii".to_string(),
                range: (0..5).into()
            },
            Token {
                kind: TokenType::Sub,
                lexeme: "🥺".to_string(),
                range: (6..10).into()
            },
            Token {
                kind: TokenType::Keysmash {
                    lowercase: true,
                    len: 14
                },
                lexeme: "afogjqwefokadj".to_string(),
                range: (15..29).into()
            },
            Token {
                kind: TokenType::Keysmash {
                    lowercase: true,
                    len: 8
                },
                lexeme: "flkadjsf".to_string(),
                range: (30..38).into()
            },
            Token {
                kind: TokenType::Keysmash {
                    lowercase: true,
                    len: 4
                },
                lexeme: "lkad".to_string(),
                range: (39..43).into()
            },
            Token {
                kind: TokenType::ColonThree { len: 3 },
                lexeme: ":333".to_string(),
                range: (44..48).into()
            },
            Token {
                kind: TokenType::BlushSlash { len: 2 },
                lexeme: ">////<".to_string(),
                range: (49..55).into()
            },
            Token {
                kind: TokenType::BlushW,
                lexeme: ">w<".to_string(),
                range: (56..59).into()
            },
            Token {
                kind: TokenType::Comment,
                lexeme: "🏳️‍⚧️ [23, 23, 23]".to_string(),
                range: (60..89).into()
            },
            Token {
                kind: TokenType::ColonThree { len: 3 },
                lexeme: ":333".to_string(),
                range: (94..98).into()
            },
            Token {
                kind: TokenType::Print { utf: false },
                lexeme: "mreow".to_string(),
                range: (99..104).into()
            },
            Token {
                kind: TokenType::Comment,
                lexeme: r#"🏳️‍⚧️ [] "E""#.to_string(),
                range: (105..128).into()
            },
            Token {
                kind: TokenType::Keysmash {
                    lowercase: true,
                    len: 11
                },
                lexeme: "adlkadlfdla".to_string(),
                range: (133..144).into()
            },
            Token {
                kind: TokenType::BlushSlash { len: 1 },
                lexeme: ">//<".to_string(),
                range: (145..149).into()
            },
            Token {
                kind: TokenType::ColonThree { len: 2 },
                lexeme: ":33".to_string(),
                range: (150..153).into()
            },
            Token {
                kind: TokenType::Print { utf: false },
                lexeme: "mrrp".to_string(),
                range: (154..158).into()
            },
            Token {
                kind: TokenType::Comment,
                lexeme: r#"🏳️‍⚧️ [] "\n""#.to_string(),
                range: (159..183).into()
            },
            Token {
                kind: TokenType::BlushDot,
                lexeme: ">.<".to_string(),
                range: (188..191).into()
            },
            Token {
                kind: TokenType::Eof,
                lexeme: '\0'.to_string(),
                range: (192..192).into()
            },
        ]
    );

    Ok(())
}
