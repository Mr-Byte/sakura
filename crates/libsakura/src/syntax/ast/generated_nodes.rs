//! Code generated by `cargo xtask codegen`; DO NOT EDIT.

use super::support;
use crate::syntax::{
    AstChildren, AstNode,
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken,
};
use crate::T;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}

impl Name {
    pub fn identifier_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["identifier"])
    }
}

impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NAME
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub(crate) syntax: SyntaxNode,
}

impl SourceFile {
    pub fn items(&self) -> AstChildren<Item> {
        support::children(&self.syntax)
    }
}

impl AstNode for SourceFile {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SOURCE_FILE
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub(crate) syntax: SyntaxNode,
}

impl Item {
    pub fn type_definition(&self) -> Option<TypeDefinition> {
        support::child(&self.syntax)
    }
}

impl AstNode for Item {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ITEM
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeDefinition {
    pub(crate) syntax: SyntaxNode,
}

impl TypeDefinition {
    pub fn type_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["type"])
    }

    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }

    pub fn equal_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["="])
    }

    pub fn body(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}

impl AstNode for TypeDefinition {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TYPE_DEFINITION
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructType {
    pub(crate) syntax: SyntaxNode,
}

impl StructType {
    pub fn struct_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["struct"])
    }

    pub fn struct_field_definition_list(&self) -> Option<StructFieldDefinitionList> {
        support::child(&self.syntax)
    }
}

impl AstNode for StructType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == STRUCT_TYPE
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumType {
    pub(crate) syntax: SyntaxNode,
}

impl EnumType {
    pub fn enum_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["enum"])
    }

    pub fn left_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["{"])
    }

    pub fn right_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["}"])
    }
}

impl AstNode for EnumType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ENUM_TYPE
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitType {
    pub(crate) syntax: SyntaxNode,
}

impl TraitType {
    pub fn trait_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["trait"])
    }

    pub fn left_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["{"])
    }

    pub fn right_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["}"])
    }
}

impl AstNode for TraitType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TRAIT_TYPE
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructFieldDefinitionList {
    pub(crate) syntax: SyntaxNode,
}

impl StructFieldDefinitionList {
    pub fn left_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["{"])
    }

    pub fn fields(&self) -> AstChildren<StructField> {
        support::children(&self.syntax)
    }

    pub fn right_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!["}"])
    }
}

impl AstNode for StructFieldDefinitionList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == STRUCT_FIELD_DEFINITION_LIST
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructField {
    pub(crate) syntax: SyntaxNode,
}

impl StructField {
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }

    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![":"])
    }

    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}

impl AstNode for StructField {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == STRUCT_FIELD
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Name(Name),
    StructType(StructType),
    EnumType(EnumType),
    TraitType(TraitType),
}

impl From<Name> for Type {
    fn from(node: Name) -> Type {
        Type::Name(node)
    }
}

impl From<StructType> for Type {
    fn from(node: StructType) -> Type {
        Type::StructType(node)
    }
}

impl From<EnumType> for Type {
    fn from(node: EnumType) -> Type {
        Type::EnumType(node)
    }
}

impl From<TraitType> for Type {
    fn from(node: TraitType) -> Type {
        Type::TraitType(node)
    }
}

impl AstNode for Type {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, NAME | STRUCT_TYPE | ENUM_TYPE | TRAIT_TYPE)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            NAME => Type::Name(Name { syntax }),
            STRUCT_TYPE => Type::StructType(StructType { syntax }),
            ENUM_TYPE => Type::EnumType(EnumType { syntax }),
            TRAIT_TYPE => Type::TraitType(TraitType { syntax }),
            _ => return None,
        };

        Some(res)
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Type::Name(it) => it.syntax(),
            Type::StructType(it) => it.syntax(),
            Type::EnumType(it) => it.syntax(),
            Type::TraitType(it) => it.syntax(),
        }
    }
}
