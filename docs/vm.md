# 虚拟机设计

## 字节码设计

| 字节码           | 描述                                               |
| ---------------- | -------------------------------------------------- |
| Push (isize)     | 将参数入栈                                         |
| Pop              | 出栈，返回栈顶数据                                 |
| Incr             | 栈顶数据++                                         |
| Decr             | 栈顶数据--                                         |
| Add              | 出栈两个数据，相加后再入栈                         |
| Sub              | 出栈两个数据，相减后再入栈                         |
| Mul              | 出栈两个数据，相乘后再入栈                         |
| Div              | 出栈两个数据，相除后再入栈                         |
| Jump (label)     | 跳转 label，设置 label 为 ip                       |
| JNE (label)      | 栈顶 != 0，跳转 label                              |
| JE (label)       | 栈顶 == 0，跳转 label                              |
| JGT (label)      | 栈顶 > 0，跳转 label                               |
| JLT (label)      | 栈顶 < 0，跳转 label                               |
| JGE (label)      | 栈顶 >= 0，跳转 label                              |
| JLE (label)      | 栈顶 <= 0，跳转 label                              |
| Call (procedure) | 跳转 procedure, 将 stack offset 设置为当前栈的长度 |
| Get (usize)      | 获得栈中 index 处的数据，将其拷贝到栈顶            |
| Set (usize)      | 将栈顶数据拷贝到栈的 index 处                      |
| GetArg (usize)   | 根据参数序号、stack offset 来取 callee 参数        |
| SetArg (usize)   | 根据参数序号、stack offset 来设置 callee 参数      |
| Noop             | 啥也不做                                           |
| Print            | 打印栈顶数据                                       |
| Ret              | 返回 procedure，重新设置 ip                        |
