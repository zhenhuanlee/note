# [Makefile](https://www.cnblogs.com/wang_yb/p/3990952.html)

- 基本格式

  ```makefile
  target ... : prerequisites ...
  	command
  	...
  	...
  ```

  - target: 目标文件，可以是Object File，也可以是可执行文件
  - prerequisites: 生成target所需要的文件或者目标 
  - command: make需要执行的命令，Makefile中的命令必须以[tab]开头  

- GNU make的工作方式

  1. 读入主Makefile
  2. 读入被include的其他Makefile
  3. 初始化文件中的变量
  4. 推到隐晦规则，并分析所有规则
  5. 为所有的目标文件创建依赖关系链
  6. 根据依赖关系，决定哪些目标要重新生成
  7. 执行生成命令



## Makefile 初级语法

- 规则主要有两部分：依赖关系 和 生成目标的方法

- 通配符

  - \*：表示任意一个或多个字符
  - ?：表示任意一个字符
  - [...]：
    - [abcd] 表任意一个
    - [^abcd] 表除abcd以外的字符
    - [0-9] 表0-9中任意一个

- 路径搜索

  - VPATH：一个特殊的变量，如果当前目录中没有相应文件，Makefile会到这个目录下去查找

    - vpath \<directories> ：当前目录中找不到文件时，就从directories中搜索 
    - vpath \<pattern> \<directories> ：符合pattern格式的文件就从directories中搜索
    - vpath \<pattern> ：清除符合pattern格式的文件搜索路径
    - vpath ：清除所有文件路径

    ```makefile
    # 当前目录中找不到的文件，按顺序从src目录中查找
    VPATH src:../parent-dir
    # .h结尾的文件，从 headers中查找
    VPATH %.h ./header
    # 清除上一个设置
    VPATH %.h
    # 清除所有的
    VPATH
    ```

