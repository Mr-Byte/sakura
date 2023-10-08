use super::SyntaxKind;

/// TokenSet is used to store a set of ['SyntaxKind'] that can be efficiently checked for membership.
/// This will panic if a SyntaxKind is passed that is not a token.
#[derive(Copy, Clone)]
pub(crate) struct TokenSet(u128);

impl TokenSet {
    const __TOKEN_SENTINEL_ASSERTION: () = assert!(SyntaxKind::__TOKEN_SENTINEL as u16 <= 128);

    pub(crate) const EMPTY: Self = { Self(0) };

    pub(crate) const fn new(kinds: &[SyntaxKind]) -> Self {
        let mut set = Self::EMPTY;

        let mut index = 0;
        while index < kinds.len() {
            set.0 |= Self::mask(kinds[index]);
            index += 1;
        }

        set
    }

    pub(crate) const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub(crate) const fn contains(&self, kind: SyntaxKind) -> bool {
        self.0 & Self::mask(kind) != 0
    }

    const fn mask(kind: SyntaxKind) -> u128 {
        assert!((kind as u16) < (SyntaxKind::__TOKEN_SENTINEL as u16));
        1u128 << (kind as usize)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kind_set() {
        let set = TokenSet::new(&[SyntaxKind::IDENTIFIER, SyntaxKind::INT_LITERAL]);

        assert!(set.contains(SyntaxKind::IDENTIFIER));
        assert!(set.contains(SyntaxKind::INT_LITERAL));
        assert!(!set.contains(SyntaxKind::STRING_LITERAL));
    }

    #[test]
    fn test_kind_set_union() {
        let set1 = TokenSet::new(&[SyntaxKind::IDENTIFIER, SyntaxKind::INT_LITERAL]);
        let set2 = TokenSet::new(&[SyntaxKind::IDENTIFIER, SyntaxKind::STRING_LITERAL]);

        let set = set1.union(set2);

        assert!(set.contains(SyntaxKind::IDENTIFIER));
        assert!(set.contains(SyntaxKind::INT_LITERAL));
        assert!(set.contains(SyntaxKind::STRING_LITERAL));
    }

    #[test]
    const fn test_kind_set_union_const() {
        const SET1: TokenSet = TokenSet::new(&[SyntaxKind::IDENTIFIER, SyntaxKind::INT_LITERAL]);
        const SET2: TokenSet = TokenSet::new(&[SyntaxKind::IDENTIFIER, SyntaxKind::STRING_LITERAL]);

        const SET: TokenSet = SET1.union(SET2);

        assert!(SET.contains(SyntaxKind::IDENTIFIER));
        assert!(SET.contains(SyntaxKind::INT_LITERAL));
        assert!(SET.contains(SyntaxKind::STRING_LITERAL));
    }

    #[test]
    fn test_kind_set_empty() {
        let set = TokenSet::EMPTY;

        assert!(!set.contains(SyntaxKind::IDENTIFIER));
        assert!(!set.contains(SyntaxKind::INT_LITERAL));
        assert!(!set.contains(SyntaxKind::STRING_LITERAL));
    }

    #[test]
    fn test_kind_set_empty_union() {
        let set1 = TokenSet::EMPTY;
        let set2 = TokenSet::EMPTY;

        let set = set1.union(set2);

        assert!(!set.contains(SyntaxKind::IDENTIFIER));
        assert!(!set.contains(SyntaxKind::INT_LITERAL));
        assert!(!set.contains(SyntaxKind::STRING_LITERAL));
    }

    #[test]
    fn test_kind_set_empty_union_nonempty() {
        let set1 = TokenSet::EMPTY;
        let set2 = TokenSet::new(&[SyntaxKind::IDENTIFIER, SyntaxKind::INT_LITERAL]);

        let set = set1.union(set2);

        assert!(set.contains(SyntaxKind::IDENTIFIER));
        assert!(set.contains(SyntaxKind::INT_LITERAL));
        assert!(!set.contains(SyntaxKind::STRING_LITERAL));
    }

    #[test]
    #[should_panic]
    fn panics_on_non_token() {
        TokenSet::new(&[SyntaxKind::INTERPOLATED_STRING_SLOT]);
    }
}
