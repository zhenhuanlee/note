# TCP Socket编程
## 建立套接字
### Ruby的套接字库
```ruby
require 'socket'
# socket库是Ruby标准库的组成部分。同opensll、zlib和curses这些类似，socket库与其所以来的
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
规则：  
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
`accpet`调用是阻塞式的，它将还未处理的连接从队列中弹出(pop)而已。如果队列为空，那么它就一直等，知道有连接被加入到队列为之

#### accept调用返回一个数组
第一个元素是建立好的连接，第二个元素是一个A`Addrinfo`对象，该对象描述了客户端连接的远程地址  
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
每一个TCP连接诶都由`local_address`(本地主机的端点)，`remote_address`(另一端端点)组成

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

# 该连接随后也许不再需要写入数据，但是可能任然需要进行读取
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

```
等同于
```ruby
require 'socket'

server = Socket.new(:INET, :STREAM)
addr = Socket.pack_sockaddr_in(4481, '0.0.0.0')
server.bind(addr)
server.listen(5)
```
> 创建一个TCPServer实例返回的实际上并不是Socket实例，而是TCPServer实例，两者的接口几乎一样，但是还是有些重要的差异  
> 最明显的就是`TCPServer#accept`只返回连接，而不是返回remote_address  
> 监听队列长度默认5，如果要设置，调用`TCPServer#listen`

这个Ruby包装器会返回两个TCP套接字，一个可以通过IPv4连接，另一个可以通过IPv6连接，两者都在同一个端口上进行监听  
```ruby
require 'socket'

servers = Socket.tcp_server_socket(4481)
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
不能简洁诶更多，它只是`Socket.tcp_server_sockets`和`Socket.accept_loop`的一个包装器  


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
