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

补充：
```shell
ABC=abc           # 声明了一个自定义变量
echo $ABC         # abc
env | grep ABC    # 环境变量中没有
set | grep ABC    # aaa=bbb     自定义变量中有
export | grep ABC # export也没导出，导出变量也没有
export aaa        # 导出变量，变成环境变量
env | grep aaa    # ABC=abc     环境变量内存在
```
> Linux分shell变量(set)，用户变量(env)，shell变量包含用户变量，export是一种命令工具
是显示那些通过export命令把shell变量中包含的shell变量导入给用户变量的那些变量  

最根本的是修改 ~/.bash_profile  ~/.bashrc  ~/.bash_logout  
- ~/.bash_profile    用户登录时被读取，其中包含的命令被执行  
- ~/.bashrc          启动新的shell时被读取，执行  
- ~/.bash_logout       shell登录退出时读取  

shell(此处指bash)的初始化过程是这样的：  
1. bash检查文件/etc/profile是否存在  
2. 如果存在，bash就读取该文件，否则跳过  
3. bash检查主目录下的文件.bash_profile是否存在  
4. 如果存在，bash就 读取该文件，否则，跳过  
5. bash检查主目录下的.bash_login是否存在，有就读取，否则跳过  
6. bash检查主目录下的.profile是否存在，有就读取，否则跳过  

单引号，双引号，反单引号的作用：  
- "": 可以保留变量的内容    
- '': 只能是一般的字符  
- \`\`: 里面的内容会先执行  

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
所有进程在退出的时候都带有数字退出码(0-255)，用于指明进程是否顺利结束  
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
> fork会占用内存，比如一个占用500MB，10个就是5000MB，这就是"fork炸弹"(fork bomb)

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

### 弃子
当父进程结束后，子进程毫无影响，操作系统并不会对子进程区别对待  

### 管理孤儿
这里有两个有意思的概念：  
1. 守护进程：一个长时间运行的进程，为了一直保持运行，他们有意作为孤儿进程存在  
2. 与脱离终端会话的进程进行通信。可以使用Unix信号来做到这一点  

## 友好的进程
### 对CoW(copy-on-write)好点
fork创建了一个和父进程一模一样的子进程，它包含了父进程在内存中的一切内容  
但是为了减少开销，现代的Unix系统采用写时复制(CoW)的方法来实现  
CoW：将实际的内存复制操作推迟到真正需要写入的时候  
所以，父进程和子进程实际上实在共享内存中的数据，直到它们其中某一个需要对数据进行修改，届时才会进行内存复制  
```ruby
arr = [1,2,3]

fork do
  # 此时子进程已经完成了初始化
  # 借助CoW，子进程并不需要赋值变量arr，应为它并没有修改任何共享变量
  # 因此可以继续从和父进程同样的内存位置中进行读取
  p arr
end

arr = [1,2,3]
fork do
  # 此时子进程已经完成了初始化
  # 由于CoW，变量arr并不会被赋值
  arr << 4
  # 上面的代码修改了数组，因此在进行修改之前需要为子进程创建一个该数组的副本，父进程中的数组并不会被影响
end
```
这样就意味着更高，更快，更省  
但是MRI或Rubinius并不支持  
想要CoW正常运作，程序需要以一种CoW友好的方式来编写。也就是说，它们得用一种使CoW成为可能的方式来管理内存。MRI和Rubinius并不是采用这种方式来写的  
> 为何不可？  
> MRI的垃圾收集器使用了一种"标记-清除"(mark-and-sweep)的算法。这意味着当垃圾收集器被调用时，它必须对每个已知的对象进行迭代并写入信息，指出改对象是否应该被回收。关键在于垃圾收集器每运行一次，内存中的搜游对象都会被写入信息。  
> 因此，在进行衍生之后，每次进行垃圾手机的时候，写时赋值所带来的好处会被撤销  

这就是创建Ruby企业版(Ruby Enterprise Edition)的主要原因之一，REE是CoW友好的  

### MRI/RBX用户
-对于其他Ruby虚拟机来说，fork享受不到CoW的好处，子进程需要获取调用进程所战友的内存内容的完整副本-
Ruby2.0以提供CoW友好的垃圾收集器  

## 进程可待
前面所说的fork都是让父进程与子进程一道运行。有时候会导致一些奇怪的结果，比如当父进程先于子进程退出时  
这种情景只适用于一种用例：即发即弃(fire and forget)-如你希望让子进程异步的处理其他事务  
```ruby
message = 'Good Morning'
recipient = 'tree@mybackyard.com'

