mod expr;
mod function;
mod module;
mod operator;
mod types;

pub use expr::*;
pub use function::*;
pub use module::*;
pub use operator::*;
pub use types::*;

#[cfg(test)]
pub(crate) mod test {
    use crate::Type;

    use super::*;

    #[test]
    fn test_print_module() {
        let module = Module::new();
        let params = Type::new(&[Type::i32(), Type::i32()]);
        let results = Type::i32();

        let x = module.local_get(0, Type::i32());
        let y = module.local_get(1, Type::i32());
        let add = module.binary_expr(Operator::add_i32(), x, y);
        let block = module.block(Some("test"), &[add], Type::i32());

        let bigger_block = module.block(None, &[block], Type::i32());

        module.add_function("adder", params, results, bigger_block);
        module.add_function_export("adder", "add");
        module.print();
    }
}