- 变量

  > `=`和`:=`的区别在于，`:=`只能使用之前定义好的变量，`=``可以使用后面定义的变量

  ```makefile
  # =
  OBJS2 = $(OBJS1) programC.o
  OBJS1 = programA.o programB.o
  
  # :=
  OBJS2 := $(OBJS1) programC.o
  OBJS1 := programsA.o programB.o
  # 会输出空
  
  ```

  - 变量替换`=` & `:=`

    ```makefile
    SRCS := programA.c programB.c programC.c
    OBJS := $(SRCS:%.c=%.o)
    
    all:
    	@echo "SRCS: " $(SRCS)
    	@echo "OBJS: " $(OBJS)
    ```

  - 变量追加`+=`

    ```makefile
    SRCS := programA.c programB.c
    SRCS += programC.c
    ```

  - 变量覆盖`override`

    - override \<variable> = \<value>
    - override \<variable> := \<value>
    - override \<variable> += \<value>

    ```makefile
    # 没有override
    SRCS := programA.c
    $ make SRC=nothing  #=> nothing
    
    override SRCS := programA.c
    $ make SRCS=nothing #=> programA.c
    ```

  - 目标变量

    > 作用是使变量的作用域仅限于这个目标，而不像之前例子中定义的变量，对整个Makefile都有用

    - \<target...> :: \<variable-assignment>
    - \<target...> :: override \<variable-assigment>

    ```makefile
    # Makefile 内容
    SRCS := programA.c 
    
    target1: TARGET1-SRCS := programD.c
    target1:
    	@echo "SRCS: " $(SRCS)
    	@echo "SRCS: " $(TARGET1-SRCS)
    	
    target2:
    	@echo "SRCS: " $(SRCS)
    	@echo "SrCS: " $(TARGET1-SRCS)
    	
    # bash 中执行make
    $ make target1
    # SRCS:  programA.c
    # SRCS:  programD.c
    $ make target2
    # SRCS:  programA.c
    # SRCS:
    ```

  - Makefile命令前缀

    > Makefile中书写shell命令时，可以加两种前缀`@`和`-`或者不加前缀
    >
    > - 不用前缀：输出执行的命令以及命令执行的结果，出错的话停止执行  
    > - 前缀`@`：只输出命令执行的执行结果，出错的话停止执行  
    > - 前缀`-`：命令执行有错的话，忽略错误，继续执行  

  - 伪目标

    > 伪目标并不是一个目标(target)，不像真正的目标那样会生成一个目标文件
    >
    > 典型的伪目标是Makefile中用来清理编译过程中中间文件的clean伪目标，一般格式如下  

    ```makefile
    .PHONY: clean  # 没有这句也行，但是最好加上
    clean:
    	-rm -f *.o
    ```

  - 引用其他Makefile

    > 语法：include <filename> （filename可以包含通配符和路径）

    ```makefile
    all:
    	@echo "主 makefile begin"
    	@make other-all
    	@echo "主 makefile end"
    	
    include ./other/makefile
    
    # ./other/makefile中
    other-all:
    	@echo "other makefile begin"
    	@echo "other makefile end"
    ```

  - 查看C文件的依赖关系

    ......

  - make退出码

    - 0 表示成功 执行
    - 1 表示make命令出现了错误
    - 2 使用`-q`选项，并且make使得一些目标不需要更新

  - 指定Makefile，指定特定目标

    默认执行make命令时，GNU make在当前目录下依次搜索三个文件，`GNUmakefile`,`makefile`,`Makefile`  

    找到对应文件之后，就开始执行文件中的第一个目标，如果找不到这三个文件就报错  

    非默认情况下，可以在make命令中指定特定的Makefile和特定的目标  

    ```makefile
    $ make -f MyMakefile target # 指定makefile和target
    ```

  - make参数介绍

    - `--debug[=<options>]  `  输出make的调试信息，options可以是a, b, v
    - `-j --jobs` 同时运行的参数的个数，也即是多线程执行Makefile
    - `-r --no-builtin-rules` 禁止使用任何隐含规则
    - `-R --no-builtin-variables` 禁止使用任何作用域变量上的隐含规则
    - `-B --always-make` 假设所有目标都有更新，即强制重编译

  - Makefile隐含规则

    ```makefile
    all:
    	@echo $(RM)
    	@echo $(AR)
    	@echo $(CC)
    	@echo $(CXX)
    # 输出
    # rm -f
    # ar
    # cc
    # g++
    ```

    | 自动变量 | 含义                                                 |
    | :------- | ---------------------------------------------------- |
    | $@       | 目标集合                                             |
    | $%       | 当目标是函数库文件时，表示其中的目标文件名           |
    | $<       | 第一个依赖目标，如果依赖目标是多个，逐个表示依赖目标 |
    | $?       | 比目标新的依赖目标的集合                             |
    | $^       | 所有依赖目标的集合，会去除重复的依赖目标             |
    | $+       | 所有依赖目标的集合，不会去除重复的依赖目标           |
    | $*       | 这个是GNU make特有，其他不一定支持                   |

  ## Makefile 高级语法

  - 嵌套Makefile

    > 在上面，已经有引用其他Makefile，但是还有另一种写法，可以向引用的其他Makefile传递参数

    - 不传递参数

      ```makefile
      all:
      	@echo "main"
      	@cd ./other && make
      
      # other 中makefile
      other-all:
      	@echo "other"
      ```

    - 传递参数

      ```makefile
      export VALUE1 := export.c # 用了export，此变量能够传递到其他makefile中
      VALUE2 := no-export.c
      
      all:
      	@echo "main"
      	@cd ./other && make
      	
      # other 中 makefile
      other-all:
      	@echo "other"
      	@echo $(VALUE1)  # export.c
      	@echo $(VALUE2)  # none
      ```

    - 定义命令包

      > 命令包有点像是个函数，将连接的相同的命令合成一条，减少Makefile中的代码量，便于以后维护

      ```makefile
      define <command-name>
      command
      ...
      enddef
      
      define run-hello-makefile
      @echo -n "hello"
      endef
      
      all:
      	$(run-hello-makefile)
      ```

    - 条件判断

      主要有`ifeq` `ifneq` `ifdef` `ifndef`  

      ```makefile
      all:
      ifeq ("aa", "bb")
      	@echo "equal"
      else
      	@echo "not equal"
      endif
      ```

      ```makefile
      SRCS := program.c
      all:
      ifdef SRCS
      	@echo $(SRCS)
      else
      	@echo "no SRCS"
      endif
      ```

    - 函数

      函数的调用方法如下：  

      ```makefile
      $(<function> <arguments>)
      # or
      ${<function> <arguments>}
      # <function> 是函数名
      # <arguments> 是函数参数
      ```

      - 字符串替换函数

        ```makefile
        all:
        	@echo $(subst t,e,maktfilt)  # 不要空格
        ```

      - 模式字符串替换函数

        ```makefile
        all:
        	@echo $(patsubst %.c,%.o,programA.c programB.c)
        ```

      - 去空格函数

        ```makefile
        all:
        	@echo $(strip "  a  sd  f  ")
        # 只能去首位空格
        ```

      - 查找字符串函数

        ```makefile
        # 如果找到返回子字符串，否则返回空字符串
        all:
        	@echo $(findstring aa,"  aabb bb cc")
        	@echo $(findstring ac,"  aabb bb cc")
        ```

      - 过滤函数

        ```makefile
        all:
        	@echo $(filter %.o %.a,program.a program.o program.c)
        ```

      - 反过滤函数

        ```makefile
        all:
        	@echo $(filter-out %.o %.a,program.c program.o program.a)
        ```

      - 排序函数

        ```makefile
        @echo $(sort bac abc acb cab)
        ```

      - 取单词函数

        ```makefile
        @echo $(word 1,aa bb cc dd) # aa
        @echo $(word 5,aa bb cc dd) # 空
        @echo $(word 4,aa bb cc dd) # dd
        ```

      - 取单词串函数

        ```makefile
        @echo $(wordlist 1,3,aa bb cc dd) # aa bb cc
        @echo $(word 5,6,aa bb cc dd) # 空
        @echo $(word 2,5,aa bb cc dd) # bb
        ```

      - 单词个数统计函数

        ```makefile
        @echo $(words aa bb cc dd) # 4
        @echo $(words aabbccdd) # 1
        @echo $(words ) # 0
        ```

      - 首单词函数

        ```makefile
        @echo $(firstword aa bb cc dd) # aa
        @echo $(firstword aabbccdd) # aabbccdd
        @echo $(firstword ) # 空
        ```

      - 文件名函数

        ```makefile
        # 从文件名中取出目录部分
        @echo $(dir /home/a.c ./bb.c ../c.c d.c)
        # /home/ ./ ../ ./
        ```

      - 取文件函数

        ```makefile
        # 从文件名中取出非目录部分
        @echo $(notdir /home/a.c ./bb.c ../c.c d.c)
        # a.c bb.c c.c d.c
        ```

      - 取后缀函数

        ```makefile
        @echo $(suffix /home/a.c ./b.o ../c.a d)
        # .c .o .a
        ```

      - 取前缀函数

        ```makefile
        @echo $(basename /home/a.c ./b.o ../c.a /home/.d .e)
        # /home/a ./b ../c /home/
        ```

      - 加后缀函数

        ```makefile
        @echo $(addsuffix .c,/home/a b ./c.o ../d.c)
        # /home/a.c b.c ./c.o.c ../d.c.c
        ```

      - 加前缀函数

        ```makefile
        @echo $(addprefix test_,/home/a.c b.c ./d.c)
        # test_/home/a.c test_b.c test_./d.c
        ```

      - 连接函数

        ```makefile
        @echo $(join a b c d,1 2 3 4)
        @echo $(join a b c d,1 2 3 4 5)
        @echo $(join a b c d e,1 2 3 4)
        # a1 b2 c3 d4
        # a1 b2 c3 d4 5
        # a1 b2 c3 d4 e
        ```



      - foreach

        ```makefile
        targets := a b c d
        objects := $(foreach i,$(targets),$(i).o)
        all:
        	@echo $(targets)
            @echo $(objects)
        # a b c d
        # a.o b.o c.o d.o
        ```

      - if

        > 这里的if是个函数，和前面的条件判断不一样，前面的条件判断属于Makefile的关键字

        ```makefile
        val := a
        objects := $(if $(val),$(val).o,nothing)
        no-objects := $(if $(no-val),$(val).o,nothing)
        
        all:
            @echo $(objects)
            @echo $(no-objects)
        # a.o
        # nothing
        ```

      - call 创建新的参数化函数

        ```makefile
        log = "====debug====" $(1) "====end===="
        all:
            @echo $(call log,"正在 Make")
        # ====debug==== 正在 Make ====end====
        ```

      - origin 判断变量的来源

        | 类型         | 含义                                      |
        | ------------ | ----------------------------------------- |
        | undefined    | 没有定义过                                |
        | default      | 是个默认的定义, 比如 CC 变量              |
        | environment  | 是个环境变量, 并且 make时没有使用 -e 参数 |
        | file         | 定义在Makefile中                          |
        | command line | 定义在命令行中                            |
        | override     | 被 override 重新定义过                    |
        | automatic    | 是自动化变量                              |

        ```makefile
        @echo $(origin not-define)
        ```

      - shell

        > 执行一个shell命令，并将shell命令的结果作为函数的返回

        ```makefile
        $(shell command)
        ```

      - make 控制函数  

        - error 产生一个致命错误

          ```makefile
          $(error there is an error!)
          ```

        - warn 输出警告

          ```makefile
          $(warning there is an warning!)
          ```

    - makefile中一些GNU约定俗成的伪目标

      > 经常会用到make clean, install 这些伪目标

      | 伪目标        | 含义                                                         |
      | ------------- | ------------------------------------------------------------ |
      | all           | 所有目标的目标，其功能一般是编译所有的目标                   |
      | clean         | 删除所有被make创建的文件                                     |
      | install       | 安装已编译好的程序，其实就是把目标可执行文件拷贝到指定的目录中去 |
      | print         | 列出改变过的源文件                                           |
      | tar           | 把源程序打包备份. 也就是一个tar文件                          |
      | dist          | 创建一个压缩文件, 一般是把tar文件压成Z文件. 或是gz文件       |
      | TAGS          | 更新所有的目标, 以备完整地重编译使用                         |
      | check 或 test | 一般用来测试makefile的流程                                   |
