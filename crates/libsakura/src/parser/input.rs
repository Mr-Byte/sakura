use crate::lexer::LexedStr;
use crate::syntax::SyntaxKind;
use bitvec::vec::BitVec;

#[derive(Default)]
pub struct ParserInputBuilder {
    kinds: Vec<SyntaxKind>,
    joint: BitVec,
}

impl ParserInputBuilder {
    pub fn push(&mut self, kind: SyntaxKind) {
        self.kinds.push(kind);
        self.joint.push(false);
    }

    pub(crate) fn set_joint(&mut self) {
        assert_eq!(self.kinds.len(), self.joint.len(), "kinds.len() != joint.len()");

        let index = self.len() - 1;

        *self.joint.get_mut(index).unwrap() = true;
    }

    fn len(&self) -> usize {
        self.kinds.len()
    }

    fn build(self) -> ParserInput {
        ParserInput { kinds: self.kinds, joint: self.joint }
    }
}

#[derive(Default)]
pub struct ParserInput {
    kinds: Vec<SyntaxKind>,
    joint: BitVec,
}

impl ParserInput {
    pub(in crate::parser) fn kind(&self, index: usize) -> SyntaxKind {
        self.kinds.get(index).copied().unwrap_or(SyntaxKind::EOF)
    }

    pub(crate) fn is_joint(&self, index: usize) -> bool {
        self.joint[index]
    }
}

impl From<LexedStr<'_>> for ParserInput {
    fn from(lexed: LexedStr<'_>) -> Self {
        let mut builder = ParserInputBuilder::default();

        let mut is_joint = false;
        for index in 0..lexed.len() {
            let kind = lexed.kind(index);

            if kind.is_trivia() {
                is_joint = false;
                continue;
            }

            if is_joint {
                builder.set_joint();
            }

            builder.push(kind);
            is_joint = true;
        }

        builder.build()
    }
}
