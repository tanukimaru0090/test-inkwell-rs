extern crate inkwell as iw;
use iw::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{FunctionValue, IntValue},
};
use std::fs::{
    File,
};
use std::io::Write;

fn write_ir_file(file_name:&str,module:&Module)->Result<(),Box<dyn std::error::Error>>{
    let ir_string = module.print_to_string();
    let mut file = File::create(file_name)?;
    file.write_all(ir_string.to_bytes())?;
    Ok(())
}
fn main()->Result<(),Box<dyn std::error::Error>> {
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();
    let i32_type = context.i32_type();
    let void_type = context.void_type();
    let add_func_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let add_func = module.add_function("add", add_func_type, None);
    let add_entry = context.append_basic_block(add_func, "entry");
    builder.position_at_end(add_entry);
    let x = add_func.get_nth_param(0).unwrap().into_int_value();
    let y = add_func.get_nth_param(1).unwrap().into_int_value();
    let sum = builder.build_int_add(x, y, "sumTemp")?;
    builder.build_return(Some(&sum)).unwrap();

    // main関数の型定義
    let main_type = void_type.fn_type(&[], false);
    // main関数の定義
    let main_func = module.add_function("main", main_type, None);
    // main関数のベーシックブロックを作成
    let main_entry = context.append_basic_block(main_func, "entry");
    builder.position_at_end(main_entry);
    builder.build_return(None)?;
    module.print_to_stderr();
    
    // ファイルに書き込み 
    write_ir_file("output.ll",&module)?;
    
    Ok(())
}
