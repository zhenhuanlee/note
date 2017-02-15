# Ruby 原理剖析
## 分词与语法分析
ruby代码 -> 分词(tokenize) -> 解析(parse) -> 编译(compile) -> YARV指令  
- 分词：读取文件中的文本字符，把它们转换为词条(token)
- 解析：Ruby对词条进行语法解析，Ruby会把这些语句编译成有意义的Ruby 语句
- 编译：Ruby 把这些语句编译成底层指令(instruction)，以便在 Ruby 的虚拟机(Yet Another Ruby Virtual Machine)上运行


