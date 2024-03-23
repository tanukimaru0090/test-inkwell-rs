extern crate inkwell as iw;
use iw::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicTypeEnum,StructType},
    values::{AnyValue, AsValueRef, BasicValue, BasicValueEnum, FunctionValue, IntValue}, AddressSpace,
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
    // llvmの初期化
    let context = Context::create();
    let module = context.create_module("main.tanu");
    let builder = context.create_builder();
    // i32型の型定義
    let i32_type = context.i32_type();
    // void型の型定義
    let void_type = context.void_type();
    // add関数の型定義
    let add_func_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    // main関数の型定義
    let main_type = i32_type.fn_type(&[], false);
    // 構造体の型定義
    let struct_type = context.struct_type(&[i32_type.into(), i32_type.into()],false);

    // add関数の定義と処理
    let add_func = module.add_function("add", add_func_type, None);
    // add関数のベーシックブロックを作成
    let add_entry = context.append_basic_block(add_func, "entry");
    builder.position_at_end(add_entry);
    // 仮引数２つを取得
    let x = add_func.get_nth_param(0).unwrap().into_int_value();
    let y = add_func.get_nth_param(1).unwrap().into_int_value();
    // 結果を保存し、戻り値として返す
    let sum = builder.build_int_add(x, y, "sumTemp")?;
    builder.build_return(Some(&sum)).unwrap();
    

    // 構造体を定義 
    let g_struct = module.add_global(struct_type, Some(AddressSpace::default()), "myStruct");
    let frist_value = i32_type.const_int(10,false);
    let second_value = i32_type.const_int(10,false);
    let struct_value = struct_type.const_named_struct(&[frist_value.into(),second_value.into()]);
    g_struct.set_initializer(&struct_value);

    // main関数の定義
    let main_func = module.add_function("main", main_type, None);
    // main関数のベーシックブロックを作成
    let main_entry = context.append_basic_block(main_func, "entry");
    builder.position_at_end(main_entry);
    let add_args = [i32_type.const_int(50, false).into(),i32_type.const_int(100, false).into()];
    let add_res = builder.build_call(add_func, add_args.as_slice(), "addTemp").unwrap().try_as_basic_value().left().unwrap();
    builder.build_return(Some(&add_res))?;
    


    module.print_to_stderr();
    
    // ファイルに書き込み 
    write_ir_file("output.ll",&module)?;
    
    Ok(())
}
