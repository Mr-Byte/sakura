use crate::parser::event::Event;
use crate::parser::input::ParserInput;
use crate::syntax::{SyntaxKind, TokenSet};
use crate::T;

pub(crate) struct Parser<'a> {
    input: &'a ParserInput,
    position: usize,
    events: Vec<Event>,
}

/// Public methods.
impl<'a> Parser<'a> {
    pub fn new(input: &'a ParserInput) -> Parser<'a> {
        Parser { input, position: 0, events: Vec::new() }
    }

    pub fn finish(self) -> Vec<Event> {
        self.events
    }
}

/// Methods for consuming token kinds.
impl<'a> Parser<'a> {
    pub(crate) fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }

        let raw_token_count = match kind {
            T!["-="]
            | T!["!="]
            | T![".."]
            | T!["*="]
            | T!["/="]
            | T!["&&"]
            | T!["&="]
            | T!["%="]
            | T!["^="]
            | T!["+="]
            | T!["<<"]
            | T!["<="]
            | T!["=="]
            | T!["=>"]
            | T![">="]
            | T![">>"]
            | T!["|="]
            | T!["||"] => 2,

            T!["..="] | T!["<<="] | T![">>="] => 3,
            _ => 1,
        };

        self.do_bump(kind, raw_token_count);
        true
    }

    pub(crate) fn bump_any(&mut self) {
        let kind = self.nth(0);

        if kind == SyntaxKind::EOF {
            return;
        }

        self.do_bump(kind, 1);
    }

    fn do_bump(&mut self, kind: SyntaxKind, raw_token_count: u8) {
        self.position += raw_token_count as usize;
        self.events.push(Event::Token { kind, raw_token_count });
    }
}

/// Methods for retrieving syntax kinds.
impl<'a> Parser<'a> {
    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        assert!(n < 3);

        self.input.kind(self.position + n)
    }
}

/// Methods for checking syntax kinds.
impl<'a> Parser<'a> {
    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    pub(crate) fn at_token_set(&self, kinds: TokenSet) -> bool {
        kinds.contains(self.current())
    }

    pub(crate) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        match kind {
            T![".."] => self.at_composite2(n, T!["."], T!["."]),
            T!["+="] => self.at_composite2(n, T!["+"], T!["="]),
            T!["-="] => self.at_composite2(n, T!["-"], T!["="]),
            T!["*="] => self.at_composite2(n, T!["*"], T!["="]),
            T!["/="] => self.at_composite2(n, T!["/"], T!["="]),
            T!["%="] => self.at_composite2(n, T!["%"], T!["="]),
            T!["^="] => self.at_composite2(n, T!["^"], T!["="]),
            T!["&="] => self.at_composite2(n, T!["&"], T!["="]),
            T!["|="] => self.at_composite2(n, T!["|"], T!["="]),

            T!["=="] => self.at_composite2(n, T!["="], T!["="]),
            T!["!="] => self.at_composite2(n, T!["!"], T!["="]),

            T!["&&"] => self.at_composite2(n, T!["&"], T!["&"]),
            T!["||"] => self.at_composite2(n, T!["|"], T!["|"]),

            T!["<="] => self.at_composite2(n, T!["<"], T!["="]),
            T![">="] => self.at_composite2(n, T![">"], T!["="]),

            T!["<<"] => self.at_composite2(n, T!["<"], T!["<"]),
            T![">>"] => self.at_composite2(n, T![">"], T![">"]),

            T!["=>"] => self.at_composite2(n, T!["="], T![">"]),

            T!["..="] => self.at_composite3(n, T!["."], T!["."], T!["="]),
            T!["<<="] => self.at_composite3(n, T!["<"], T!["<"], T!["="]),
            T![">>="] => self.at_composite3(n, T![">"], T![">"], T!["="]),

            _ => self.input.kind(self.position + n) == kind,
        }
    }

    pub(crate) fn at_composite2(&self, n: usize, first: SyntaxKind, second: SyntaxKind) -> bool {
        self.input.kind(self.position + n) == first
            && self.input.kind(self.position + n + 1) == second
            && self.input.is_joint(self.position + n)
    }

    pub(crate) fn at_composite3(
        &self,
        n: usize,
        first: SyntaxKind,
        second: SyntaxKind,
        third: SyntaxKind,
    ) -> bool {
        self.input.kind(self.position + n) == first
            && self.input.kind(self.position + n + 1) == second
            && self.input.kind(self.position + n + 2) == third
            && self.input.is_joint(self.position + n)
            && self.input.is_joint(self.position + n + 1)
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::LexedStr;
    use crate::parser::input::ParserInput;
    use crate::T;

    #[test]
    fn nth_at_returns_true_for_joint_tokens() {
        let tokens = LexedStr::new(
            "\
            -= \
            != \
            .. \
            *= \
            /= \
            && \
            &= \
            %= \
            ^= \
            += \
            <= \
            == \
            => \
            >= \
            >> \
            |= \
            || \
            ..= \
            <<= \
            >>= \
            - \
            = \
            ",
        );

        let input: ParserInput = tokens.into();
        let parser = super::Parser::new(&input);

        assert!(parser.nth_at(0, T!["-="]));
        assert!(parser.nth_at(2, T!["!="]));
        assert!(parser.nth_at(4, T![".."]));
        assert!(parser.nth_at(6, T!["*="]));
        assert!(parser.nth_at(8, T!["/="]));
        assert!(parser.nth_at(10, T!["&&"]));
        assert!(parser.nth_at(12, T!["&="]));
        assert!(parser.nth_at(14, T!["%="]));
        assert!(parser.nth_at(16, T!["^="]));
        assert!(parser.nth_at(18, T!["+="]));
        assert!(parser.nth_at(20, T!["<="]));
        assert!(parser.nth_at(22, T!["=="]));
        assert!(parser.nth_at(24, T!["=>"]));
        assert!(parser.nth_at(26, T![">="]));
        assert!(parser.nth_at(28, T![">>"]));
        assert!(parser.nth_at(30, T!["|="]));
        assert!(parser.nth_at(32, T!["||"]));
        assert!(parser.nth_at(34, T!["..="]));
        assert!(parser.nth_at(37, T!["<<="]));
        assert!(parser.nth_at(40, T![">>="]));
        assert!(parser.nth_at(43, T!["-"]));
        assert!(parser.nth_at(44, T!["="]));
    }
}
