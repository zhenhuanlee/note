# TCP Socket编程
## 建立套接字
### Ruby的套接字库
```ruby
require 'socket'
# socket库是Ruby标准库的组成部分。同opensll、zlib和curses这些类似，socket库与其所依赖的
# C语言库之间是thin binding关系
```

### 创建首个套接字库
```ruby
require 'socket'
socket = Socket.new(Socket::AF_INET, Socket::SOCK_STREAM)
```
代码在INET域创建了一个类型为`STREAM`的套接字  
> INET是internet的缩写，特别用于指代IPv4版本的套接字   
> STREAM表示将使用数据流进行通信，该功能由TCP提供    
> DGRAM(datagram)，则表示UDP套接字  

### 什么是端点  
套接字使用IP地址将消息指向特定的主机  

### 环回地址  
IP地址未必总是指向远端地址，有时候是连接自己本地主机上的套接字  
所以就有了环回地址(loopback interface)，和网卡接口不同，这是一个和硬件无关，完全虚拟的接口。  
发送到环回接口上的数据立即会再同一个接口上被接受。  
环回接口对应的主机名是localhost，对应的IP地址通常是127.0.0.1(定义在hosts文件中)

### IPv6
...

### 端口
对于端点而言，还有一个重要的方面-端口号。  
对于每个套接字而言，IP地址和端口号的组合必须是唯一的，所以在同一个侦听端口上可以有两个套接字，一个使用IPv4地址，另一个IPv6地址  

### 创建第二个套接字
第一个有点烦，来点语法糖  
```ruby
require 'socket'

socket = Socket.new(:INET6, :STREAM)
# :INET6 这个symbol描述了Socket::AF_INET
# :STREAM 这symbol描述了Socket::SOCK_STREAM
```

### 文档
1. Unix手册页 `man 2 scoket`  
2. ri-Ruby命令行工具  `ri Socket.new`  


## 建立连接
...

## 服务器的生命周期
1. 创建  
2. 绑定  
3. 侦听  
4. 接受  
5. 关闭  

### 服务器的绑定
```ruby
require 'socket'
socket = Socket.new(:INET, :STREAM)

# 创建一个C结构体来保存用于侦听的地址
addr = Socket.pack_sockaddr_in(4481, '0.0.0.0')

# 执行绑定
socket.bind(addr)
```
这是一个低层次实现，演示了如何将TCP套接字绑定到本地端口上，客户端套接字可以使用该端口号链接服务器套接字  

### 应该绑定到哪个端口
1. 不要使用0-1024之间的端口，这些端口是保留给系统使用的  
2. 不要使用49000-65535之间的端口，这些都是临时端口  
3. 1025-48999之间的端口的使用是一视同仁的，决定使用时可以查一下IANA注册端口列表

### 该绑定到哪个地址
1. 127.0.0.1 只监听环回接口，只有到localhost和127.0.0.1的连接才会呗服务器套接字接受  
2. IP地址     只监听此接口，任何寻址到这个接口的客户端都会被监听  
3. 如果希望监听每一个接口，那么可以使用 0.0.0.0  

### 服务器监听
创建套接字并绑定到特定端口之后，需要告诉套接字对接入的连接进行侦听  
```ruby
require 'socket'

# 创建套接字，并绑定到端口4481
socket = Socket.new(:INET, :STREAM)
addr = Socket.pack_sockaddr_in(4481, '0.0.0.0')
socket.bind(addr)

# 告诉套接字侦听接入的连接
socket.listen(5)
```
这个代码还是会立刻退出，在服务器套接字能够处理连接之前，还需要另一个步骤。
接下来讲listen：  

#### 侦听队列
`listen(5)` 表示服务器套接字能够容纳5个待处理(pending)的最大连接数。  
待处理的连接列表被称作*侦听队列*   
如果服务器正忙于处理某个客户端连接，如果这是其他新的客户端连接到达，将会被置于侦听队列  

#### 侦听队列的长度
这个长度为什么不设为无穷大呢？  
系统有限制，`Socket::SOMAXCONN`可以获知当前所允许的最大的侦听队列长度  
一般来说`server.listen(Socket::SOMAXCONN)`

### 接受连接
下面的代码演示了如何创建监听套接字：
```ruby
require 'socket'

# 创建服务器套接字
server = Socket.new(:INET, :STREAM)
addr = Socket.pack_sockaddr_in(4481, '0.0.0.0')
server.bind(addr)
server.listen(128)

# 接受连接
connection, _ = server.accept

#####
connection.local_address
connection.remoter_address
```
`accept`会一直堵塞到有连接到达  

用netcat发起一个连接：
```shell
echo ohai | nc localhost 4481
```
程序顺利退出

#### 以阻塞方式接受连接
`accpet`调用是阻塞式的，它将还未处理的连接从队列中弹出(pop)而已。如果队列为空，那么它就一直等，直到有连接被加入到队列为止

#### accept调用返回一个数组
第一个元素是建立好的连接，第二个元素是一个`Addrinfo`对象，该对象描述了客户端连接的远程地址  
> Addrinfo  
> Addrinfo 是一个Ruby类，描述了一台主机及其端口号。它将端点信息进行了包装。  
> 可以使用`Addrinfo.tcp('localhost', 4481)`构建这些信息   
> 一些有用的信息包括 `#ip_address` 和 `#ip_port`  

#### 连接类
尽管`accept`返回了一个"连接"，但是这段代码告诉我们并没有特殊的连接类(connection class)。一个连接实际上就是Socket的一个实例

#### 文件描述符
`accept`返回一个`Socket`实例，但是这个连接的文件描述符编号和服务器套接字不一样。这表明`accept`返回了一个不同于服务器套接字的全新`Socket`，这个很重要，每个连接都有一个全新的`Socket`对象描述，这样服务器套接字可以保持不变，不停的接受新的连接  
> 文件描述符编号是内核用于跟踪当前进程所打开文件的一种方法(套接字是文件)  

#### 连接地址
每一个TCP连接都由`local_address`(本地主机的端点)，`remote_address`(另一端端点)组成

