# [WebAssembly](https://rustwasm.github.io/book/what-is-webassembly.html)
WebAssembly(wasm)是一个具有广泛规范的简单的机器模型和可执行模式，它被设计成轻便的，紧凑的，执行效率近似于原始速度  

作为一个编程语言，wasm由两种表示相同结构的格式组成，尽管方式不同  
1. `.wat`text格式，使用`S-expressions`，和Lisp有一些相似之处   
2. `.wasm`binary格式，是一个底层的旨在直接被wasm虚拟机消费  

#### Linear Memory  
WebAssemly有一个非常简单的内存模型，一个wasm模块可以访问单个`线性内存`,这个线性内存实际上是一个bytes的数组。这个内存可以以pagesize(64k)的倍数增长，不能收缩  




