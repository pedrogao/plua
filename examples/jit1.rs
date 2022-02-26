use std::mem;

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{default_libcall_names, Linkage, Module};

fn main() {
    // jit module
    let mut module = JITModule::new(JITBuilder::new(default_libcall_names()));
    // 上下文
    let mut ctx = module.make_context();
    // 函数上下文
    let mut func_ctx = FunctionBuilderContext::new();

    // 函数签名
    let mut sig = module.make_signature();
    sig.params.push(AbiParam::new(types::I32)); // 添加参数 i32
    sig.params.push(AbiParam::new(types::I32)); // 添加参数 i32
    sig.returns.push(AbiParam::new(types::I32)); // 添加返回值 i32

    // funca 函数声明
    let funca = module
        .declare_function("funca", Linkage::Local, &sig)
        .unwrap();

    ctx.func.signature = sig; // 指定上下文函数中的签名
    ctx.func.name = ExternalName::user(0, funca.as_u32()); // 指定上下文函数中的名称
    {
        // 函数构造器
        let mut bcx: FunctionBuilder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
        let block = bcx.create_block();
        bcx.switch_to_block(block);
        
        bcx.append_block_params_for_function_params(block); // 函数参数在block中可用

        let param0 = bcx.block_params(block)[0];
        let param1 = bcx.block_params(block)[1];

        let cst = bcx.ins().iconst(types::I32, 25);

        let add = bcx.ins().iadd(cst, param0);
        let mul = bcx.ins().imul(add, param1);

        bcx.ins().return_(&[mul]); // 返回 mul

        bcx.seal_all_blocks();
        bcx.finalize(); // 完成
    }

    module.define_function(funca, &mut ctx).unwrap();
    module.clear_context(&mut ctx);

    // Perform linking.
    module.finalize_definitions();

    // 得到funca对应的机器码
    let codea = module.get_finalized_function(funca);

    // 将机器码强转为函数
    let ptra = unsafe { mem::transmute::<_, fn(u32, u32) -> u32>(codea) };

    // 调用
    let res = ptra(1, 3); // (1 + 25) * 3

    println!("{}", res);
}