fork do
  # 在这个假象的例子中，父进程衍生出一个子进程来负责将数据发送给统计接收器
  # 父进程同事继续进行自己实际的数据发送工作

  # 父进程不希望资深被这个项目所拖缓，即使任务由于某种原因失败
  # 父进程也不会受到影响
  StatsColler.record message, recipient
end
# 发送信息给接收方
```

### 看顾(Babysitting)
对于其他多数涉及fork的用例来说，希望有一些能够监视子进程动向的方法。Ruby中，Process.wait就提供了
```ruby
fork do
  5.times do
    sleep 1
    puts 'I am an orphan!'
  end
end

process.wait
abort "Parent process died..."

# output
=begin
I am an orphan!
I am an orphan!
I am an orphan!
I am an orphan!
Parent process died...
=end
```
`Process.wait`是一个阻塞调用，该调用使得父进程一直等到它的某个子进程退出后才继续执行  

### Process.wati 一家子
`Process.wait`会一直保持堵塞，知道任意一个子进程退出位置。如果父进程拥有不止一个进程，并且使用了`Process.wait`，那么就需要知道究竟是哪个子进程退出了。这时可以用返回值来解决这个问题  
`Process.wait`会返回退出的那个子进程的pid  
```ruby
3.times do
  fork do
    sleep rand(5)
  end
end

3.times do
  puts Process.wait
end
```

### 使用Process.wait2进行通信
`Process.wait2`返回两个值(pid, status)  
通过退出码，状态可以用作进程间通信  
```ruby
5.times do
  fork do
    if rand(5).even?
      exit 111
    else
      exit 112
    end
  end
end

5.times do
  # 等待子进程逐个退出
  pid, status = Process.wait2
  # 如果子进程的退出码是111
  if status.exitstatus === 111
    puts "#{pid} encountered an even number!"
  else
    puts "#{pid} encountered an odd number!"
  end
end
```

### 等待特定的子进程
`Process.waitpid`和`Process.waitpid2`
等待特定由pid指定的子进程退出  
```ruby
favourite = fork do
  exit 77
end

middle_child = fork do
  abort "I want to be waited on!"
end

pid, status = Process.waitpid2 favourite
puts status.exitstatus
```
> 其实，`Process.wait`和`Process.waitpid`是一样的，可以传一个pid给`Process.wait`也可以传-1给`Process.waitpid`

### 竞争条件(race conditions)
当一个子进程退出时，处理某个退出进程的代码还在运行，这时候会怎样？如果还没来得及从`Process.wait`返回，另一个进程也退出了，又会怎么样？
```ruby
2.times do
  fork do
    # 两个子进程立即退出
    abort "Finished!"
  end
end

# 父进程等待第一个子进程，然后休眠5秒
# 期间第二个子进程也退出，不再运行
puts Process.wait
sleep 5

# 父进程会再等待一次，第二个子进程的退出信息会被加入队列并在此返回
puts Process.wait
```
这项技术能避免竞争条件，内核将退出的进程信息加入队列，这样一来父进程就总能依照子进程退出顺序接收到信息

### 实践领域
关注子进程的理念是一般的Unix编程模式的核心。这种模式有事被称为看顾进程，master/worker或者preforking  
此模式的核心概念：你有一个衍生出多个并发子进程的进程，这个进程看管着这些子进程，确保他们能够保持响应，并对子进程的退出做出回应   
例如，Web服务器Unicorn就采用了这种模式。可以告诉服务器自己希望启动多少个工作进程，父进程同每个子进程保持联系，并保证所有的子进程能够保持响应   
这种模式兼顾并发性和可靠性  

### 系统调用  
Ruby的Process.wait及其表亲都对应于waitpid(2)

## 僵尸进程
即发即弃的模式得确保清除子进程，以免变成僵尸  
内核会一直保留已退出的子进程的状态信息，直到父进程使用`Process.wait`请求这些信息。因此创建即发即弃子进程，却不去读取状态信息，便是在浪费内核资源  
如果不使用`Process.wait`等待某个子进程的退出，那么就得分离这个子进程  
```ruby
message = 'Good Moring'
recipient = 'tree@mybackyard.com'

