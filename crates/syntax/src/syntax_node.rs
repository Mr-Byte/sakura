use rowan::{GreenNodeBuilder, Language};

use crate::{Parse, SyntaxError, SyntaxKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SakuraLang {}

impl Language for SakuraLang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<SakuraLang>;
pub type SyntaxToken = rowan::SyntaxToken<SakuraLang>;
pub type SyntaxElement = rowan::SyntaxElement<SakuraLang>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<SakuraLang>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<SakuraLang>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<SakuraLang>;

#[derive(Default)]
pub struct SyntaxTreeBuilder {
    errors: Vec<SyntaxError>,
    builder: GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    pub fn finish(self) -> Parse<SyntaxNode> {
        let green_node = self.builder.finish();
        let errors = self.errors;

        Parse::new(green_node, errors)
    }

    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let kind = SakuraLang::kind_to_raw(kind);
        self.builder.token(kind, text);
    }

    pub fn start_node(&mut self, kind: SyntaxKind) {
        let kind = SakuraLang::kind_to_raw(kind);
        self.builder.start_node(kind);
    }

    pub fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    pub fn error(&mut self, error: String, text_pos: crate::TextSize) {
        self.errors.push(SyntaxError::new_at_offset(error, text_pos));
    }
}
