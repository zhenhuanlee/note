# 理解Unix进程(Working with Unix Processes)

## 基础知识

### 系统调用
Unix系统的组成，具体来说就是用户空间(userland)和内核  
系统调用：内核和用户空间搭建的桥梁  
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
> Linux分自定义变量(set)，环境变量(env)，自定义变量包含环境变量，export是一种命令工具
是显示那些通过export命令把自定义变量中包含的变量导入给环境变量  

最根本的是修改 .bash_profile    .bashrc    .bash_logout  
- ~/.bash_profile ---- 用户登录时被读取，其中包含的命令被执行  
- ~/.bashrc ---------- 启动新的shell时被读取，执行  
- ~/.bash_logout ----- shell登录退出时读取  

shell(此处指bash)的初始化过程是这样的：  
1. bash检查文件/etc/profile是否存在，存在就读取，否则跳过  
2. bash检查主目录下的文件.bash_profile是否存在，存在就读取，否则跳过  
3. bash检查主目录下的.bash_login是否存在，有就读取，否则跳过  
4. bash检查主目录下的.profile是否存在，有就读取，否则跳过  

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
环境变量经常作为一种将输入传递到命令行程序中的通用方法，所有的终端(Unix或Windows)均已支持环境变量  
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
3. 有两种运作在进程本身层面上的机制可以用来互通信息，进程名称、退出码  

### 进程名称
系统中每一个进程都有名称，如一个irb会话，对应的进程就获得了"irb"的名称。进程名的妙处在于它可以在运行期间被修改并作为一种通信手段  
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
衍生(forking)(Unix编程中最强大的概念之一)。fork(2)系统调用允许运行中的进程以编程的形式创建新的进程  
这个新的进程和原始的进程一模一样  
进行衍生时，调用fork(2)的进程被称为"父进程"，新创建的进程被称为"子进程"  
_子进程从父进程处继承了其所占用内存中的所有内容，以及所有属于父进程的已打开的文件描述符，这样两个进程就可以共享打开的文件，套接字等等_  
子进程对于想要在内存中载入多个应用程序来说简直是完美方案。  
子进程可以随意改写其内存内容的副本，而不会对父进程造成任何影响。  
```ruby
puts "Parent process pid is #{Process.pid}"
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
这样就意味着~更高~，更快，更省  
但是MRI或Rubinius并不支持  
想要CoW正常运作，程序需要以一种CoW友好的方式来编写。也就是说，它们得用一种使CoW成为可能的方式来管理内存。MRI和Rubinius并不是采用这种方式来写的  
> 为何不可？  
> MRI的垃圾收集器使用了一种"标记-清除"(mark-and-sweep)的算法。这意味着当垃圾收集器被调用时，它必须对每个已知的对象进行迭代并写入信息，指出改对象是否应该被回收。关键在于垃圾收集器每运行一次，内存中的搜游对象都会被写入信息。  
> 因此，在进行衍生之后，每次进行垃圾手机的时候，写时赋值所带来的好处会被撤销  

这就是创建Ruby企业版(Ruby Enterprise Edition)的主要原因之一，REE是CoW友好的  

### MRI/RBX用户
~对于其他Ruby虚拟机来说，fork享受不到CoW的好处，子进程需要获取调用进程所战友的内存内容的完整副本~  
Ruby2.0已提供CoW友好的垃圾收集器  

## 进程可待
前面所说的fork都是让父进程与子进程一道运行。有时候会导致一些奇怪的结果，比如当父进程先于子进程退出时  
这种情景只适用于一种用例：即发即弃(fire and forget)-如你希望让子进程异步的处理其他事务  
```ruby
message = 'Good Morning'
recipient = 'tree@mybackyard.com'

fork do
  # 在这个假象的例子中，父进程衍生出一个子进程来负责将数据发送给统计接收器
  # 父进程同时继续进行自己实际的数据发送工作

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
`Process.wait`会一直保持堵塞，直到任意一个子进程退出为止。如果父进程拥有不止一个进程，并且使用了`Process.wait`，那么就需要知道究竟是哪个子进程退出了。这时可以用返回值来解决这个问题  
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

# 通过捕获:CHLD信号，内核会体型父进程它的子进程合适退出
trap(:CHLD) do
  # 由于Process.wait将它获得的数据都加入了队列，因此可以在此进行查询
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