#### accpet循环
`accept`会返回一个连接，前面的代码，服务器在接受了一个连接后退出，这个是不行的：
```ruby
require 'socket'

server = Socket.new(:INET, :STREAM)
addr = Socket.pack_sockaddr_in(4481, '0.0.0.0')
server.bind(addr)
server.listen(128)

# 进入无线循环
loop do
  connection, _ = server.accept
  # 处理连接
  connection.close
end
```
Ruby编写某类服务器的一种常用方法，后面有语法糖

### 关闭服务器
调用`close`

#### 退出时关闭
为什么要手动关闭？   
1. 资源使用。如果你使用的套接字没有关闭，那么那些你不再使用的套接字引用很可能保留着，没有被系统自动删除  
2. 打开文件的数量限制。所有的进程都只能够打开一定数量的文件。  

#### 不同的关闭方式
套接字允许双向通信，实际上可以只关闭其中的一个通道  
```ruby
require 'socket'

server = Socket.new(:INET, :STREAM)
addr = Socket.pack_sockaddr_in(4481, '0.0.0.0')
server.bind(addr)
server.listen(128)
connection, _ = server.accept

# 该连接随后也许不再需要写入数据，但是可能仍然需要进行读取
connection.close_write

# 该连接不再需要进行任何数据读写操作
connection.close_read
```
关闭写操作流(write stream)会发送一个EOF到套接字的另一端  
`close_write`和`close_read`方法在底层都利用了shutdown(2)。同close(2)明显不同的是：即使是存在着连接的副本，shutdwon(2)也可以完全关闭该连接的某一部分   
> 连接副本
> 可以使用Socket#dup创建文件描述符的副本  
> 获得一个文件描述符副本的更常见的方法是利用`Process.fork`方法。  

`close`会关闭调用它的套接字实例，但是副本不会被关闭，其他副本仍然可以交换数据  
`shutdown`会完全关闭在当前套接字及其副本上的通信，但是它不会回收套接字所使用过的资源，每个套接字实例仍必须使用`close`结束它的生命周期  
```ruby
require 'socket'

server = Socket.new(:INET, :STREAM)
addr = Socket.pack_sockaddr_in(4481, '0.0.0.0')
server.bind(addr)
server.listen(128)
connection, _ = server.accept

# 创建副本
copy = connection.dup

# 关闭所有连接副本上的通信
connection.shutdown

# 关闭原始连接，副本会再垃圾收集器进行收集时关闭
connection.close
```

### Ruby包装器
#### 服务器创建
`TCPServer`类，它将进程中"服务器创建"进行了非常简洁的抽象
```ruby
require 'socket'

server = TCPServer.new(4481)
# #<TCPServer:fd 10>

```
等同于
```ruby
require 'socket'

server = Socket.new(:INET, :STREAM)
addr = Socket.pack_sockaddr_in(4481, '0.0.0.0')
server.bind(addr)
server.listen(5)
```
> 创建一个`TCPServer`实例返回的实际上并不是`Socket`实例，而是`TCPServer`实例，两者的接口几乎一样，但是还是有些重要的差异  
> 最明显的就是`TCPServer#accept`只返回连接，而不是返回`remote_address`
> 监听队列长度默认5，如果要设置，调用`TCPServer#listen`

这个Ruby包装器会返回两个TCP套接字，一个可以通过IPv4连接，另一个可以通过IPv6连接，两者都在同一个端口上进行监听  
```ruby
require 'socket'

servers = Socket.tcp_server_sockets(4481)
# [#<Socket:fd 11>, #<Socket:fd 12>]
```

#### 连接处理
除了创建服务器，Ruby也为连接处理提供了抽象  
比如说那个`loop`
```ruby
require 'socket'

server = TCPServer.new(4481)

Socket.accept_loop(server) do |connection|
  connection.close
end
```
连接并不会在每个代码块结尾处自动关闭，传递给代码块的参数和`accept`调用的返回值一模一样  
`Socket.accept_loop`的另一个好处：可以向它传递多个监听的套接字，它可以接收在这些套接字上的全部连接(还记得`Socket.tcp_server_sockets`么)
```ruby
require 'socket'

servers = Socket.tcp_server_sockets(4481)
Socket.accept_loop(servers) do |connection|
  connection.close
end
```

#### 合而为一
更简洁的来了
```ruby
require 'socket'

Socket.tcp_server_loop(4481) do |connection|
  connection.close
end
```
不能简洁更多，它只是`Socket.tcp_server_sockets`和`Socket.accept_loop`的一个包装器  


## 客户端的生命周期
1. 创建  
2. 绑定  
3. 连接  
4. 关闭  

第一个阶段服务器和客户端都是一样的  

### 客户端绑定
客户端和服务器套接字一样，都是以`bind`作为起始，很少有服务器不去调用`bind`，也很少会有客户端去调用`bind`  
如果客户端套接字(或服务器套接字)不调用`bind`，那么它会从临时接口范围内获得一个随机端口号  
> 为什么不用`bind`  
> 因为客户端不需要通过某个指定端口访问，没有人需要知道他们的端口号

### 客户端连接
客户端和服务器真正的区别就在于`connect`调用，该调用发起到远程套接字的连接  
```ruby
require 'ruby'

socket = Socket.new(:INET, :STREAM)

# 发起到google.com端口80的连接
remote_addr = Socket.pack_sockaddr_in(80, 'google.com')
socket.connect(remote_addr)
```

#### 连接故障
在客户端的生命周期中，很可能在服务器还没准备好的时候客户端就发起了连接，同样也很有可能连接了一个并不存在的服务器，这两种情况很相似，因为TCP所具备的容错性，它会尽最大可能等待远程主机的回应
```ruby
# 尝试连接一个不可用的端点
require 'socket'

socket = Socket.new(:INET, :STREAM)

# 尝试在gopher port上连接google.com
remote_addr = Socket.pack_sockaddr_in(70, 'google.com')
socket.connect(remote_addr)
```
很长时间才能从connect调用返回。connect调用默认有一段较长时间的超时  

