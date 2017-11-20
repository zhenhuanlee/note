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

### RabbitMQ 三种exchange模式比较
所有生产者提交的消息都由Exchange来接收，然后Exchange按照特定的策略转发到Queue进行存储  
RabbitMQ提供了四种Exchange：fanout, direct, topic, header  
header用太少  
1. Fanout  
  任何发送到Fanout Exchange的消息都会被转发到与该Exchange绑定(binding)的所有Queue上  

2. Direct  
  任何发送到Direct Exchange的消息都会被转发到RouteKey中指定的Queue  

3. Topic  
  任何发送到Topic Exchange的消息都会被转发到所有关心RouteKey的指定话题的Queue上，这个支持模糊匹配  
  '#'表示0或若干个关键字 '\*'表示一个关键字   
  如："#.log.#"表示该队列关心所有涉及log的消息（MQ.log.error）  

## 重要概念
  - ConnectionFactory、Connection、Channel  
  	- connectionFactory是创建连接的工厂  
  	- connection是RabbitMQ客户端与服务器端的socket连接  
  	- channel是我们与RabbitMQ交互的接口   
  - Queue  
  	Queue是RabbitMQ的内部对象，消息只能保存在Queue中  
  - Message acknowledgment  
  	消息确认，可以通过设置它，来向RabbitMQ服务器发送一个回执，收到回执后，消息才从Queue中删除  
  - Exchange（交换机）  
  	生产者将消息发送到Exchange，由Exchange将消息路由到一个或多个Queue中(或丢弃)。Exchange type有四种： fanout, direct, topic, headers  
  - Routing key  
  	消息路由规则，与Exchange type及binding key 联合使用才能最终生效，长度限制为255bytes  
  - Binding Key  
  	当binding key 与routing key相匹配时，消息将会被路由到对应的Queue中。在绑定多个Queue到同一个Exchange时，这些binding允许使用同样的binding key  

  ## RabbitMQ常用使用模式  
  1. Work queues：工作队列，为不同的worker分配task(关键词，`prefetch`)
  	> 一个生产者，一个队列，多个worker
  	> 比如：publisher发布了10个任务，如果worker1收到4个任务，那worker2就会收到6个任务  

  2. publish/subscribe：发布/订阅模式，一次性向多个消费者发送消息
  	> 一个生产者，多个队列，多个消费者  
  	> 生产者将消息发送给exchange，exchange按照规则，路由到各个queue中，消费者接收   

  3. Routing：有选择性的接收消息(按照一定的路由规则)  
  4. Topics：基于一定的pattern来接收消息  
  5. RPC：可以得到远程worker的处理结果  
