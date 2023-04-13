pub(crate) unsafe trait UnsafeMaybe {
    type Out;

    fn as_ptr(self: Self) -> *mut Self::Out;
}