### Ruby包装器
#### 客户端创建
```ruby
require 'socket'

socket = Socket.new(:INET, :STREAM)

# 发起到google.com端口80的连接
remote_addr = Socket.pack_sockaddr_in(80, 'google.com')
socket.connect(remote_addr)
```
语法糖一下
```ruby
require 'socket'

socket = TCPSocket.new('google.com', 80)
```
还有一个使用`Socket.tcp`的类似的客户端构建方法
```ruby
require 'socket'

Socket.tcp('google.com', 80) do |connection|
  connection.write("GET /HTTP/1.1\r\n")
  connection.close
end

# 如果省略代码参数，则行为方式同TCPSocket.new()一样
```

## 交换数据
### 流
TCP是一个基于流的协议(:STREAM)  
从协议上而言，TCP在网络上发送的是分组   
从程序代码上来说，TCP连接提供了一个不间断的、有序的通信流  
> 流并没有消息边界的概念，即便是客户端分别发送了3分数据，服务器在读取的时候，也是将其作为一份数据来接受，它并不知道客户端那是分批发送的数据，但是流的内容的次序还是会被保留的  

### 套接字操作
#### 简单的读操作
```ruby
require 'socket'

Socket.tcp_server_loop(4481) do |connection|
  # 从连接中读取数据最简单的方法
  puts connection.read
  # 关闭连接，让客户端知道不用再等待数据返回
  connection.close
end
```
> Ruby的各种套接字类以及File的IO都有一个共同的父类。Ruby中所有的IO对象(套接字，管道，文件...)都有一套通用的接口，支持`read`,`write`,`flush`等方法  

#### 没那么简单
读很容易出错
```shell
tail -f /var/system.log | nc localhost 4481
```
服务器永远不会停止读取数据，因为`tail -f` 不会停止发送数据，所以`netcat`的管道一直出于打开状态  

#### 不成熟的解决方案
##### 读取长度
指定最小的读取长度
```ruby
require 'socket'
one_kb = 1024 # 字节

Socket.tcp_server_loo
  # 以1KB为单位进行读取
  while data = connection.read(one_kb) do
    puts data
  end

  connection.close
end
```

#### 堵塞的本质
`read`调用会一直堵塞，知道获取了完成长度(full length)的数据为止，上面的例子是1KB    
但是如果客户端只发送了500B，那么就会造成死锁，解决方案：  
1. 客户端发送完500B后发送一个EOF  
2. 服务器采用部分读取(partial read)的方式  

#### EOF事件
当连接上调用read并接收到EOF事件时，就可以确定不会再有数据，可以停止读取了  
EOF并不是一个字符，EOF更像是一个状态事件(state event)  
如果一个套接字没有数据可写，它可以使用`shutdown`或`close`来表明自己不再需要写入任何数据。这就会导致一个EOF事件被发送给另一端  
```ruby
require 'socket'
one_kb = 1023

Socket.tcp_server_loop(4481) do |connection|
  while data = connection.read(one_kb) do
    puts data
  end

  connection.close
end
```
客户端
```ruby
require 'socket'

client = TCPSocket.new('localhost', 4481)
client.write('gekko')
client.close
# 客户端发送EOF最简单的方式就是关闭自己的套接字
```

#### 部分读取
`readpartial`不会堵塞，而是立即返回可用的数据，调用`readpartial`时，必须传递一个整数作为参数，来指定最大长度。  
`readpartial`最多读取到指定长度。如果指明1KB数据，但是客户端只发送了500B，并不会堵塞，它会立即将已读取到的数据返回  
服务器：
```ruby
require 'socket'
one_hundred_kb = 1024 * 100

Socket.tcp_server_loop(4481) do |connection|
  begin
    # 每次去读100KB或更少
    while data = connection.readpartial(one_hundred_kb) do
      puts data
    end
  rescue EOFError
  end

  connection.close
end
```
就EOF而言，`readpartial`的工作方式不同于`read`，当接到EOF时，`read`仅仅是返回，而`readpartial`会产生一个`EOFError`异常


## 套接字的写操作
只有一种方式可以向套接字写入数据`write`
```ruby
require 'socket'

Socket.tcp_server_lopp(4481) do |connection|
  connection.write('Welcome!')
  connection.close
end
```

## 缓冲
几个重要的问题：  
1. 在一次调用中，应该读/写多少数据  
2. 如果write成功返回，是否意味着连接的另一端已经接收到了数据  
3. 是否应该将一个大数据量的write分割成多个小数据量进行多次写入

### 写缓冲
TCP连接上，`write`写入时究竟发生了什么？  
`write`调用并返回时，就算没有引发异常，也不代表数据已经通过网络顺利发送并被客户端套接字接收到  
`write`返回时，它只是表明你已经将数据交给Ruby的IO系统和底层的操作系统内核  
在应用程序代码和实际的网络硬件之间至少还存在一个缓冲层  
TCP套接字默认将sync设为true。这就跳过了Ruby的内部缓冲，否则就又要多出一个缓冲层了  
> 为什么要缓冲区
> 提高性能。将缓慢的网络发送过程交由幕后操作，让`write`可以立即返回。幕后还可以将所有还未执行的写操作汇总到一起，在发送时进行分组及优化，在实现最佳性能的同事避免网络过载。在网络层面上，发送大量的小分组会引发可观的开销

### 该写入多少数据
由于缓冲区的存在，我们不需要考虑这个问题，但是如果是大文件，或者大数据，最好先将这些数据进行分割，避免全部载入到内存中  

### 读缓冲
读操作同样会被缓冲  
比如，`read`从TCP连接中读取数据并给它传递一个最大的读取长度，Ruby实际上可能会接受大于所指定长度的数据  
多出来的数据被存储在Ruby内部的缓冲区中，在下次调用`read`时，Ruby会先查看自己的内部缓冲区中有没有未读取的数据，然后通过系统内核请求更多的数据  

### 该读取多少数据
如果指定太大的话，会浪费内核分配的内存  
太小的话会导致系统的频繁调用  
参考大多数的项目，多采用了`readpartial(1024 * 16)`16KB作为读取长度  
> 记住 16KB

