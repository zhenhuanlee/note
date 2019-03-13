# Linux IO模型
### 常见的IO模型有四种  
1. 同步阻塞IO(Blocking IO)：传统的IO模型  
2. 同步非堵塞IO(Non-Blocking IO)：默认创建的socket都是堵塞的，非堵塞IO要求socket被设置为NONBLOCK。   
3. IO多路复用(IO Multiplexing)：经典的Reactor模型，有时也成为异步堵塞IO  
4. 异步IO(Asynchronous IO)：经典的Proactor设计模式，也称为异步非堵塞IO  

同步和异步的概念描述的是用户线程与内核的交互方式：  
1. 同步指用户线程发起IO请求后，需要等待或者轮询内核IO操作完成后才能继续执行  
2. 异步指用户线程发起IO请求后仍然继续执行，当内核IO操作完成后会通知用户线程，或者调用用户线程注册的回调函数  

阻塞和非阻塞的概念描述的是用户线程调用内核IO操作的方式：  
1. 阻塞是指IO操作需要彻底完成后才返回到用户空间  
2. 非堵塞指IO操作被调用后立即返回给用户一个状态值，无需等到IO操作彻底完成  

#### 一 同步堵塞IO
![同步堵塞IO](https://img-blog.csdn.net/20150308185933156)  
用户线程通过系统调用read发起IO读操作，由用户空间转到内核空间。内核等到数据报到达后，然后将接收的数据拷贝到用户空间，完成read操作  
用户需要等待read将socket中的数据读取到buffer中后，才能继续处理接收到的数据。整个IO请求的过程中，用户线程是被堵塞的，这将导致用户在发起IO请求时，不能做任何事情，对CPU利用低  

#### 二 同步非堵塞IO  
同步非堵塞IO是在同步阻塞IO的基础上，将socket设置为NONBLOCK。这样做用户线程可以发起IO请求后立即返回  
![同步非阻塞IO](https://img-blog.csdn.net/20150308185912050)  
由于socket是非堵塞方式，用户线程发起IO请求时立即返回，但并未读取到任何数据，用户县城需要不断地发起IO请求，知道数据到达后才能读取到数据  
```
while(read(socket, buffer) != SUCCESS);
process(buffer);
```
用户不断地调用`read`，尝试读取socket中的数据，直到成功后，才继续粗粒接收到的数据。  
整个IO请求的过程中，虽然用户线程请求后可以立即返回，但是为了等到数据，仍需要不断轮询  

#### 三 IO多路复用  
![IO多路复用](https://img-blog.csdn.net/20150308185957524)  
IO多路复用和同步阻塞模型差不多，但是支持一个线程同时处理多个socket的IO请求  
IO多路复用模型使用Reactor设计模式实现了这一机制  
![reactor](https://img-blog.csdn.net/20150315171322938)  
在reactor模式中，先对每个client注册感兴趣事件，然后有一个线程专门去轮询每个client是否有事件发生，当有事件发生时，变顺便处理每个事件，当所有事件处理完之后，便再转去继续轮询  
![](https://img-blog.csdn.net/20150308190149282)  

#### 异步IO
真正的异步IO需要操作系统更强的支持。在IO多路复用模型中，事件循环将文件句柄的状态事件通知给用户线程，由用户线程自行读取数据，处理数据。而在异步IO模型中，当用户线程收到通知时，数据已经被内核读取完毕，并放在了用户线程指定的缓冲区内，内核在IO完成后通知用户线程直接使用即可  
异步IO模型使用了Proactor设计模式  
![Proactor设计模式](https://img-blog.csdn.net/20150308190212605)  
Proactor和Reactor模式在结构上比较相似，不过用户端在使用方式上差别比较大，reactor模式用，用户线程通过reactor对象注册感兴趣的事件监听，然后事件触发时调用事件处理函数。而Proactor模式中，用户线程将AsynchronousOperation（读/写等）、Proactor以及操作完成时的CompletionHandler注册到AsynchronousOperationProcessor(AOP)，AOP使用Facade模式提供了一组异步操作API供用户使用，当用户线程调用异步API后，便继续执行自己的任务  