### 信号入门
信号是一种异步通信。当进程从内核那里接收到信号，它可以执行下列的某一操作：  
1. 忽略该信号  
2. 执行特定的操作  
3. 执行默认的操作  

### 信号来自何方
信号由一个进程发送到另一个进程，借用内核作为中介   
1. 第一个ruby会话中：
```ruby
puts Process.pid
sleep
```
2. 第二个ruby会话中：
```ruby
Process.kill(:INT, pid)
```
> 第二个进程会向第一个进程发送一个INT信号，使其退出  
> INT是INTERRUPT(中断)的缩写

此处学到一个技巧：
```
# bash中相继输入
ruby
puts 123
# 按下
ENTER
CTRL + SHIFT + D
```

### 信号一览
Term    进程立即结束
Core    进程会立即结束，并进行核心转储(栈跟踪)
Ign     表示进程会忽略该信号
Stop    表示进程会停止运行(暂停)
Cont    表示进程会回府运行(继续)
> 还有很多，pdf P61

### 重定义信号
...

### 忽略信号
...

### 信号处理是全局性
捕获一个信号有点像使用一个全局变量

### 恰当的冲定义信号处理程序
最佳角度来说，代码不应该定义任何信号处理程序，除非它是服务器  

### 何时接收不到信号？
进程可以在任何时候接收到信号。


## 进程皆可互通
这属于进程间通信(IPC)研究领域的一部分。有很多方法可以实现IPC，最常用的两种是：管道和套接字对(socket pairs)