## 第一个客户端服务器
### 服务器
```ruby
require 'socket'

module CloudHash
  class Server
    def initialize(port)
      # 创建底层的服务器套接字
      @server  = TCPServer.new(port)
      puts "Listening on port #{@server.localhost_address.ip_port}"
      @storage = {}
    end

    def start
      # accept 循环
      Socket.accept_loop(@server) do |connection|
        handle(connection)
        connection.close
      end
    end

    def handle(connection)
      request = connection.read
      connection.write process(request)
    end

    # 所支持的命令：
    # SET key value
    # GET key
    def Process(request)
      command, key, value = request.split
      case command.upcase
      when 'GET'
        @storage[key]

      when 'SET'
        @storage[key] = value
      end
    end

  end
end

server = CloudHash::Server.new(4481)
server.start
```

### 客户端
```ruby
require 'socket'

module CloudHash
  class Client
    class << self
      attr_accessor :host, :port
    end

    def self.get(key)
      request "GET #{key}"
    end

    def self.set(key, value)
      request "SET #{key} #{value}"
    end

    def self.request(string)
      # 每一个新请求创建一个新连接
      @client = TCPSocket.new(host, port)
      @client.write(string)

      # 完成请求后发送EOF
      @client.close_write

      # 一直读取到EOF来获取响应信息
      @client.read
    end
  end
end

CloudHash::Client.host = 'localhost'
CloudHash::Client.port = 4481
```
使用
```ruby
puts CloudHash::Client.set 'prez', 'obama'
puts CloudHash::Client.get 'prez'
puts CloudHash::Client.get 'vp'
```

### 投入运行
```shell
ruby code/cloud_hash/server.rb
```

```shell
tail -4 code/cloud_hash/client.rb
puts CloudHash::Client.set 'prez', 'obama'
puts CloudHash::Client.get 'prez'
puts CloudHash::Client.get 'vp'

ruby code/cloud_hash/client.rb
```

## 套接字选项
套接字选项是一种配置特定系统下套接字行为的低层手法，因为涉及低层设置，所以并没有这方面的系统调用提供便捷的包装器  

### 套接字选项
#### SO_TYPE (指连接类型)
```ruby
require 'socket'

scoket = TCPSocket.new('google', 80)
# 获得一个描述套接字类型的Socket::Option实例
opt = socket.getsockopt(Socket::SOL_SOCKET, Socket::SO_TYPE)

# 将描述该选项的整数值通存储在Socket::SOCK_STREAM中的整数进行比较
opt.int == Socket::SOCK_STREAM # => true
opt.int == Socket::SOCK_DGRAM  # => false
```
`getsockopt`返回一个`Socket::Option`实例，在这个层面上进行操作时，所有一切都被转化成整数，因此`SocketOption#int`可以获得与返回值相关联的底层的整数值  
上例中，将int值通各种`Socket`类型常量进行比较，发现这是一个`STREAM`套接字  
上例也可以写成这样  
```ruby
require 'socket'

socket = TCPSocket.new('google.com', 80)
# 使用符号
opt = socket.getsockopt(:SOCKET, :TYPE)
```

#### SO_REUSE_ADDR ()
每个服务器都应该设置的一个常见选项  
`SO_REUSE_ADDR`选项告诉内核：如果服务器当前处于`TIME_WAIT`状态，即便另一个套接字要绑定(bind)到服务器目前所使用的本地地址也无妨
> TIME_WAIT状态  
> 当关闭了(close)了某个缓冲区，但其中仍有未处理数据套接字之时就会出现`TIME_WAIT`状态，前面说过，调用`write`值保证数据进入了缓冲区，当关闭一个套接字时，它未处理的数据并不会被丢弃，内核会保持足够长的打开时间，等待接收方的确认  
> 如果关闭一个尚有数据未处理的服务器并立即将同一个地址绑定到另一个套接字上(比如重启服务器)，则会引发一个`Errno::EADDRINUSE`。除非未处理的数据被丢弃，设置`SO_REUSE_ADDR`可以绕过这个问题，使的可以绑定到一个处于`TIME_WAIT`状态的套接字所使用的地址上  

```ruby
# 如何打开这个选项
require 'socket'

server = TCPServer.new('localhost', 4481)
server.setsockopt(:SOCKET, :REUSEADDR, true)

server.getsocketopt(:SOCKET, :REUSEADDR)  # => true

# TCPServer.new、Socket.tcp_server_loop以及类似的方法都默认打开了此选项
```
> `setsockopt(2)`可以查看系统上可用的套接字选项的完整列表

## 非阻塞式IO
### 非阻塞式读操作
`read`会一直保持堵塞  
`readpartial`可以暂时解决堵塞，`readpartial`会立即返回所有的可用数据，但如果没有数据可用，那么它人就会陷入拥塞  
`read_nonblock`不会堵塞，和`readpartial`类似，`read_nonblock`需要一个整数的参数，指定需要读取的最大字节数
```ruby
require 'socket'

Socket.tcp_server_loop(4481) do |connection|
  # 注意这里使用了一个循环，要不然，read_nonblock读完就过了
  loop do
    begin
      puts connection.read_nonblock(4096)
    rescue Errno::EAGAIN
      retry
    rescue EOFError
      break
    end
  end

  connection.close
end
```

```shell
tail -f /var/log/system.log | nc localhost 4481
```
即便没有发送数据，`read_nonblock`调用仍然会立即返回，事实上，它产生了一个Errno::EAGAIN异常
> EAGAIN  
> 文件被标记用于非堵塞式IO，无数据可读  

对被堵塞的读操作进行重试的正确做法是使用`IO.select`
```ruby
begin
  connection.read_nonblock(4096)
rescue Errno::EAGAIN
  IO.select([connection])
  # 一个堵塞方法，直到connection(或数组中的任一个)变得可读
  retry
end
```
使用套接字数组作为`IO.select`调用的第一个参数将会造成堵塞，直到其中的某个套接字变得可读。所以应该仅当套接字有数据可读时才调用`retry`  
很神奇的用非堵塞方法重新实现了堵塞式的`read`方法，当然这本身没有什么用处  
`IO.select`提供了一种灵活性，可以在进行其他工作的同时监控多个套接字或是定期检查他们的可读性   
根据自身使用情况：`read_nonblock`会一直抛出异常，直到获取到了一个返回   
> 什么时候读操作会堵塞  
> `read_nonblock`首先会检查Ruby的内部缓冲区时候还有未处理的数据，如果有，立即返回  
> 然后`read_nonblock`会询问内核时候有其他可用的数据可供`select`读取，如果有，不管这些数据是在内核缓冲区还是网络中，都会被立即返回。其他情况都会使`read`堵塞，并在`read_nonblock`中引发异常  

