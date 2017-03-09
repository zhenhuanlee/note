# 理解Unix进程(Working with Unix Processes)

## 基础知识

### 系统调用
Unix系统的组成，具体来说就是用户空间(userland)和内核  
系统调用：内核和用户控件搭建的桥梁  
所有的程序都运行在用户空间  
总的来说，系统调用允许你的用户控件通过内核间接地与计算机硬件进行交互。

### 命名法, wtf(2)
Unix手册
```
$ man 2 getpid
$ man 3 malloc
$ man find # 等同于 man 1 find
```
### 进程：Unix之本
所有的代码都是在进程中执行的  
pid唯一标识进程
```ruby
puts Process.pid
```
进程都有父进程(ppid)，特定进程的父进程就是调用它的那个

```ruby
puts Process.ppid
```
父进程在检测守护进程时有重要作用  

### 万物皆文件
甚至可以将设备视为文件，套接字，管道等也都是  
当然，一般把上面的称之为资源  
无论何时在进程中打开一个资源，都会得到一个文件描述编号(file descriptor
number)。文件描述符不会在无关进程之间共享，只属于其所属的进程之中。进程结束后会被关闭  
> 在ruby中，IO类描述了打开的资源。任意一个IO对象都拥有一个相关联的文件描述符编号。可以使用`IO#fileno`进行访问
```ruby
passwd = File.open('/etc/passwd')
puts passwd.fileno
```
1. 所分配的文件描述编号是尚未使用的最小的数值  
2. 资源一旦关闭，对应的文件描述符编号就又可以使用了

#### 标准流  
每个Unix进程都有三个打开的资源，它们是标准输入(STDIN)、标准输出(STDOUT)和标准错误(STDERR)  
- STDIN 提供一种从键盘或管道中读取输入的通用方法  
- STDOUT和STDERR提供了一种向显示器、文件、答应机
```
puts STDIN.fileno     # 0
puts STDOUT.fileno    # 1
puts STDERR.fileno    # 2
```

### 进程皆有资源限制