pid = fork do
  # 父进程衍生出一个子进程来负责将数据发送给统计接收器，同时，父进程继续自己的工作
  StatusCollector.record message, recipient
end

# 这个代码确保子进程不会变成僵尸
Process.detach(pid)
```
`Process.detach`会生成一个新线程，这个线程的唯一工作是等待由pid指定的那个子进程退出，这确保了内核不会一直保留那些我们不需要的状态信息  

### 僵尸长什么样子
```ruby
# 创建一个子进程，1秒后退出
pid = fork { sleep 1 }
# 打印出改进程的pid
puts pid
# 父进程长眠，以便于检查子进程的状态信息
sleep
```
ps pid
```shell
10582 s000  Z+     0:00.00 (ruby)
```
状态为'z'或'Z+'就表示这是一个僵尸进程  

### 实践领域
任何已经结束的进程，如果它的状态信息一直未被读取，那么它就是一个僵尸进程  
所以任何子进程如果在结束之时，其父进程人在运行，那这个子进程就会成为僵尸  
即发即弃的方式极其少见，如果需要后台执行工作，更为常见的做法是采用一个专门的后台排队系统(sidekiq?)

## 进程皆可获得信号
`Process.wait`是一个很好的父进程监管子进程的方式，但这是一个阻塞调用  
所以便有了其他解决方案：Unix信号  

### 捕获SIGCHLD
```ruby
child_processes = 3
dead_processes = 0

child_processes.times do
  fork do
    sleep 3
  end
end

# 通过捕获:CHLD信号，内核会体型父进程他的子进程合适退出
trap(:CHLD) do
  # 由于Process.wait将它获得的数据都加入了队列，因此可以在此进行查询，因为我们知道其中的一个进程已经退出
  puts Process.wait
  dead_processes += 1
  # 一旦所有的子进程都统计完毕，直接退出
  exit if dead_processes == child_processes
end

# 父进程需要执行的任务
loop do
  (Math.sqrt(rand(44)) ** 8).floor
  sleep 1
end
```

### SIGCHLD与并发
_信号投递是不可靠的，如果代码在处理CHLD信号，这时候另一个子进程结束了，那么未必能收到第二个CHLD_  
要正确处理CHLD，必须在一个循环中调用`Process.wait`，查找所有已经结束的子进程  
但是`Process.wait`是一个阻塞，如果只有一个已结束的子进程，而调用了两次`Process.wait`，如何避免阻塞整个进程？  
常量`Process::WNOHANG`描述了这个标志的值  
```ruby
Process.wait(-1, Process::WNOHANG)
```
重写
```ruby
child_processes = 3
dead_processes = 0

child_processes.times do
  fork do
    sleep 3
  end
end

# 设置$stdout的sync，使得在CHLD信号处理程序中不会对#puts调用进行缓冲
# 如果信号处理程序在调用#puts之后被中断，则会引发一个ThreadError
# 如果你的信号处理程序需要执行IO操作，那么这不失为一个好方法
$stdout.sync = true
# 不知道是干嘛的

# 通过捕获:CHLD信号，内核会体型父进程它的子进程合适退出
trap(:CHLD) do
  # 由于Process.wait将它获得的数据都加入了队列，英雌可以在此进行查询
  # 因为我们知道其中一个子进程已经退出了

  # 执行一个非阻塞的Process.wait以确保统计每一个结束的子进程
  begin
    while pid = Process.wait(-1, Process::WNOHANG)
      puts pid
      dead_process += 1
      # 一旦所有的子进程统计完毕退出
      exit if dead_processes == child_processes
    end
  rescue Errno::ECHILD
  end
end

loop do
  (Math.sqrt(rand(44)) ** 8).floor
  sleep 1
end
```