### 非堵塞式写操作
`write_nonblock`可能会返回部分写入的结果，`write`总是将数据全部写入
```ruby
require 'socket'

client = TCPSocket.new('locahost', 4481)
payload = 'xxx' * 10_000

written = client.write_nonblock(payload)
written < payload.size #=> true>
# 写入的数据小于负载的数据长度
```
当`write_nonblock`碰上了某种使它堵塞的情况，因此也就没法进行写入，所以返回了整数值，表示写入了多少数据  
`write_nonblock`的行为和系统调用`write(2)`一模一样，它尽可能多地写入数据并返回写入的数量。这和Ruby的`write`方法不同，后者可能会多次调用`write(2)`写入所有的请求数据  
这个时候可以调用`IO.select`，它可以显示某个套接字可写，这意味着可以进行无阻塞的进行写入
```ruby
require 'socket'

client = TCPSocket.new('localhost', 4481)
payload = 'xxx' * 10_000

begin
  loop do
    bytes = client.write_nonblock(payload)

    break if bytes >= payload.size
    payload.slice!(0, bytes)
    IO.select(nil, [client])
  end
rescue Errno::EAGAIN
  IO.select(nil, [client])
  retry
end
```
> 什么时候会写操作堵塞
> 1. TCP连接的接收端还没有确认接收到对方的数据，而发送方已经发送了所允许发送的数据量。  
_TCP使用拥塞控制算法确保网络不会被分组所淹没。如果数据花费了很长时间才到达TCP连接的接收端，那么要注意不要发送超出网络处理能力的数据，以免网络过载_   
> 2. TCP连接的接收端无力处理更多的数据   
_即便是另一端已经确认接收到了数据，它仍必须清空自己的数据窗口，以便重新填入其他数据。这设计内核的读缓冲区。如果接收端没有处理它接收的数据，那么堵塞控制算法会强制发送端阻塞，直到客户端可以接受更多的数据为止_  

### 非拥塞式接收
非堵塞式`read`和`write`用得最多，但别的方法也有非堵塞形式  
`accept_nonblock`和普通的`accept`几乎一样。  
`accept`只是从侦听队列中弹出一个连接，如果侦听队列为空，那`accept`就得堵塞，而`accept_nonblock`就不会堵塞，只是产生一个`Errno::EAGAIN`
```ruby
require 'socket'

server = TCPServer.new(4481)

loop do
  begin
    connection = server.accept_nonblock
  rescue Errno::EAGAIN
    retry
  end
end
```

### 非堵塞时连接
`connect_nonblock`和其他非堵塞式IO方法有些不同  
其他方法要么是完成操作，要么产生一个异常，而`connect_nonblock`则是保持操作继续运行，并产生一个异常  
如果`connect_nonblock`不能立即发起到远程主机的连接，它会在后台继续执行操作，并产生`Errno::EINPROGRESS`，提醒我们操作仍在进行中  
```ruby
require 'socket'

socket = Socket.new(:INET, :STREAM)
remote_addr = Socket.pack_sockaddr_in(80, 'google.com')

begin
  # 发起一个非堵塞式连接
  socket.connect_nonblock(remote_address)
rescue Errno::EINPROGRESS
  # 操作在进行中
rescue Errno::EALREADY
  # 之前的非堵塞式连接已经在进行当中
rescue Errno::ECONNREFUSED
  # 远程主机拒绝连接
end
```

## 连接复用
连接复用是指同时处理多个活动套接字(不是并行，和多线程无关)  
利用非堵塞式IO来避免在特定的套接字功能上陷入停止，以实现一个多条TCP连接中的可用数据的服务器
```ruby
# 创建一个连接数组
connections = [<TCPSocket>, <TCPSocket>, <TCPSocket>]

loop do
  # 处理每个连接
  connections.each do |conn|
    begin
      # 采用非堵塞的方式从每个连接中进行读取
      # 处理接收到的任何数据，不然就尝试下一个连接
      data = conn.read_nonblock(4096)
      process(data)
    rescue Errno::EAGAIN
    end
  end
end
```
每一次`read_nonblock`都需要使用至少一个系统调用，如果没有数据可读，服务器会浪费大量的处理周期。这时候就要使用`select(2)`了

### select(2)
下面是处理多个TCP连接中可用数据的更好的方法
```ruby
connections = [<TCPSocket>, <TCPSocket>, <TCPSocket>]

loop do
  # 查询select哪一个连接可以进行读取
  # 接受若干个对象，然后告知哪一个可以进行读写
  ready = IO.select(connections)

  # 从可用连接中进行读取
  readable_connections = ready[0]
  readabel_connections.each do |conn|
    data = conn.readpartial(4096)
    process(data)
  end
end
```
`IO.select`极大的降低了处理多个连接的开销  

#### IO.select的一些参数
`select`可以使用三个数组作为参数
```ruby
for_reading = [<TCPSocket>, <TCPSocket>, <TCPSocket>]
for_writing = [<TCPSocket>, <TCPSocket>, <TCPSocket>]

IO.select(for_reading, for_writing, for_writing)

# => [[<TCPSocket>], [], []]
```
第一个参数是希望从中进行读取的IO对象数组  
第二个参数是希望进行写入的IO对象数组  
第三个参数是可以忽略，除非对带外数据(out-of-band data)感兴趣  
注意，参数只能是数组  
`IO.select`返回一个包含三个元素的嵌套数组，分别对应它的参数列表，
第一个元素包含可供无拥塞读取的IO对象  
第二个元素包含看可以进行无拥塞写入的IO对象  
第三个元素包含了使用异常条件的对象  

> IO.select会阻塞，它是一个同步方法调用

其实`IO.select`还有第四个参数，一个整数或浮点数，指定超时，返回`nil`  

