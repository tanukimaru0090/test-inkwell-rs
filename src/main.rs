extern crate inkwell as iw;
use iw::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicType, BasicTypeEnum, StructType},
    values::{AnyValue, AsValueRef, BasicValue, BasicValueEnum, FunctionValue, IntValue},
    AddressSpace,
};
//use core::slice::SlicePattern;
use std::fs::File;
use std::io::Write;

fn write_ir_file(file_name: &str, module: &Module) -> Result<(), Box<dyn std::error::Error>> {
    let ir_string = module.print_to_string();
    let mut file = File::create(file_name)?;
    file.write_all(ir_string.to_bytes())?;
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // メインモジュールの名前
    let module_name = "main";
    // llvmの初期化
    let context = Context::create();
    let module = context.create_module(module_name);
    let builder = context.create_builder();
    // i32型の型定義
    let i32_type = context.i32_type();
    // i8型の型定義
    let i8_type = context.i8_type();
    // void型の型定義
    let void_type = context.void_type();
    // main関数の型定義
    let main_type = i32_type.fn_type(&[], false);
    // 構造体の型定義
    let struct_type = context.struct_type(&[i32_type.into(), i32_type.into()], false);
    // add関数の型定義
    let add_func_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);

    // sub関数の型定義
    let sub_func_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    // sub関数の型定義
    let mul_func_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);

    // sub関数の型定義
    let div_func_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);

    // printf関数の型定義
    let printf_type = i32_type.fn_type(&[i8_type.ptr_type(AddressSpace::default()).into()], true);
    // printf関数の定義
    let printf_func = module.add_function("printf", printf_type, None);
    let format_str = "%d\n\0";
    let global_format_str = module.add_global(
        i8_type.array_type(format_str.len() as u32),
        Some(AddressSpace::default()),
        "formatStr",
    );
    global_format_str.set_initializer(
        &i8_type.const_array(
            &format_str
                .as_bytes()
                .iter()
                .map(|&b| i8_type.const_int(b as u64, false))
                .collect::<Vec<IntValue>>(),
        ),
    );
    global_format_str.set_constant(true);
    global_format_str.set_linkage(iw::module::Linkage::Private);

    // add関数の定義と処理
    let add_func = module.add_function("add", add_func_type, None);
    // add関数のベーシックブロックを作成
    let add_entry = context.append_basic_block(add_func, "entry");
    builder.position_at_end(add_entry);
    // 仮引数２つを取得
    let x = add_func.get_nth_param(0).unwrap().into_int_value();
    let y = add_func.get_nth_param(1).unwrap().into_int_value();
    // 結果を保存し、戻り値として返す
    let add_sum = builder.build_int_add(x, y, "sumTemp")?;
    builder.build_return(Some(&add_sum)).unwrap();

    // sub関数の定義と処理
    let sub_func = module.add_function("sub", sub_func_type, None);
    // sub関数のベーシックブロックを作成
    let sub_entry = context.append_basic_block(sub_func, "entry");
    builder.position_at_end(sub_entry);
    // 仮引数２つを取得
    let x = sub_func.get_nth_param(0).unwrap().into_int_value();
    let y = sub_func.get_nth_param(1).unwrap().into_int_value();
    // 結果を保存し、戻り値として返す
    let sub_sum = builder.build_int_sub(x, y, "sumTemp")?;
    builder.build_return(Some(&sub_sum)).unwrap();

    // mul関数の定義と処理
    let mul_func = module.add_function("mul", mul_func_type, None);
    // mul関数のベーシックブロックを作成
    let mul_entry = context.append_basic_block(mul_func, "entry");
    builder.position_at_end(mul_entry);
    // 仮引数２つを取得
    let x = mul_func.get_nth_param(0).unwrap().into_int_value();
    let y = mul_func.get_nth_param(1).unwrap().into_int_value();
    // 結果を保存し、戻り値として返す
    let mul_sum = builder.build_int_mul(x, y, "sumTemp")?;
    builder.build_return(Some(&mul_sum)).unwrap();

    // div関数の定義と処理
    let div_func = module.add_function("div", div_func_type, None);
    // div関数のベーシックブロックを作成
    let div_entry = context.append_basic_block(div_func, "entry");
    builder.position_at_end(div_entry);
    // 仮引数２つを取得
    let x = div_func.get_nth_param(0).unwrap().into_int_value();
    let y = div_func.get_nth_param(1).unwrap().into_int_value();
    // 結果を保存し、戻り値として返す
    let div_sum = builder.build_int_signed_div(x, y, "sumTemp")?;
    builder.build_return(Some(&div_sum)).unwrap();

    // 構造体を定義
    let g_struct = module.add_global(struct_type, Some(AddressSpace::default()), "myStruct");
    let frist_value = i32_type.const_int(10, false);
    let second_value = i32_type.const_int(10, false);
    let struct_value = struct_type.const_named_struct(&[frist_value.into(), second_value.into()]);
    g_struct.set_initializer(&struct_value);

    // main関数の定義
    let main_func = module.add_function("main", main_type, None);
    // main関数のベーシックブロックを作成
    let main_entry = context.append_basic_block(main_func, "entry");
    builder.position_at_end(main_entry);
    //let add_args = [i32_type.const_int(10, false).into(),i32_type.const_int(5, false).into()];
    //let add_res = builder.build_call(sub_func, add_args.as_slice(), "addTemp").unwrap().try_as_basic_value().left().unwrap();
    let add_res = i32_type.const_int(1919, false);
    unsafe {
        let format_str_ptr = builder
            .build_in_bounds_gep(
                global_format_str.as_pointer_value(),
                &[i32_type.const_int(0, false), i32_type.const_int(0, false)],
                "formatStrPtr",
            )
            .unwrap();
        let printf_args = &[format_str_ptr.into(), add_res.into()];
        let printf_res = builder
            .build_call(printf_func, printf_args.as_slice(), "printfTemp")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap();
        builder.build_return(Some(&printf_res))?;
    }

    module.print_to_stderr();

    // ファイルに書き込み
    write_ir_file("output.ll", &module)?;

    Ok(())
}
