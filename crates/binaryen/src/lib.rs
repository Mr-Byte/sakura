mod expression;
mod function;
mod literals;
mod module;
mod operator;
mod types;
mod unsafe_util;

pub use expression::*;
pub use function::*;
pub use literals::*;
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

        let arg_x = module.expr_local_get(0, Type::i32());
        let arg_y = module.expr_local_get(1, Type::i32());
        let add = module.expr_binary(Operator::add_i32(), arg_x, arg_y);
        module.add_function("adder", params, results, add.clone());
        module.add_function_export("adder", "add");

        let x = Literal::i32(10);
        let y = Literal::i32(20);
        let x_const = module.expr_const(x);
        let y_const = module.expr_const(y);
        let call = module.expr_call("adder", &mut [x_const, y_const], Type::i32());
        let main = module.add_function("main", Type::none(), Type::none(), call);

        module.set_start(main);

        module.print();
    }
}
