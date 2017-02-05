# Node
## Node 旨在提供一种简单的构建可伸缩网络程序的方法
- 当前的服务器程序有什么问题：如 Java 和 PHP
  这类语言，每个连接都会生成一个新线程，每个线程可能需要2MB的内存，一个8GB
RAM的系统上，理论上最大的并发连接数量是4,000
个用户，扩展需要更多的服务器，而且会增加技术成本，即在各个服务器之间共享资源
- Node
  更改连接到服务器的方式。每个连接发射一个在Node引擎的进程中运行的事件，而不是为每个连接生成一个新的OS线程。Node不允许锁，不会直接阻塞I/O调用。Node宣城，运行它的服务器能支持数万个并发连接

## Node 如何工作
- Node 本身运行 V8 JavaScript引擎，引擎负责解释并执行代码。

### 事件驱动编程
面向对象见鬼去吧！
和点击等事件类似，一个连接被建立，就是一个事件，数据通过连接进行接收，这也是一个时间；数据通过连接停止，这还是一个事件。

JS是一种很棒的事件驱动语言，因为它允许使用匿名函数和闭包
```javascript
// = 右边就是匿名函数
var double = function(x) { return 2 * x; }

// 第一个括号内为匿名函数，第二个括号表示调用，并传入参数
(function(x, y){
  alert(x + y);
})(2, 3);
```

## Node对什么有好处
###
Node非常适合以下情况：在响应客户端之前，预计可能有很高的流量，但所需的服务器逻辑和处理不一定很多。Node表现出众的典型示例包括：
- RESTful API
这是适合Node的最理想情况，可以构建它来处理万条连接。它不需要大量逻辑，本质上只是从某个数据库中查找值并组成一个响应,请求和响应都只是少量文本，因此流量不高，一台机器也可以处理很多
- Twitter 队列
  - 接受tweets并将其写入数据库：Node迅速收集Tweet，讲他们写入一个内存排队机制(如memcached)，另一个进程可以写入数据库。
  - 如若是一个PHP服务器(常规的服务器会自己尝试处理对数据库本身的写入)，每个tweet都会在写入时导致一个短暂的延迟，因为数据库调用正在堵塞通道，一个这样设计机器每秒大概能处理2000挑tweet，相反Node能处理每个连接而不会堵塞通道，从而能捕获更多的tweets
