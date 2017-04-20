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
`accpet`调用是阻塞式的，它将还未处理的连接从队列中弹出(pop)而已。如果队列为空，那么它就一直等，知道有连接被加入到队列为止

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

servers = Socket.tcp_server_socket(4481)
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
> 因为客户端不需要通过某个一直端口访问，没有人需要知道他们的端口号

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
在客户端的生命周期中，很可能在服务器还没准备好的时候客户端就发起了连接，同样也很有可能连接了一个并不存在的服务器，这两种情况很相似，因为TCP所具备的容错性，它会经最大可能等待远程主机的回应
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
EOF并不是一个字符，EOF更想是一个状态事件(state event)  
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
TCP套接字默认将sync设为true。这就逃过了Ruby的内部缓冲，否则就又要多出一个缓冲层了  
> 为什么要缓冲区
> 提高性能。将缓慢的网络发送过程交由幕后操作，让`write`可以立即返回。幕后还可以将所有还未执行的写操作汇总到一起，在发送时进行分组及优化，在实现最佳性能的同事避免网络过载。在网络层面上，发送大量的小分组会引发客观的开销

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

### SO_TYPE