### 读/写之外的事件
`IO.select`可以监视套接字的读写状态，其实，它还有一些其他的作用  

#### EOF
如果监视某个套接字的可读性时，它接收到了一个`EOF`，那么该套接字会作为可读套接字数组的一部分被返回

#### accept
在监视某个服务器套接字的可读性时，它收到了一个接入的连接，那么个该套接字会作为可读套接字数组的一部分返回，这种套接字需要特殊处理，应该使用`accept`而不是`read`

#### connect
上面说过，`connect_nonblock`如果不能给你立刻完成连接，则会产生`Errno::EINPROGRESS`。可以使用`IO.select`了解后台连接是否已经完成  
```ruby
require 'socket'

socket = Socket.new(:INET, :STREAM)
remote_addr = Socket.pack_sockaddr_in(80, 'google.com')

begin
  socket.connect_nonblock(remoter_addr)
rescue Errno::EINPROGRESS
  IO.select(nil, [socket])

  begin
    socket.connect_nonblock(remote_addr)
  rescue Errno::EISCONN
    # 成功
  rescue Errno::ECONNREFUSED
    # 被远程主机拒绝
  end
end
```
这段代码尝试进行`connect_nonblock`并处理`Errno::EINPROGRESS`，这意味着连接过程发生在后台  
利用`IO.select`监视套接字状态是否变得可写。如果发生改变，就可以确定底层的连接已经完成，为了获知状态，我们只需要再次试用`connect_nonblock`即可，如果产生了`Errno::EISCONN`就表明套接字已经连接到远程主机。其他异常表明连接远程主机时出现错误     

### 高性能复用
`IO.select`是ruby中进行复用的唯一手段。大多数操作系统支持多种复用方法，`select`几乎是这些方法中最古老，用的最少的  
`IO.select`的性能通它所监视的连接数呈线性关系，而且`select`系统调用收到`FD_SETSIZE`的限制，这是一个定义在本地C代码库中的宏。`select`无法对编号大于`FD_SETSIZE`(多数系统是1024)的文件描述符进行监视  
Linux的`epoll`以及BSD的`kqueue`系统调用比`select`和`poll`的效果更好，更能更先进  

## Nagle算法
Nagle算法是一种默认应用于所有的TCP连接的优化  
这种优化最适合那些不进行缓冲，每次只发送很少数据量的应用程序(不满足这些条件的服务器应该禁用)  
来，看下算法  
程序向套接字执行写操作后，有下面3种可能：  
1. 如果本地缓冲区中有足够的数据可以组成一个完成的TCP分组，立即发送  
2. 如果本地缓冲区没有尚未处理的数据，接收端的数据也都全部已经确认接收，那么就立即发送  
3. 如果接收端还有未确定的答复(acknowledgement)，也没有足够的数据组成一个完整的TCP分组，那么就将数据放入本地缓冲区  
该算法避免发送大量的微型TCP分组(telnet协议，每次敲击键盘，字符都会被立刻发送到网络上)  
如果是HTTP协议，它的请求/响应至少能够组成一个TCP分组，因此Nagle算法除了会延缓最后一个分组发送之外，一般不会造成什么影响  
考虑到Ruby缓冲以及TCP之上所实现的大部分常用协议，应该禁用该选项  
```ruby
require 'socket'

server = TCPServer.new(4481)

# 禁用Nagle算法
server.setsockopt(Socket::IPPROTO_CTP, Socket::TCP_NODELAY, 1)
```

### 消息划分
之前都是`EOF`来表明消息的终止，每次都是一个新连接，增加了开销   
多条消息重用连接的想法同HTTP keep-alive特性背后的理念是一样的，在多个请求间保持连接开放(包括客户端和服务器协商的划分消息的方法)，通过避免打开新的连接节省资源   
> 说白了，有无数种方法可以划分消息，有的复杂有的简单，取决于自己  

#### 使用新行
使用新行(newlines)是划分消息的简单方法，如果确定客户端和服务器都运行在同样的操作系统上(windows和linux的换行符不一样)，甚至可以在套接字上退而使用`IO#gets`和`IO#puts`发送带新行的消息  
服务器：  
```ruby
def handle(connection)
  loop do
    request = connection.gets
    break if request == 'exit'
    connection.puts process(request)
  end
end
```
> 当然，最好还是`IO.select`一下   

客户端：  
```ruby
def initialize(host, port)
  @connection = TCPSocket.new(host, port)
end

def get
  request "GET #{key}"
end

def set
  request "SET #{key} #{value}"
end

def request(string)
  @connection.puts(string)

  # 读取数据，知道收到新行获取答复
  @connection.gets
end
```
> GET /index.html HTTP/1.1\r\n  
> \r\n 所有的HTTP客户端/服务器都能识别这些新行，不管什么操作系统  

#### 使用内容长度
另一中划分消息的方法是指定内容长度(content length)
消息发送方先计算出消息的长度，使用`pack`将其转换成固定宽度的整数，后面跟上消息主体一并发送  
接收方先获取这个长度，然后严格读取长度所指定的字节数  
```ruby
# 获得一个随机的固定宽度整数的大小
SIZE_OF_INT = [11].pack('i').size

def handle(connection)
  # 用pack将信息长度转换成固定宽度的整数，对它进行read及unpack操作
  packed_msg_length = connection.read(SIZE_OF_INT)
  msg_length = packed_msg_length.unpack('i').first

  # 输入给定长度的消息
  request = connection.read(msg_length)
  connection.write process(request)
end
```
客户端：  
```ruby
payload = 'SET prez obama'

# 用pack将消息长度转换成固定宽度的整数
msg_length = payload.size
packed_msg_length = [msg_length].pack('i')

# 写入消息长度以及消息主体
connection.write(packed_msg_length)
connection.write(payload)
```
客户端用pack将消息长度转换成一个与处理器要求相一致的整数(native endian integer)。这非常重要，因为它保证了任何给定的整数都会被转化成同样数量的字节。如果不是这样，服务器就不知道究竟应该读取多少数位来得到消息的长度，用了这个方法，客户端/服务器在通信时总是会使用同样数量的字节表示消息长度  


