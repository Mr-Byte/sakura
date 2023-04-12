#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sanity_check_module_create() {
        unsafe {
            let module = BinaryenModuleCreate();

            assert!(!module.is_null());

            BinaryenModuleDispose(module);
        }
    }
}
