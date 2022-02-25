#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>

int main(int argc, char *argv[])
{
    // 机器码：
    //   mov eax, 0
    //   ret
    unsigned char code[] = {0xb8, 0x00, 0x00, 0x00, 0x00, 0xc3};

    if (argc < 2)
    {
        fprintf(stderr, "Usage: jit1 <integer>\n");
        return 1;
    }

    // 把用户给出的数值写入机器码，覆盖立即数 "0"
    //   mov eax, <user's value>
    //   ret
    int num = atoi(argv[1]);
    memcpy(&code[1], &num, 4);

    // 分配可写可执行内存
    // 注意：真实的程序不应该映射同时可写可执行的内存，
    // 这里有安全风险。
    void *mem = mmap(NULL, sizeof(code), PROT_WRITE | PROT_EXEC,
                     MAP_ANON | MAP_PRIVATE, -1, 0);
    memcpy(mem, code, sizeof(code));

    // 定义一个函数指针指向机器码内存，再执行函数
    int (*func)() = mem;
    int ret = func();
    printf("ret: %d\n", ret);

    return 0;
}
