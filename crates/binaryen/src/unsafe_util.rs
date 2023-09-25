pub(crate) unsafe trait UnsafeMaybe {
    type Out;

    fn into_nullable_ptr(self) -> *mut Self::Out;
}
