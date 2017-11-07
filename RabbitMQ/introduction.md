RabbitMQ是一个erlang开发的AMQP(advanced message queue)。

### 应用场景
对于一个大型软件系统而言，它会有很多的组件或者模块，或者说子系统。这些模块如何通信，如果使用socket，那么久需要解决很多问题：  
  1. 信息的发送者和接受者如何保持这个连接，如果一方连接中断，着期间的数据如何防止丢失  
  2. 如何降低发送者和接受者的耦合  
  3. 如何让Priority搞的接受者先接收到数据  
  4. 如何做到load balance？有效均衡接受者的负载  
  5. 如何有效的将数据发送到相关的接收者？  
  6. 如何做到可扩展，甚至将这个通信模块发送到cluster上？
  7. 如何保证接收者收到了完整，正确的数据？  

### 系统架构
![应用场景系统架构](http://img.blog.csdn.net/20140220173559828)  

- 它的角色就是维护一条从Producer到Consumer的路线，保证数据能够按照指定的方式进行传输  
ClientA & B：也叫Producer，数据的发送方。create messages and publish(send) them to a broker server(RabbitMQ)。 一个Message有两部分：payload（数据）和label（标签）。RabbitMQ通过这个label来决定把这个message发送给哪个consumer。

- client1, 2, 3：也叫consumer，数据的接收方。consumers attach to a broker server(RabbitMQ) and subscribe to a queue。可以发送给多个consumers，label会被删除  

- 对于一个message 从 producer到consumer的正确传递，还有三个概念：exchanges， queues and bindings
  - Exchanges are where producers publish their messages  
  - Queues are where the message end up and are received by consumers  
  - Bindings are how the message get routed from the exchange to particular queues  

- 还有一些：connection, channel  
  - connection: 就是一个TCP连接，Producer和Consumer都是通过TCP连接到RabbitMQ server的  
  - channel: 虚拟连接。它建立在上述的TCP连接中。数据流动都是在channel中进行的，一般来说，程序建立起TCP连接，第二部就是建立这个channel  
