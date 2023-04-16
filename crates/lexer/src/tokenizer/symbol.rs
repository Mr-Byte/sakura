#[inline]
pub(super) fn is_identifier_start(c: char) -> bool {
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

#[inline]
pub(super) fn is_identifier_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

#[inline]
pub(super) fn is_reserved_string_symbol(c: char) -> bool {
    c == '\\' || c == '"' || c == '$'
}