## 超时
### IO.select
```ruby
require 'socket'
require 'timeout'

timeout = 5

Socket.tcp_server_loop(4481) do |connection|
  begin
    # 发起一个初始化read(2)
    # 应为要求套接字上有被请求的数据，有数据可读时，避免使用select
    connection.read_nonblock(4096)
  rescue Errno::EAGAIN
    # 监视连接是否可读
    if IO.select([connection], nil, nil, timeout)
      # IO.select会将套接字返回
      # 不反悔nil就意味着套接字可读
      retry
    else
      raise Timeout::Error
    end
  end

  connection.close
end
```

### 接受超时
`accept`能和`IO.select`很好的合作
```ruby
server = TCPServer.new(4481)
timeout = 5

begin
  server.accept_nonblock
rescue Errno::EAGAIN
  if IO.select([server], nil, nil, timeout)
    retry
  else
    raise Timeout::Error
  end
end
```

### 连接超时
```ruby
require 'socket'
require 'timeout'

socket = Socket.new(:INET, :STREAM)
remote_addr = Socket.pack_sockaddr_in(80, 'google.com')
timeout = 5

begin
  # 发起到google.com端口80的非阻塞连接
  socket.connect_nonblock(remote_addr)
rescue Errno::EINPROGRESS
  # 报名连接过程正在进行中
  # 监视套接字合适可读，这意味着连接完成
  # 一旦再次进入上面的代码，就会转入EISCONN rescue代码块
  # 然后运行值begin代码块外，在这里套接字就已经可以使用了
  if IO.select(nil, [socket], nil, timeout)
    retry
  else
    raise Timeout::Error
  end
rescue Errno::ESICONN
  # 表明连接已经顺利完成
end

socket.write('ohai')
socket.close
```

## DNS查询
```ruby
require 'socket'
socket = TCPSocket.new('googel.com', 80)
```
缓慢的DNS服务器会堵塞ruby进程

#### MRI和GIL
标准Ruby实现(MRI)包含了一个全局解释器锁(Global Interpreter Lock, GIL)。它确保Ruby解释器一次只做一件有潜在危险的事情。在多线程环境中才能真正发挥作用。当一个线程活动时，其他线程全部出于堵塞状态。这使得MRI可以使用更安全、更简单的代码来编写  
好在GIL可以理解堵塞式IO。如果一个线程正在进行堵塞式IO(例如一个堵塞式read)，MRI会释放GIL并让另一个线程继续执行。当堵塞IO调用完成后，线程就等待下一次运行  

#### resolv
所幸的是，Ruby在标准中提供了该问题的解决方法。resolv库为DNS提供了一套纯Ruby的替代方案，这使得MRI能够为长期堵塞的DNS查询释放GIL。  
resolv库有自己的API，不过标准库中也包含了一个库，它对Socket进行了猴子补丁，使他可以使用resolv
```ruby
require 'resolv'
require 'resolv-replace'
```

## SSL套接字
SSL套接字：将不安全的套接字升级到安全的SSL套接字  
在ruby中常用openssl实现  
```ruby
require 'socket'
require 'openssl'

def main
  server = TCPServer.new(4481)

  # 建立ssl环境
  ctx = OpenSSL::SSL::SSLContext.new
  ctx.cert, ctx.key = create_self_signed_cert(
    1024,
    [['CN', 'localhost']],
    'Generated by Ruby/OpenSSL'
  )
  # 安全性角度
  # OpenSSL::SSL::VERIFY_NONE表示允许未经验证的SSL证书
  ctx.verify_mode = OpenSSL::SSL::VERIFY_PEER

  # 建立TCP服务器的SSL包装器
  ssl_server = OpenSSL::SSL::SSLServer.new(server, ctx)

  # 在SSL套接字上接受连接
  connection = ssl_server.accept

  # 处理连接
  connection.write("Bah now")
  connection.close
end

# 接下来的代码直接取自webrick/ssl，
# 它生成一个适用于Context对象使用的自签名SSL证书。
def create_self_signed_cert(bits, cn, comment)
  rsa = OpenSSL::PKey::RSA.new(bits){ |p, n|
    case p
    when 0; $stderr.putc "." # BN_generate_prime
    when 1; $stderr.putc "+" # BN_generate_prime
    when 2; $stderr.putc "*" # 搜索good prime,
      # n = #of try,
      # 数据同样取自BN_generate_prime。
    when 3; $stderr.putc "\n" # 找到good prime, n==0 - p, n==1 - q, # 数据同样取自BN_generate_prime
    else; $stderr.putc "*" # BN_generate_prime
    end
  }
  cert = OpenSSL::X509::Certificate.new
  cert.version = 2
  cert.serial = 1
  name = OpenSSL::X509::Name.new(cn)
  cert.subject = name
  cert.issuer = name
  cert.not_before = Time.now
  cert.not_after = Time.now + (365*24*60*60)
  cert.public_key = rsa.public_key
  ef = OpenSSL::X509::ExtensionFactory.new(nil,cert)
  ef.issuer_certificate = cert
  cert.extensions = [
    ef.create_extension("basicConstraints","CA:FALSE"),
    ef.create_extension("keyUsage", "keyEncipherment"),
    ef.create_extension("subjectKeyIdentifier", "hash"),
    ef.create_extension("extendedKeyUsage", "serverAuth"),
    ef.create_extension("nsComment", comment),
  ]
  aki = ef.create_extension("authorityKeyIdentifier",
                            "keyid:always,issuer:always")
  cert.add_extension(aki)
  cert.sign(rsa, OpenSSL::Digest::SHA1.new)
  return [ cert, rsa ]
end

main
```
> 生成了一个自签名的SSL证书，用它来支持SSL连接

客户端：
```ruby
require 'socket'
require 'openssl'

socket = TCPSocket.new('0.0.0.0', 4481)

ssl_socket = OpenSSL::SSL::SSLSocket.new(socket)
ssl_socket.connect

ssl_socket.read
```

## 紧急数据
### 发送紧急数据
TCP紧急数据又叫"带外数据"(out-of-band data)，支持将数据推送到队列的前端
`Socket#send`就像一个特殊的`Socket#write`，其实如果不带参数，两者相同  
```ruby
# 两者相同
socket.write 'foo'
socket.send 'foo'

# 发送紧急数据
# 第二个参数：标志（flag）
# 一次只能发送一个字节
socket.send '!', Socket::MSG_OOB
```

