#### 1. 什么是RunLoop
- 内部就是一个do{} while循环，在这个循环内部不断处理各种任务（如：source,timer,Observer）  
- RunLoop就是为线程而存在的，线程执行完任务后就不能再次执行任务，因为默认情况下，线程是没有开启RunLoop的，如果开启RunLoop之后，线程执行完任务会一直等待，知道再次接收到任务，线程销毁前，会先释放这个Runloop  

#### 2. RunLoop基本作用
- 保持程序的持续运行，保持线程的持续运行  
- 处理App中的各种事件(如：触摸，定时器，Selector)  
- 节省CPU资源，提供程序性能：工作或休息  

#### 3. RunLoop对象
iOS中有两套API来访问和使用RunLoop  
1. Fundation(纯OC)框架中的  NSRunLoop  
  ```objective-c
  // 获得当前线程的RunLoop对象  
  [NSRunLoop currentRunLoop];
  // 获得主线程的RunLoop对象
  [NSRunLoop mainRunLoop];
  ```
2. Core Fundation(纯C)框架  CGRunLoopRef  
  ```objective-c
  // 获得当前线程的RunLoop对象
  CGRunLoopGetCurrent();
  // 获得主线程的RunLoop对象  
  CFRunLoopGetMain();
  ```

#### 4. RunLoop与线程  
- 每条线程都有唯一与之对应的RunLoop对象  
- 主线程的RunLoop系统已经自动创建好了，子线程的RunLoop需要手动创建  
- RunLoop在第一次获取时由系统自动创建，在线程结束时销毁   
