use dynasm::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi};

use std::io::{stdout, Write};

// unsafe extern "sysv64" fn print(buf: *const u8, len: u64) -> u8 {
//     let buf = std::slice::from_raw_parts(buf, len as usize);
//     stdout().write_all(buf).is_err() as u8
// }

// 避免 panic
unsafe extern "sysv64" fn print(buf: *const u8, len: u64) -> u8 {
    let ret = std::panic::catch_unwind(|| {
        let buf = std::slice::from_raw_parts(buf, len as usize);
        stdout().write_all(buf).is_err()
    });
    match ret {
        Ok(false) => 0,
        Ok(true) => 1,
        Err(_) => 2,
    }
}

fn main() {
    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    let s = b"Hello, JIT\n";

    dynasm!(ops
        ; .arch x64
        ; ->hello:  // 字符串label名为 hello
        ; .bytes s
    );

    let oft = ops.offset(); // 字符串地址偏移
    dynasm!(ops
        ; lea rdi, [->hello]                // 将字符串地址存储在 rdi 中
        ; mov rsi, QWORD s.len() as _       // 将字符串长度存储在 rsi 中
        ; mov rax, QWORD print as _         // 将 print 函数地址放入 rax
        ; call rax                          // 调用函数
        ; ret                               // 返回
    );

    let asm = ops.finalize().unwrap();

    let hello_fn: unsafe extern "sysv64" fn() -> u8 = unsafe {
        // 得到调用函数的汇编便宜地址，并将其作为函数地址返回
        std::mem::transmute(asm.ptr(oft))
    };

    let ret = unsafe { hello_fn() };

    assert_eq!(ret, 0);
}
