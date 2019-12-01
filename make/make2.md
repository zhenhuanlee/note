# makefile 脚本语言
#### 第一层：显示规则
1. 目标文件:依赖文件  
```makefile
# hello.i是目标文件，hello.c是依赖文件
# 简单的c编译过程
hello.i:hello.c # 预处理
    gcc -E hello.c -o hello.i

hello.S:hello.i # 汇编
    gcc -S hello.i -o hello.s

hello.o:hello.S # 编译
    gcc -c hello.S -o hello.o

hello:hello.o # 链接
    gcc hello.o -o hello
```

2. 第一个目标文件是最终目标  
```makefile
# 最终目标是hello
# 这就相当于递归，每次都去找缺少的文件
hello:hello.o # 链接，假设-如果文件中直接有个hello.o，那会出问题的
    gcc hello.o -o hello

hello.o:hello.S # 编译
    gcc -c hello.S -o hello.o

hello.S:hello.i # 汇编
    gcc -S hello.i -o hello.s

hello.i:hello.c # 预处理
    gcc -E hello.c -o hello.i
```

3. 伪目标: 没有目标文件，只是想得到写东西  
```makefile
.PHONY:
    clean:
        rm -rf hello.o hello.s hello.i hello
# 在命令行中执行  make clean
```

#### 第二层- 变量
- `=` 替换  
`TAR = test test1 test2`
- `+=` 追加  
`TAR += test1`
- `:=` 恒等于，常量  
`cc := gcc`
- `$(xx)` 使用变量  

```makefile
TAR = test
OBJ = circle.o cube.o main.o
CC := gcc

$(TAR):$(OBJ)
    $(cc) $(OBJ) -O $(TAR)
```

#### 第三层 隐含规则
- `%.c`, `%.o` 任意的 .c 和 .o 文件  
- `*.c`, `*.o` 所有的 .c 和 .o 文件  
```makefile
# 任意的.c文件，输出.o文件  
%.o:%.c
    gcc -c %.c -o %.o
```

#### 第四层 通配符
- $^ 所有的依赖文件，就是冒号前面的  
- $@ 所有的目标文件，就是冒号后面的
- $< 所有的依赖文件的第一个文件  
```makefile
TAR=test
OBJ=circle.o cube.o main.o
CC:=gcc
RMRF:=rm -rf

$(TAR):$(OBJ)
    $(CC) $^ -o $@

%.o:%.c
    $(CC) -c $^ -o $@
```

#### 第五层 函数
