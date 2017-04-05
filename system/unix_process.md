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
```ruby
p Process.getrlimit(:NOFILE)
[7168, 9223372036854775807]
# [软限制， 硬限制]
# 超过软限制，只会异常，且这个值可以修改
# 只有超级用户能修改硬限制，但是通常这个值并没有意义
```

## 进程皆有环境
这里的环境指的是：环境变量（包含进程数据的键-值对`key-value pairs`）  
所有进程都从父进程处继承环境变量，他们由父进程设置并被子进程继承。每一个进程都有环境变量，环境变量对于特定进程而言是全局性的  
如何在bashshell中设置一个环境变量
```shell
$ MESSAGE='wing it'
$ $MESSAGE   # 访问这个环境变量
```
`VAR=value`这是在bash中设置环境变量的方法，也可以在ruby中使用`ENV`常量来实现  
```ruby
# 用最少的输入来完成同一件事
ENV['MESSAGE'] = 'wing it'
system "ECHO #MESSAGE"
```
shell中使用
```shell
$ printenv   # 打印部分或所有的环境变量
$ set        # 设置shell选项
$ export     # 导出环境变量，让随后执行的程序知道
$ alias      # 创建命令别名
```
一般我们修改.bash_profile文件来修改PATH变量或者额外定义环境变量

### 这个是散列么
虽然是key-value形式，但并非Hash，它只实现了Enumerable和部分Hash API

### 实践领域
```shell
$ RAILS_ENV=production rails server
$ EDITOR=mate bundle open actionpack
$ QUEUE=default rake resque:work
```
环境变量京城作为一种将输入传递到命令行程序中的通用方法，所有的终端(Unix或Windows)均已支持环境变量  
比起解析命令行选项，使用环境变量的开销通常更小

## 进程皆有参数
所有进程都可以访问名为ARGV(argument vector)的特殊数组，ARGV就是一个参数向量或数组。它保存了命令行中传给当前进程的参数。
```shell
$ cat argv.rb
p ARGV
$ ruby argv.rb foo bar -va
["foo", "bar", "-va"]
```

### 这是个数组
ARGV只是一个Array

### 实践领域
最常用的大概就是将文件名传入程序。

## 进程皆有名
Unix进程几乎没有什么固有的方法来获悉彼此的状态  
解决之道：  
1. 日志文件，向文件系统写入信息来了解彼此的状态信息，这术语文件系统层面，而非进程本身所固有   
2. 进程可以接住网络来打开套接字同其他进程进行通信，依靠网络，也非进程本身所固有
3. 有两种晕坐在进程本身层面上的机制可以用来互通信息，进程名称，退出码  

### 进程名称
系统中每一个进程都有名称，如一个irb会话，对应的进程就获得了"irb"的名称。进程名的妙处在与它可以在运行期间被修改并作为一种通信手段  
ruby中可以在变量$PROGRAM_NAME中获得或修改当前进程的名称
```ruby
puts $PROGRAM_NAME
$PROGRAM_NAME = "PROCESS 1"
```

### 实践领域
***附录1***

## 进程皆有退出码
当进程即将结束时，它还有最后一线机会留下自身的信息：退出码  
所有进程在退出的时候都带有数字退出码(0-255)，用于致命进程是否顺利结束  
按照惯例，_退出码为0的进程被认为是顺利结束_；其他退出码则表明进程出现了错误  