### 第一个管道
管道是一个单向数据流。只能从一个进程传给另一个进程  
如何创建一个管道：
```ruby
reader, writer = IO.pipe
=> [#<IO:fd 9>, #<IO:fd 10>]
```
IO.pipe 返回一个包含两个IO对象的数组  (Ruby的IO类是File、TCPSocket、UDPSocket的超类)
基本上可以将IO对象当做File来对待(#read、#write、#close、etc..)   
```ruby
reader, writer = IO.pipe
writer.write("Into the pipe I go...")
writer.close   # 立刻就关闭了
puts reader.read
```
> reader调用read时，它会不停的试图从管道中读取数据，直到EOF标志

### 管道是单向的
...

### 共享管道
管道也是资源，它有自己的文件描述符以及其他的一切，因此也可以被共享  
```ruby
reader, writer = IO.pipe

fork do
  reader.close

  10.times do
    writer.puts "Another one bites the dust"
  end
end

writer.close
while message = reader.gets
  $stdout.puts message
end
```
> 关闭了管道未使用的一端，以免干扰正在发送的EOF  
> 现在涉及到两个进程，在考虑EOF时就要多考虑一层，因为文件描述符会被复制，所以就出现了4个文件描述符，其中只有两个会被用于通信，其他两个必须关闭

### 流与消息
上面的例子中，使用puts和get是应为它们使用终止符作为分隔符，为什么需要终止符呢？是因为，"流"没有开始和结束的概念，当使用管道或TCP套接字这样的IO流时，将数据写入流中，之后跟着一些特定协议的分隔符，随后从IO流中读取数据时，一次读取一块，遇到分隔符就停止读取  
当然，也可以用消息来代替流进行通信，虽然不能在管道中使用消息，但是_Unix套接字_可以  
> Unix套接字：一种只能用于同一台物理主机中进行通信的套接字，比TCP套接字快得多，非常适合IPC

```ruby
require 'socket'
Socket.pair(:UNIX, :DGRAM, 0)
# => [#<Socket:fd 9>, #<Socket:fd 10>]
```
上面的代码创建了一对已经互相连接好的UNIX套接字，它们使用数据报(datagram)而不是流进行通信，在这种模式中，可以向其中一个套接字写入整个信息，然后从另一个套接字中读取整个信息，不用分隔符  
```ruby
require 'socket'

child_socket, parent_socket = Socket.pair(:UNIX, :DGRAM, 0)
maxlen = 1000

fork do
  parent_socket.close

  4.times do
    instruction = child_socket.recv(maxlen)
    child_socket.send("#{instruction} accomplished!", 0)
  end
end
child_socket.close

2.times do
  parent_socket.send("Heavy lifting", 0)
end
2.times do
  parent_socket.send("Feather lifting", 0)
end

4.times do
  $stdout.puts parent_socket.recv(maxlen)
end

# 输出
=begin
  Heavy lifting accomplished!
  Heavy lifting accomplished!
  Feather lifting accomplished!
  Feather lifting accomplished!
=end
```
> 管道提供的是单向通信，套接字对提供的是双向通信  
> 我在主block和fork block中分别输出了parent_socket的object_id 是一样的

### 实践领域
Ruby的IO.pipe对应于pipe(2)，Socket.pair对应于socketpair(2)  
Socket.recv对用于recv(2)，Socket.send对用于send(2)  


## 守护进程
系统中有一个init进程，他的ppid是0，作为所有进程的祖父，它是首个进程，没有祖先。pid是1

### 创建第一个守护进程
rack项目的rackup命令包含一个选项，可以将服务器变成守护进程，并置于后台运行

### 深入Rack
```ruby
def daemonize_app
  if RUBY_VERSION < "1.9"
    exit if fork
    Process.setsid
    exit if fork
    Dir.chdir "/"
    STDIN.reopen "/dev/null"
    STDOUT.reopen "/dev/null", "a"
    STDERR.reopen "/dev/null", "a"
  else
    Process.daemon
  end
end
```
else代码快中，`Process.daemon`将当前的进程编程守护进程  
这个方法所做的工作和if块中的是一样的  

### 逐步将进程变成守护进程
```ruby
exit if fork
```
这一行代码灵活运行了`fork`方法的返回值，他在父进程中返回子进程的pid，在子进程中返回nil  
所以这个在父进程中为真，子进程中为假，这就意味着，父进程将会退出，子进程成为孤儿继续运行    
> 孤儿进程的`Process.ppid`返回1   

这一步是必须的，这可以让调用次脚本的终端认为该命令已执行完毕，浴室将控制返回到终端，不再考虑那条命令了   
```ruby
Process.setsid
```
调用`Process.setsid`完成下面三件事
1. 该进程变成一个新会话的会话领导    
2. 该进程变成一个新进程组的组长  
3. 该进程没有控制终端  
下面具体说说这三个

### 进程组和会话组
进程组和会话组都和作业控制有关。用"作用控制"来引指终端处理进程的方法  

#### 进程组
每一个进程都属于某个进程组，每一个组都有唯一的整数id。进程组是一个相关进程的集合，通常是父进程与其子进程
```ruby
puts Process.getpgrp
puts Process.pid
```
在一个irb会话中，这两个值是一样的，通常情况下，进程组id和进程组组长的pid相同。进程组组长是终端命令的"发起"进程  

```ruby
puts Process.pid
puts Process.getpgrp

fork do
  puts Process.pid
  puts Process.getpgrp
end
```
可以看到，子进程的组id是从父进程中继承来的  
那么问题来了，当父进程退出后，子进程变成了孤儿进程，继续运行。
但是如果父进程由终端控制，并被信号终止的话，会发成什么？  
答案是，子进程也被终止！这是因为，父进程和子进程都是同一个组的成员，因此会被同一个信号终结  

#### 会话组
更高一级的抽象，它是进程组的集合
```shell
git log | grep shipped | less
```
在这个例子中，每个命令都有自己的进程组，这是因为每个命令都可能创建子进程，但是这些进程并不属于其他命令。尽管这些命令不属于同一个进程组，但是CTRL-C仍然可以将其全部终止  
这些命令都是同一个会话组的成员，在shell中的每一次调用都会获得自己的会话组。一次调用可以是单个命令，也可以是管道连接的一串命令  


接下来继续Rack代码：  
第一行代码衍生出一个子进程，然后父进程退出。启动该进程的终端觉察到进程退出后，将控制权返回给用户，但是之前衍生出的子进程任然拥有从父进程中继承而来的组id和会话id。此时，这个衍生进程既非会话领导，也非组长  
因此，仍然可能被发送到进程组的信号干预   
`Process.setsid`会使得衍生进程成为一个新进程组和新会话组的组长兼领导，注意，只能在子进程中调用  
新的会话组并没有控制终端，但是可以分配一个
```ruby
exit if fork
```
已成为进程组和会话组组长的衍生进程再次进行衍生，然后退出  
显然，这个新衍生出的进程不再是进程租的组长，也不是会话领导。由于之前的会话领导没有控制终端，并且此进程也不是会话领导，因此这个进程绝不会有控制终端，终端只能够分配给会话领导，如此就可以保证进程现在完全脱离控制终端，并且可以独自运行  
```ruby
Dir.chdir "/"
```
将当前的工作目录更改为系统的根目录，为了确保守护进程的当前目录不会突然消失(非必须)  
```ruby
STDIN.reopen "/dev/null"
STDOUT.reopen "/dev/null", "a"
STDERR.reopen "/dev/null", "a"
```
这将所有的标准流设置到/dev/null，也就是将其忽略，因为守护进程不再依附于某个终端会话，也就不需要什么标准流了，但是也不能关闭，因为一些程序还指望它们随时可用  

### 实践领域
如果想要创建一个守护进程，那之前应该先考虑一个问题：这个进程需要一直保持响应么？是不是可以考虑定时任务或者后台作业系统  


## 生成终端进程
### fork + exec
下面描述的所有方法都是fork(2) + exec(2)的变体  
exec：允许你使用另一个进程来替换当前进程，换言之，可以将当前进程转变成另一个进程。如，可以先使用一个Ruby进程，然后把它变成Python进程、ls(1)进程或另一个Ruby进程  
```
exec 'ls', '--help'
```
exec的转变是不可逆的，所以fork + exec就非常美滋滋了  
fork创建一个新进程，然后exec把这个进程变成其他的  
```ruby
hosts = File.open('/etc/hosts')

exec 'python', '-c', "import os; print os.fdopen(#{hosts.fileno}).read()"
```

### exec的参数
将字符串传给exec，它实际上会启动一个shell进程，然后再将这个字符串交给shell解释  
传递一个数组的话，他会跳过shell，直接将此数组作为新进程的ARGV  
_通常避免传递字符串_

#### Kernel#system
```ruby
system('ls')
system('ls', '--help')
system('git log | tail -10')
# Kernel#system的返回值用最基本的方式反映了终端命令的退出码，如果终端命令的退出码是0，则true，否则false
```

#### Kernel#`
```ruby
`ls`
`ls --help`
%x[git log | tail -10]
# Kernel#` 略有不同，它的返回值是由终端程序的STDOUT汇集而成的一个字符串
# Kernel#system 和 %x[]做的事完全一样
```
它使用fork(2)来实现，对于STDERR不做任何特殊处理

#### Process.spawn
```ruby
# 此调用会启动rails server进程并将环境变量RAILS_ENV设置为test
Process.spawn({'RAILS_ENV' => 'test'}, 'rails server')

# 此调用在执行ls --help阶段将STDERR与STDOUT进行合并
Process.spawn('ls', '--help', STDERR => STDOUT)
```
`Process.spawn`是非阻塞的  
```ruby
# 以阻塞方式执行
system 'sleep 5'

# 非阻塞方式
Process.spawn 'sleep 5'

# 使用Process.spawn以阻塞方式执行
# 返回子进程的pid
pid = Process.spawn 'sleep 5'
Process.waitpid(pid)
```

#### IO.popen
```ruby
# 这个例子会返回一个文件描述符(IO 对象)
# 对其进行读取会返回该shell命令打印到STDOUT中的内容
IO.popen('ls')
```
IO.popen最常见的用法是用纯Ruby来实现Unix管道(pipes)，这即popen中'p'的由来  
在底层`IO.popen`人就做的是fork+exec的事，但它还设置了一个通道用于通生成的进程进行通信，这个管道作为块参数(block argument)被传递到`IO.popen`代码块中  
```ruby
# 一个IO对象被传递到代码块中，在本例中打开stream进行写入  
# 因此将stream设为生成进程的STDIN
#
# 如果打开stream进行读取(默认操作)，那么将stream设置为生成进程的STDOUT
IO.popen('less', 'w') { |stream|
  stream.puts "some\ndata"
}
# 必须选择访问哪个流
```

#### Open3
Open3允许同时访问一个生成进程的STDIN、STDOUT、STDERR
```ruby
# Open3是标准库的一员
require 'open3'

Open3.popen3('grep', 'data') { |stdin, stdout, stderr|
  stdin.puts "some\ndata"
  stdin.close
  puts stdout.read
}

# 在可行的情况下，Open3会使用Process.spawn
# 可以像这样将选项传递给Process.spawn
Open3.popen3('ls', '-nhh', :err => :out) { |stdin, stdout, stderr|
  puts stdout.read
}
```
从用法来说，`Open3`想是一个更灵活的IO.popen  


## 尾声(°̥̥̥̥̥̥̥̥ω°̥̥̥)
