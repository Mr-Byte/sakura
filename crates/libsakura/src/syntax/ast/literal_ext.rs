use super::{ByteLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral};
use crate::{
    syntax::{ast::AstToken, SyntaxToken},
    T,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    StringLiteral(StringLiteral),
    IntLiteral(IntLiteral),
    FloatLiteral(FloatLiteral),
    ByteLiteral(ByteLiteral),
    CharLiteral(CharLiteral),
    BoolLiteral(bool),
}

impl super::Literal {
    pub fn token(&self) -> SyntaxToken {
        self.syntax
            .children_with_tokens()
            .find(|e| !e.kind().is_trivia())
            .and_then(|e| e.into_token())
            .unwrap()
    }

    pub fn kind(&self) -> LiteralKind {
        let token = self.token();

        if let Some(int_literal) = IntLiteral::cast(token.clone()) {
            return LiteralKind::IntLiteral(int_literal);
        }

        if let Some(float_literal) = FloatLiteral::cast(token.clone()) {
            return LiteralKind::FloatLiteral(float_literal);
        }

        if let Some(byte_literal) = ByteLiteral::cast(token.clone()) {
            return LiteralKind::ByteLiteral(byte_literal);
        }

        if let Some(char_literal) = CharLiteral::cast(token.clone()) {
            return LiteralKind::CharLiteral(char_literal);
        }

        if let Some(string_literal) = StringLiteral::cast(token.clone()) {
            return LiteralKind::StringLiteral(string_literal);
        }

        match token.kind() {
            T!["true"] => LiteralKind::BoolLiteral(true),
            T!["false"] => LiteralKind::BoolLiteral(false),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::syntax::builder::SyntaxTreeBuilder;
    use crate::{
        syntax::{ast::Literal, SyntaxKind},
        T,
    };

    #[test]
    fn test_string_literal() {
        let source = r#""Hello, world!""#;
        let mut builder = SyntaxTreeBuilder::default();

        builder.start_node(SyntaxKind::LITERAL);
        builder.token(SyntaxKind::STRING_LITERAL, source);
        builder.finish_node();

        let parse = builder.finish();

        let literal = parse.cast::<Literal>().unwrap().tree();
        let value = literal.syntax.text().to_string();

        assert!(matches!(literal.kind(), LiteralKind::StringLiteral(_)),);
        assert_eq!(source, value);
    }

    #[test]
    fn test_true_bool_literal() {
        let source = r#"true"#;
        let mut builder = SyntaxTreeBuilder::default();

        builder.start_node(SyntaxKind::LITERAL);
        builder.token(T!["true"], source);
        builder.finish_node();

        let parse = builder.finish();

        let literal = parse.cast::<Literal>().unwrap().tree();
        let value = literal.syntax.text().to_string();

        assert!(matches!(literal.kind(), LiteralKind::BoolLiteral(true)),);
        assert_eq!(source, value);
    }

    #[test]
    fn test_false_bool_literal() {
        let source = r#"false"#;
        let mut builder = SyntaxTreeBuilder::default();

        builder.start_node(SyntaxKind::LITERAL);
        builder.token(T!["false"], source);
        builder.finish_node();

        let parse = builder.finish();

        let literal = parse.cast::<Literal>().unwrap().tree();
        let value = literal.syntax.text().to_string();

        assert!(matches!(literal.kind(), LiteralKind::BoolLiteral(false)),);
        assert_eq!(source, value);
    }
}
