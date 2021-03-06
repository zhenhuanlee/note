# [编译器原理](http://blog.jobbole.com/114466/)
### 编译器是什么
- 编译器是一个软件，编译器读入一个文本文件，经过大量的处理，最终产生一个二进制软件  

### 编译器是做什么的
因为从复杂的代码直接转化为二进制会很复杂，所以编译器在产生可运行程序之前有很多步骤：  
1. 从给定的源代码中读取单个词  
2. 把这些词按照单词、数字、符号、运算符进行分类  
3. 通过模式匹配从分类好的单词中找出运算符，明确这些运算符想进行的运算，然后产生一个运算符的树(表达式树)  
4. 最后一步便利表达式树种的所有运算符，产生相应的二进制数据  
其实从表达式树到二进制的过程中，还会产生汇编代码，之后汇编代码会被汇编/编译到二进制数据。汇编程序就好比一种高级的、人类可读的二进制。  

### 解释器是什么
解释器非常像编译器，它也是读入编程语言的代码，然后处理这些胆码。但是会跳过代码生成，然后即时编译并执行AST。解释器最大的优点就在于快。编译一个程序要一秒到几分钟不等，而解释器可以立刻开始执行程序，而不必编译。缺点是必须要安装在用户电脑上才能运行  

1. 词法分析  
第一步是把输入一个词一个词的拆分开。这一步叫**分词**。这一步的关键就在于**把字符组合成我们需要的单词、标识符、符号等**。比如`2+2`这个表达式只有三种标记，一个数字2，一个加号，另一个数字2  

2. 解析  
解析器是语法解析的核心。解析器提取由词法分析器产生的标记，并尝试判断它们是否符合特定的模式，然后把这些模式与函数调用，变量调用，数学运算之类的表达式关联起来  
`int a=3`和`a: int=3`的区别在于分析器的处理上面。解析器决定了语法的外在形式是怎么样的。它确保括号和花括号的左右括号是数量平衡的，每个语句借位都有一个分好，每个函数都有一个名称。当标记不符合预期的模式时，解析器就会知道标记的顺序不正确  
最常见的类型解析器是从上到下的**递归降解的解析器**。递归降解的解析器是用起来最简单也是最容易理解的解析器。  
解析器解析的语法可以使用一种语法表示出来。像EBNF这样的语法就可以描述一个解析器用于解析简单的数学运算，如`12+3`  
```
expr = additive_expr;
additive_expr = term, ('+' | '-'), term;
term = number;
```

解析器在解析时产生的树状结构被称为**抽象语法树(AST)**。ast中包含了所有要进行的操作。解析器只是以正确的顺序来收集其中的标记  
![](http://wx3.sinaimg.cn/large/7cc829d3ly1fwjgj2x9uij20ky0cljs4.jpg)  

3. 生成代码  
**代码生成器**接受一个AST，然后生成相应的代码或者汇编代码。代码生成器必须以递归下降的顺序遍历AST中的所有内容，就像是解析器的工作方式一样，之后生成相应的内容，只不过这里生成的不再是语法树，而是代码  
[Godbolt Compiler Explorer](https://godbolt.org)是一个高级到汇编的工具。  
[堆栈的知识](https://norasandler.com/2018/01/08/Write-a-Compiler-5.html)  

后端指的是编译器的代码生成器或者表达式解析器；因此前端是词法分析器和解析器。同样也有一个中间端，它通常与优化和IR有关，这部分会在稍后解释。后端通常与前段无关，后端只关心它接受到的AST。这意味着可以为集中不同的前段或者语言重用相同的后端。比如大名鼎鼎的GNU Compiler Collection

在生成汇编代码之后，这些汇编代码会被写到一个新的汇编文件中(.s或.asm)。然后该文件会被传递给汇编器，**汇编器**是汇编语言的编译器，它会生成相应的二进制代码。之后这些二进制代码会被写入到一个新的目标文件中(.o)  

目标文件是机器码，但是它们并不可以被执行。为了让他们变成可执行文件，目标文件需要被链接到一起。连接器读取通用的机器码，然后使他们编程一个可执行文件，共享库或者静态库  

**链接器**是因操作系统而不同的应用程序。随便一个第三方的链接器都应该可以编译你后端产生的目标代码。因此在写编译器的时候不需要创建你自己的链接器  

编译器可能叫[Intermediate representation](https://en.wikipedia.org/wiki/Intermediate_representation)或者简称IR。IR主要是为了在优化或者翻译成另一门语言的时候，无损的标识原来的指令  
