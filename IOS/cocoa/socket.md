# [iOS socket](http://www.jianshu.com/p/3e4f3de18e3b)
socket是支持TCP/IP协议的网络通信的基本操作单元，包含进行网络通信所必须的五种信息：连接使用的协议，本地主机的IP地址，本地进程的协议端口，远程主机的IP地址，远程进行的协议端口  

多个TCP连接或者多个应用程序可能需要通过同一个TCP协议端口传输数据。为了区别不同的应用程序进程和连接，计算机操作系统为应用程序与TCP/IP协议交互提供了socket接口。应用层可以和传输层通过socket接口，区分来自不同应用程序进行或网络连接的通信，实现数据传输的并发服务  

![Socket的使用](http://upload-images.jianshu.io/upload_images/1170656-6b9392fad31b711d.png?imageMogr2/auto-orient/strip%7CimageView2/2/w/1240)  

### Socket使用的函数库
1. 创建socket  
  ```objective-c
  Socket(af,type,protocol) //建立地址和套接字的联系
  bind(sockid, local addr, addrlen) //服务器端侦听客户端的请求
  listen(sockid, quenlen) // 建立服务器/客户端的连接(面向连接TCP)
  ```

2. 客户端请求连接  
  ```objective-c
  Connect(sockid, destaddr, addrlen) //
服务器端等待从编号为Sockid的socket上接收客户连接请求
  newsockid = accept(sockid, clientaddr, paddrlen) // 发送/接收数据
  ```

3. 面向连接  
  ```objective-c
  send(sockid, buff, bufflen)
  recv()
  ```

4. 面向无连接  
  ```objective-c
  sendto(sockid, buff, ... , addrlen)
  recvfrom()
  ``` 

5. 释放套接字  
  ```objective

  ```
