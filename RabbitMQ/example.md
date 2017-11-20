## [RabbitMQ blog](https://ruby-china.org/topics/22332)
传统的用户与服务器都通过HTTP API以"我要某某某"方式进行  
一个武断的解决方案是：『HTTP缓存』  
HTTP Cache最大的问题是：**生命周期**  
几乎没有办法来确定资源需要被缓存多久。如果短，请求就会多；如果长，数据更新不及时。  

#### 消息传递是解决之道  
面向对象的变成原则推崇：tell，don't ask  
无需让消费者每次问生产者要资源，而是生产者主动告诉消费者发生了什么变化  

#### RabbitMQ是什么  
一个AMQP的协议的开源消息中介，队列系统的实现  

#### 简单的架构：博客的仪表盘
仪表盘应用无需向博客应用的HTTP API索要最近的博文，可以让博客应用主动说出每一个最新发布的博文  
![flow](http://blog-img-bed.qiniudn.com/event_sourcing_diagram.png   
- P - RabbitMQ 生产者  
- X - RabbitMQ 交换  
- Queue - RabbitMQ 队列  
- C - RabbitMQ消费者  