### 如何退出进程
在Ruby中有多种方法可以退出进程  
- exit：退出最简单的方法是使用`Kernel#exit`，虽然脚本没有明确指出使用`exit`结束，但是幕后用的就是这种方式  
```ruby
# 这将是你的程序携带顺利状态码(0)退出
exit

# 你可以给这个方法传递一个定制的退出码
exit 22

# 当Kernetl#exit被调用时，在退出之前，Ruby会调用由Kernel#at_exit所定义的全部语句块
at_exit { puts 'Last!' }
exit
=> exit
```
- exit!  
```ruby
# exit! 几乎和Kernel#exit一模一样，但是有两处不同：
# 它将错误状态码设置为默认的1，但是仍可以传递一个退出码
exit! 33
# 它不会调用由Kernel#at_exit定义的状态码
```
- abort  
Kernel#abort 提供了一种从错误进程中退出的通用方法，Kernel#abort会将当前进程的退出码设置为1
```ruby
# 携带退出码1退出
abort

# 你可以传递一条信息给Kernel#abort，在进程退出之前，该消息会被打印到 STDERR
abort "Something went horribly wrong."

# 当使用Kernel#abort时，会调用Kernel#at_exit语句块
at_exit { puts 'Last!' }
abort "Something went horribly wrong."
=> Something went horribly wrong.
=> last!
```
- raise  
另一种结束进程的方法是使用一个未处理的异常  
它不会立刻结束进程，它只是抛出一个异常，如果没有代码对其进行处理，那么这个未处理的异常将会结束改进程  
以这种方式结束的进程仍然会调用at_exit处理程序，并向STDERR打印出异常消息和回溯  
```ruby
# 类似于abort，一个未处理的异常会将退出码设置为1
raise 'hell'
```

## 进程皆可衍生
### Luke，使用fork(2)
衍生(firking)(Unix编程中最强大的概念之一)。fork(2)系统调用允许运行中的进程以编程的形式创建新的进程  
这个新的进程和原始的进程一模一样  
进行衍生时，调用fork(2)的进程被称为"父进程"，新创建的进程被称为"子进程"  
_子进程从父进程处继承了其所占用内存中的所有内容，以及所有属于父进程的已打开的文件描述符，这样两个进程就可以共享打开的文件，套接字等等_  
子进程对于想要在内存中载入多个应用程序来说简直是完美方案。  
子进程可以随意改写其内存内容的副本，而不会对父进程造成任何影响。  
```ruby
puts "Parent process pid is #{Process.id}"
if fork
  puts "entered the if block from #{Process.pid}"
else
  puts "entered the else block from #{Process.pid}"
end
# 为什么子进程不会fork出新进程
```
输出是：
```ruby
parent process is 21268
entered the if block from 21268
entered the else block from 21282
```
if语句快在父进程中执行，else语句快的代码是子进程执行的。子进程执行完else块后退出，父进程则继续运行  
> 子进程中fork返回nil，父进程中fork返回新创建的子进程的pid

### 多核编程
通过生成新的进程，代码可以(不能完全保证)被分配到多个CPU核心中  
在配备4个CPU的系统中，如果衍生出4个新进程，那么这些进程会分别由不同的CPU来处理，从而实现多核并发(multicore concurrency)  
然而，并不保证他们会并行操作，在繁忙的系统中，有可能所有4个进程都由同一个CPU来处理  
> fork会占用内存，比如一个占用500MB，10个就是5000MB，这就是"fork炸弹"(fork bomb)_irb中进行fork操作时明显卡顿了，不知道是不是这个原因_

### 使用block
在ruby中更常见的是通过block来使用fork  
如果将一个block传递给fork方法，那么这个block将在新的子进程中继续执行，而父进程则会跳过block中的内容，子进程执行完block之后就会退出，它不会像父进程那样执行随后的代码  
```ruby
fork do
  # 此处的代码仅在子进程中执行
end
# 此处的代码仅在父进程中执行
```

### 实践领域
附录Spyglass项目

## 孤儿进程
### 失控
如果涉及子进程，就不能从终端来控制一切  
```ruby
fork do
  5.times do
    sleep 1
    puts "I'm an orphah1"
  end
end

abort "Parent process dieds..."

# 父进程结束后，立刻返回到终端命令提示符下，此时终端被子进程输出到STDOUT的内容重写！在进程进行衍生时，就会发生这些莫名其妙的事
```
