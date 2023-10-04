use crate::parser::event::Event;
use crate::parser::input::ParserInput;
use crate::parser::marker::Marker;
use crate::syntax::{SyntaxKind, TokenSet};
use crate::T;

pub(in crate::parser) struct Parser<'a> {
    input: &'a ParserInput,
    position: usize,
    pub(super) events: Vec<Event>,
    // TODO: Add parser hang detection.
}

/// Public methods.
impl<'a> Parser<'a> {
    pub(in crate::parser) fn new(input: &'a ParserInput) -> Parser<'a> {
        Parser { input, position: 0, events: Vec::new() }
    }

    pub(in crate::parser) fn start_node(&mut self) -> Marker {
        let position = self.events.len();
        self.push_event(Event::tombstone());

        Marker::new(position)
    }

    pub(in crate::parser) fn finish(self) -> Vec<Event> {
        self.events
    }
}

/// Methods for consuming token kinds.
impl<'a> Parser<'a> {
    pub(in crate::parser) fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    pub(in crate::parser) fn eat(&mut self, kind: SyntaxKind) -> bool {
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

    pub(in crate::parser) fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }

        self.error(format!("expected {:?}", kind));
        false
    }

    pub(in crate::parser) fn bump_any(&mut self) {
        let kind = self.nth(0);

        if kind == SyntaxKind::EOF {
            return;
        }

        self.do_bump(kind, 1);
    }

    fn do_bump(&mut self, kind: SyntaxKind, raw_token_count: u8) {
        self.position += raw_token_count as usize;
        self.push_event(Event::Token { kind, raw_token_count });
    }

    pub(in crate::parser) fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }
}

/// Methods for retrieving syntax kinds.
impl<'a> Parser<'a> {
    pub(in crate::parser) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    pub(in crate::parser) fn nth(&self, n: usize) -> SyntaxKind {
        assert!(n < 3);

        self.input.kind(self.position + n)
    }
}

/// Methods for checking syntax kinds.
impl<'a> Parser<'a> {
    pub(in crate::parser) fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    pub(in crate::parser) fn at_token_set(&self, kinds: TokenSet) -> bool {
        kinds.contains(self.current())
    }

    pub(in crate::parser) fn nth_at_token_set(&self, n: usize, kinds: TokenSet) -> bool {
        kinds.contains(self.nth(n))
    }

    pub(in crate::parser) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
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

    pub(in crate::parser) fn at_composite2(
        &self,
        n: usize,
        first: SyntaxKind,
        second: SyntaxKind,
    ) -> bool {
        self.input.kind(self.position + n) == first
            && self.input.kind(self.position + n + 1) == second
            && self.input.is_joint(self.position + n)
    }

    pub(in crate::parser) fn at_composite3(
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

/// Methods for error handling.
impl Parser<'_> {
    pub(in crate::parser) fn error(&mut self, message: impl Into<String>) {
        self.push_event(Event::Error { message: message.into() });
    }

    pub(in crate::parser) fn error_recover(&mut self, message: &str, recovery_set: TokenSet) {
        if let T!["{"] | T!["}"] = self.current() {
            self.error(message);
            return;
        }

        if self.at_token_set(recovery_set) {
            self.error(message);
            return;
        }

        let marker = self.start_node();
        self.error(message);
        self.bump_any();
        marker.complete(self, SyntaxKind::ERROR);
    }

    pub(in crate::parser) fn error_and_bump(&mut self, message: &str) {
        self.error_recover(message, TokenSet::EMPTY);
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::LexedStr;
    use crate::parser::input::ParserInput;
    use crate::syntax::SyntaxKind;
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

        let input: ParserInput = tokens.as_input();
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

    #[test]
    fn eat_processes_joint_token() {
        let tokens = LexedStr::new("-=");
        let input: ParserInput = tokens.as_input();
        let mut parser = super::Parser::new(&input);

        assert!(parser.eat(T!["-="]));
        assert_eq!(parser.current(), SyntaxKind::EOF);
    }
}
