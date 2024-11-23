#[cfg(test)]
mod tests {
    use crate::tokenize;
    use codecrafters_interpreter::{Token, TokenType};

    fn destructure(token: Token) -> (TokenType, String, String) {
        let Token {
            token_type,
            lexeme,
            literal,
            line_num: _,
        } = token;

        (token_type, lexeme, literal)
    }

    #[test]
    fn comment_tokenization() {
        let (res, _) = tokenize("test// ignore this comment".to_string());
        assert_eq!(
            destructure(res[0].clone()),
            (
                TokenType::IDENTIFIER,
                String::from("test"),
                String::from("null")
            )
        );
        assert_eq!(
            destructure(res[1].clone()),
            (TokenType::EOF, String::from(""), String::from("null"))
        );
    }

    #[test]
    fn string_tokenization() {
        let (res, _) = tokenize("\"a string\" not_a_string .*+".to_string());
        assert_eq!(
            destructure(res[0].clone()),
            (
                TokenType::STRING,
                String::from("\"a string\""),
                String::from("a string")
            )
        );
        assert_eq!(
            destructure(res[1].clone()),
            (
                TokenType::IDENTIFIER,
                String::from("not_a_string"),
                String::from("null")
            )
        );
        assert_eq!(
            destructure(res[2].clone()),
            (TokenType::DOT, String::from("."), String::from("null"))
        );
        assert_eq!(
            destructure(res[3].clone()),
            (TokenType::STAR, String::from("*"), String::from("null"))
        );
        assert_eq!(
            destructure(res[4].clone()),
            (TokenType::PLUS, String::from("+"), String::from("null"))
        );
        assert_eq!(
            destructure(res[5].clone()),
            (TokenType::EOF, String::from(""), String::from("null"))
        );
    }

    #[test]
    fn number_tokenization() {
        let (res, _) = tokenize("23.000 57 3.1.4".to_string());
        assert_eq!(
            destructure(res[0].clone()),
            (
                TokenType::NUMBER,
                String::from("23.000"),
                String::from("23.0")
            )
        );
        assert_eq!(
            destructure(res[1].clone()),
            (TokenType::NUMBER, String::from("57"), String::from("57.0"))
        );
        assert_eq!(
            destructure(res[2].clone()),
            (TokenType::NUMBER, String::from("3.1"), String::from("3.1"))
        );
        assert_eq!(
            destructure(res[3].clone()),
            (TokenType::DOT, String::from("."), String::from("null"))
        );
        assert_eq!(
            destructure(res[4].clone()),
            (TokenType::NUMBER, String::from("4"), String::from("4.0"))
        );
        assert_eq!(
            destructure(res[5].clone()),
            (TokenType::EOF, String::from(""), String::from("null"))
        );
    }
}
