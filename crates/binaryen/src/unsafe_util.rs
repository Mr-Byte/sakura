pub(crate) unsafe trait UnsafeMaybe {
    type Out;

    fn as_ptr_or_null(self: Self) -> *mut Self::Out;
}