### 接受紧急数据
使用`Socket#revc`
```ruby
require 'socket'

Socket.tcp_server_loop(4481) do |connection|
  # 优先接受紧急数据
  urgent_data = connection.recv(1, Socket::MSG_OOB)

  data = connection.readpartial(1024)
end
```

### 紧急数据和`IO.select`
`IO.select`可以监视多个套接字上的紧急数据，不过需要留意，`IO.select`第三个参数是一个IO对象数据，需要回去这些IO对象上的带外数据
```ruby
for_reading = [<TCPSocket>, <TCPSocket>, <TCPSocket>]
for_writing = [<TCPSocket>, <TCPSocket>, <TCPSocket>]

# for_reading 作为第三个元素
# 如果这些套接字接收到了紧急数据，会被包含在返回的第三个数组中
IO.select(for_reading, for_writing, for_reading)
```

### SO_OOBINLINE选项
将紧急数据放入普通数据流  
```ruby
require 'socket'

Socket.tcp_server_loop(4481) do |connection|
  # 在带内随同其他普通数据接受紧急数据
  connection.setsockopt :SOCKET, :OOBINLINE, true

  # 当遇到紧急数据时停止读取
  # 并不会被优先读取
  connection.readpartial(1024) # => 'foo'
  connection.readpartial(1024) # => '!'
end
```

## 网络架构模式
6种网络架构模式  

### 串行化
串行化 架构中，所有客户端连接是依次进行的，因为不涉及并发，多个客户端不会同时接受服务  
串行化的护理流程很直观：  
1. 客户端连接  
2. 客户端/服务器交换请求及响应  
3. 客户端断开连接  
4. 返回到步骤1  

### 单连接进程
接受连接后，服务器fork出一个子进程，这个子进程唯一的作用就是处理处理新连接。连接处理完毕之后就退出  
处理流程：  
1. 连接抵达服务器  
2. 主服务器进程接受该连接  
3. 衍生出一个和服务器一模一样的新子进程  
4. 服务器进程返回步骤1，由子进程并行处理连接  
不管何时，总有一个父进程等着接受连接，但可能会有多个子进程分别处理连接  
```ruby
loop do
  pid = fork do
    ...
  end
  # monitor
  Process.detach(pid)
end
```
优势：简单、易于理解(不用留心边界情况(edge cases)、没有锁或者竞争条件)  
劣势：子进程占用过多的资源(需要preforking)  

### 单连接线程
生成线程而不是进程  
> 线程和进程都可以用于并行操作  

```ruby
loop do
  Thread.new do
    ...
  end
end
```
通过限制活动线程数解决无限制添加线程数  
> 线程是从同一个服务器实例中产生的，所有的线程都共享同一个实例变量，所以有一条tips:  
> 让每个线程获得它自己的连接对象  

### Preforking
依赖进程作为并行操作手段，但并不为每个接入的连接衍生对用的子进程，而是在服务器启动后，连接到达之前就衍生出一批进程  
流程：  
1. 主服务器进程创建一个侦听套接字  
2. 主服务器进程衍生出一大批子进程  
2. 每个子进程在共享套接字上接受连接，然后进行独立处理  
4. 主服务器随时关注子进程  
这个流程的重点是：主服务器进程打开侦听套接字，却并不接受该套接字之上的连接  
然后衍生出预定义数量的一批子进程，每个子进程都有一份侦听套接字的副本  
子进程在各自的侦听套接字上调用`accept`，不再考虑父进程  
> 这个模式的精妙之处在于，无需担心负载均衡或是子进程连接的同步，因为内核已经替我们完成了这个工作  
> 对于多个进程试图在同一个套接字的不同副本上接受(`accept`)连接的问题，内核会困很负载并确保只有一个套接字副本可以接受某个特定的连接  

### 线程池
同preforking差不多，线程池在服务器启动之后会生成一批线程，将处理连接的任务交给独立的线程来完成  

### 事件驱动
事件驱动(基于Reactor模式)结合了单线程和单进程，它至少可以达到之前模式所提供的并行操作级别  
一个中央连接复用器(Reactor核心)为中心，连接生命周期中的每个阶段都被分解成单个的事件，这些事件可以按照任意的次序交错并处理。连接的不同阶段只是可能的IO操作而已: `accpet` `read` `write` `close`  
中央复用器监视所有活动连接的事件，在触发事件时分派相关的代码：  
1. 服务器监听侦听套接字，等待接入的连接  
2. 将接入的新连接加入到套接字列表进行监视  
3. 服务器现在要监视活动连接以及侦听套接字  
4. 当某个活动连接可读时，服务器从该连接读取一块数据并分派相关的回调函数  
5. 当某个活动连接仍然可读时，服务器读取另一块数据并再次分配回调函数  
6. 服务器收到另一个新连接，将其加入套接字列表进行监视  
7. 服务器注意到第一个连接已经可以写入，因而将响应信息写入该连接  

该模式以极高的并发处理能力而闻名，能够处理成千上万的并发连接，其他的都受到进程/线程数的限制  
主要的劣势是所施加的编程模型，一方面这个模型更加简单，无需处理众多的进程/线程(没有共享内存，同步，越界进程等问题)
<span style="color: yellow;">但是绝不能堵塞Reactor</span>

### 混合模式
#### Nginx
Nginx在核心部分使用的是Preforking模式，但是在每个衍生进程中使用的是事件驱动模式  
Preforking最大限度的利用多核以及服务器资源  

#### Puma
因为Puma大量倚重线程，所以并没有使用GIL(Rubinius或JRuby)  
高层上，Puma使用线程池提供并发，主线程一直用于accept新的连接，这便是不适用keep-alive的HTTP连接的处理方法  
当然Puma也支持keep-alive，这时Puma就不会关闭连接，这时Puma就不再只是accept这个连接了，它需要监视该连接上的新请求，并进行处理，这事通过事件驱动类型实现的  

#### EventMachine
