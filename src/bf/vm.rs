use std::io::{Read, Write};
use std::path::Path;
use std::ptr;

use dynasm::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi};

use crate::bf::compile::{compile, optimize};
use crate::bf::error::{Result, RuntimeError, VMError};
use crate::bf::opcode::BfIR;

const MEMORY_SIZE: usize = 4 * 1024 * 1024;

pub struct BfVM<'io> {
    code: dynasmrt::ExecutableBuffer, // 汇编流
    start: dynasmrt::AssemblyOffset,  // 开始地址
    memory: Box<[u8]>,                // 内存
    input: Box<dyn Read + 'io>,       // 输入
    output: Box<dyn Write + 'io>,     // 输出
}

#[inline(always)]
fn vm_error(re: RuntimeError) -> *mut VMError {
    let e = Box::new(VMError::from(re));
    Box::into_raw(e)
}

impl<'io> BfVM<'io> {
    pub fn new(
        file_path: &Path,
        input: Box<dyn Read + 'io>,
        output: Box<dyn Write + 'io>,
        optimized: bool,
    ) -> Result<Self> {
        let src = std::fs::read_to_string(file_path)?;
        let mut ir = compile(&src)?;

        if optimized {
            optimize(&mut ir);
        }

        let (code, start) = Self::generate(&ir)?;
        let memory = vec![0; MEMORY_SIZE].into_boxed_slice();

        Ok(Self {
            code,
            start,
            memory,
            input,
            output,
        })
    }

    // Checks for casts of a function pointer to a numeric type except usize.
    #[allow(clippy::fn_to_numeric_cast)]
    fn generate(code: &[BfIR]) -> Result<(dynasmrt::ExecutableBuffer, dynasmrt::AssemblyOffset)> {
        let mut ops = dynasmrt::x64::Assembler::new()?;
        let start = ops.offset(); // 开始地址

        // 当作栈来使用
        let mut loops = vec![];

        // 下面是生成的汇编代码，并不是直接调用：
        // sysv64 调用约定规定 rdi, rsi, rdx, rcx 存放前四个整数参数，rax 存放返回值
        // agr0: vm
        // agr1: memory_start
        // agr2: memory_end
        // vm:         rdi r12
        // memory_start: rsi r13
        // memory_end:   rdx r14
        // ptr:          rcx r15
        dynasm!(ops
            ; push rax       // 保存 rax 的值
            ; mov r12, rdi   // save vm, r12 = rdi
            ; mov r13, rsi   // save memory_start
            ; mov r14, rdx   // save memory_end
            ; mov rcx, rsi   // ptr = memory_start, rcx = rsi
        );

        use BfIR::*;
        for &ir in code {
            match ir {
                AddPtr(x) => dynasm!(ops
                    ; add rcx, x as i32     // ptr += x
                    ; jc  ->overflow        // jmp if overflow
                    ; cmp rcx, r14          // ptr - memory_end
                    ; jnb ->overflow        // jmp if ptr >= memory_end
                ),
                SubPtr(x) => dynasm!(ops
                    ; sub rcx, x as i32     // ptr -= x
                    ; jc  ->overflow        // jmp if overflow
                    ; cmp rcx, r13          // ptr - memory_start
                    ; jb  ->overflow        // jmp if ptr < memory_start
                ),
                AddVal(x) => dynasm!(ops
                    ; add BYTE [rcx], x as i8    // *ptr += x
                ),
                SubVal(x) => dynasm!(ops
                    ; sub BYTE [rcx], x as i8    // *ptr -= x
                ),
                GetByte => dynasm!(ops
                    ; mov  r15, rcx         // save ptr
                    ; mov  rdi, r12
                    ; mov  rsi, rcx         // arg0: this, arg1: ptr
                    ; mov  rax, QWORD BfVM::getbyte as _
                    ; call rax              // getbyte(this, ptr)
                    ; test rax, rax
                    ; jnz  ->io_error       // jmp if rax != 0
                    ; mov  rcx, r15         // recover ptr
                ),
                PutByte => dynasm!(ops
                    ; mov  r15, rcx         // save ptr
                    ; mov  rdi, r12
                    ; mov  rsi, rcx         // arg0: this, arg1: ptr
                    ; mov  rax, QWORD BfVM::putbyte as _
                    ; call rax              // putbyte(this, ptr)
                    ; test rax, rax
                    ; jnz  ->io_error       // jmp if rax != 0
                    ; mov  rcx, r15         // recover ptr
                ),
                Jz => {
                    let left = ops.new_dynamic_label();
                    let right = ops.new_dynamic_label();
                    loops.push((left, right));

                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jz => right       // jmp if *ptr == 0
                        ; => left
                    )
                }
                Jnz => {
                    let (left, right) = loops.pop().unwrap();
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jnz => left       // jmp if *ptr != 0
                        ; => right
                    )
                }
            }
        }

        dynasm!(ops
            ; xor rax, rax // rax = 0
            ; jmp >exit    // jmp => exit
            ; -> overflow: // 定义 overflow
            ; mov rax, QWORD BfVM::overflow_error as _
            ; call rax
            ; jmp >exit
            ; -> io_error: // 定义 io_error
            ; exit:       // 定义 exit
            ; pop rdx
            ; ret
        );

        let code = ops.finalize().unwrap();

        Ok((code, start))
    }

    pub fn run(&mut self) -> Result<()> {
        type RawFn = unsafe extern "sysv64" fn(
            vm: *mut BfVM<'_>,
            memory_start: *mut u8,
            memory_end: *const u8,
        ) -> *mut VMError;
        // 将内存重新解释为函数
        let raw_fn: RawFn = unsafe { std::mem::transmute(self.code.ptr(self.start)) };

        let vm: *mut Self = self;
        let memory_start = self.memory.as_mut_ptr();
        let memory_end = unsafe { memory_start.add(MEMORY_SIZE) };
        let ret: *mut VMError = unsafe { raw_fn(vm, memory_start, memory_end) };

        if ret.is_null() {
            Ok(())
        } else {
            Err(*unsafe { Box::from_raw(ret) })
        }
    }

    // getbyte 读取字节
    unsafe extern "sysv64" fn getbyte(vm: *mut Self, ptr: *mut u8) -> *mut VMError {
        let mut buf = [0_u8];
        let vm = &mut *vm;
        match vm.input.read(&mut buf) {
            Ok(0) => {}
            Ok(1) => *ptr = buf[0],
            Err(e) => return vm_error(RuntimeError::IO(e)),
            _ => unreachable!(),
        }
        ptr::null_mut()
    }

    // putbyte 输出字节
    unsafe extern "sysv64" fn putbyte(vm: *mut Self, ptr: *const u8) -> *mut VMError {
        let buf = std::slice::from_ref(&*ptr);
        let vm = &mut *vm;
        match vm.output.write_all(buf) {
            Ok(()) => ptr::null_mut(),
            Err(e) => vm_error(RuntimeError::IO(e)),
        }
    }

    // overflow_error 溢出
    unsafe extern "sysv64" fn overflow_error() -> *mut VMError {
        vm_error(RuntimeError::PointerOverflow)
    }
}
