# 多线程
iOS的App启动之后，默认开启一条主线程(串行)并且开启[Runloop](http://blog.ibireme.com/2015/05/18/runloop/)来处理消息
> 主线程应该能自由响应用户事件，并发也会带来一定的开销(调度)，增加编写和调试的难度    

#### Runloop
Runloop是一种循环，和while的循环不同，Runloop是一种‘闲’等待，可以类比linux的epoll。当没有事件时，Runloop会进入休眠状态，有事件发生时，Runloop会去对应的Handler处理事件。Runloop可以让线程在需要做事的时候忙起来，不需要的话就让线程休眠  
![runloop](https://developer.apple.com/library/ios/documentation/Cocoa/Conceptual/Multithreading/Art/runloop.jpg)  
从input source和timer source接受事件，然后线程中处理事件  

#### Runloop与线程  
Runloop和线程是绑定在一起的，每个线程(包括主线程)都有一个对应的Runloop对象。这个对象由系统提供，自己无法创建  
主线程的Runloop会在应用启动的时候完成，其他线程的Runloop默认不会开启，需要手动启用  

#### Imput Source & Timer Source
这两个都是Runloop事件的来源  
- Input Source：  
  1. Prot-Based Sources，系统底层的port事件，如CFSocketRef，在应用底层基本用不到  
  2. Custom Input Sources，用户手动创建的Source  
  3. Cocoa Perform Selector Sources，Cocoa提供的performSelector系列方法，也是一种事件源  
- Timer Source 定时器事件

#### Runloop Observer
Runloop通过监控Source来决定有没有任务要做，除此之外，我们可以用Runloop Observer来监控Runloop本身的状态  
Runloop Observer可以监控下面的事件：  
- The entrance to the run loop  
- When the run loop is about to process a timer  
- When the run loop is about to process an input source   
- When the run loop is about to go to sleep  
- When the run loop has woken up, but before is has processed the event that woke it up  
- The exit for the run loop  

#### Runloop Mode
在监视与被监视中，Runloop要处理的事情还挺复杂的，为了让Runloop能专心处理自己关心的那部分事情，引入Runloop Mode概念  
![mode](http://cc.cocimg.com/api/uploads/20150528/1432798883604537.png)  
如上，Runloop Mode实际上是Source, Timer, Observer的集合，不同的Mode把不同组的Source, Timer, Observer隔绝开来，Runloop在某一时刻只能跑在在一个Mode，处理这Mode中的Source, Timer, Observer  
五个Mode分别是:  
- NSDefaultLoopMode  (公开)  
- NSConnectionReplyMode  
- NSModalPanelLoopMode  
- NSEventTrackingRunLoopMode  
- NSRunLoopCommonModes  (公开)    

#### Runloop相关的坑
日常中，与Runloop接触最近的可能就是通过NSTimer了，一个Timer一次只能加入到一个Runloop中。通常就是加入到当前的Runloop的default mode中，而ScrollView在用户滑动时，主线程RunLoop会转到UITrackingRunLoopMode。而这个时候，Timer就不会运行  

#### 错误代码
- 阻塞主线程  
- 将UI操作放在非主线程
  ```objective-c
  for(int i=0; i<10; i++) {
        self.progressView.progress = (CGFloat)(i+1) / 10;
        sleep(1);
    }
    // 为什么不起作用
  ```

#### 常用方法
- dispatch_async  
  将block交给制定的线程处理  

- dispatch_group_async  
  监听一组任务是否完成，完成后通知执行其他操作。比如，执行三个下载任务，都结束后，通知界面  

- dispatch_apply  
  执行某个代码N次

- dispatch_barrier_async  
  前面的任务执行后，它才执行  
  ```objective-c
  dispatch_queue_t queue = dispatch_queue_create("gcdtest", DISPATCH_QUEUE_CONCURRENT);

    dispatch_async(queue, ^{
        [NSThread sleepForTimeInterval:2];
        NSLog(@"dispatch_async1");
    });

    dispatch_async(queue, ^{
        [NSThread sleepForTimeInterval:4];
        NSLog(@"dispatch_async2");
    });

    dispatch_async(queue, ^{
        NSLog(@"dispatch_barrier_async");
        [NSThread sleepForTimeInterval:4];
    });

    dispatch_async(queue, ^{
        [NSThread sleepForTimeInterval:1];
        NSLog(@"dispatch_async3");
    });
    /* 输出
    dispatch_async1
    dispatch_async2
    dispatch_barrier_async
    dispatch_async3
    */

  ```

#### Dispatch Queue
##### 创建dispatch Queue  
`dispatch_queue_create`用于创建queue，两个参数分别是queue名和一组queue属性  
```objective-c
// 串行队列
dispatch_queue_t queue = dispatch_queue_create("", NULL);
dispatch_queue_t queue = dispatch_queue_create("test.xx.testQueue", DISPATCH_QUEUE_SERIAL)

// 并行队列
dispatch_queue_t queue3 = dispatch_queue_create("test.xx.testQueue", DISPATCH_QUEUE_CONCURRENT);
```
> dispatch_get_current+queue 函数作为调试用途，或者测试当前queue的标识  
> 在block中调用这个函数，会返回block提交到的queue

##### 运行时获得公共Queue
- 使用`dispatch_get_main_queue`函数获得应用主线程关联的*串行*dispatch queue  
- 使用`dispatch_get_global_queue`来获得共享的*并发*queue，优先级从低到高分别是  
  - DISPATCH_QUEUE_PRIORITY_LOW  
  - DISPATCH_QUEUE_PRIORITY_DEFAULT  
  - DISPATCH_QUEUE_PRIORITY_HIGH  

##### 在Queue中存储自定义的上下文信息
`dispatch_set_context` & `dispatch_get_context`函数来设置和获取对象的上下文数据  

##### 添加任务到Queue
将任务dispatch到一个适当的dispatch queue，可以同步或异步的dispatch一个任务，也可以单个或按组(group)来dispatch  
- 添加单个任务到queue  
  尽可能使用`dispatch_async`或`dispatch_async_f`函数异步的dispatch任务。因为添加任务到Queue中时，无法确定这些代码什么时候执行。因此异步的添加block或函数可以让你立刻调用这些代码执行，然后调用线程可以执行其他  
  > 某些情况，可能希望同步的dispatch任务，以避免竞争条件或其他错误。`dispatch_sync`和`dispatch_sync_f`函数同步的添加任务到queue，这两个函数会堵塞当前进程，知道相应任务完成执行  

  ```objective-c
  dispatch_queue_t myCustomQueue = dispatch_queue_create("test.xx.MyCustomQueue", NULL);

  dispatch_async(myCustomQueue, ^{
      [NSThread sleepForTimeInterval:4];
      NSLog(@"Do some work here!");
  });

  NSLog(@"The first block may or may not have run.");

  // 相当于直接顺序打印一句话，根本不需要加队列
  dispatch_sync(myCustomQueue, ^{
      NSLog(@"Do some more work here");
  });
  // 上面的同步操作会堵塞当前进程，所以肯定是最后执行的
  NSLog(@"Both blocks have completed");
  ```
- 任务完成时执行Completion Block  
传统异步变成模型中，可能会用到回调机制，但是，Dispatch Queue可以用Completion Block   
Completion Block是你dispatch到queue的另一段代码，在原始任务完成时自动执行。  
```objective-c
dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
    for(int index=0; index<10; ++index) {
        sleep(1);
        dispatch_async(dispatch_get_main_queue(), ^{
            NSLog(@"index - %d", index);
            self.progressView.progress = (CGFloat)(index + 1) / 10;
        });
    };
});
```

##### 并发的执行Loop Iteration
如果使用循环执行固定次数的迭代，并发dispatch queue可能会提高性能  
当然也是有使用条件的：  
每次迭代执行的任务与其他迭代独立无关，而且迭代执行顺序也是无关紧要的话，可以使用`dispatch_apply`或`dispatch_apply_f`函数来替代循环  
> 注意：`dispatch_apply`和`dispatch_apply_f`会堵塞当前线程，而且和普通循环一样，也是在所有迭代完成后才返回  
> 所以如果你当前传递的是串行queue，而且正是执行当前代码的queue，就会产生死锁。主线程调用这两个函数必须小心，可能阻止事件处理循环无法响应用户事件  

##### 挂起和继续执行queue
`dispatch_suspend`函数，增加挂起计数  
`dispatch_resume`函数，减少挂起计数  
当挂起计数 > 0，挂起，否则继续  
挂起和继续是异步的，而且只在执行block之间生效，挂起一个queue不会导致正在执行的block停止  

##### Dispatch Queue和线程安全性  
- Dispatch Queue本身是线程安全的。换句话说，可以在应用的任意线程中提交任务到dispatch queue，不需要使用锁或者其他机制  
- 不要在执行任务代码中调用`dispatch_sync`函数调度相同的queue，这样做会死锁这个queue。如果需要dispatch到当前queue，需要使用`dispatch_async`函数异步调度  
- 避免在提交到dispatch queue的任务中获得锁，虽然在任务中使用锁是安全的，但在请求锁时，如果锁不可用，可能会完全堵塞串行queue。类似的，并发queue等待锁也可能阻止其他任务的执行。如果代码需要同步，就使用串行的dispatch queue  
- 虽然可以获得运行任务的底层线程的信息，最好不要这样做  

##### dispatch Semaphore
类似于传统的semaphore(信号量)，但是更加高效。只有当调用小城由于信号量不可用，需要堵塞时，Dispatch Semaphore才会去调用内核。如果信号量可用，就不会与内核进行交互  
使用信号量，可以实现对有限资源的访问控制  
- 使用Dispatch Semaphore控制有限资源的使用  
如果提交到Dispatch Queue中的任务需要访问某些有限资源，可以使用Dispatch Semaphore来控制同时访问这个资源的任务数量。Dispatch Semaphore和普通的信号量类似，唯一的区别是当资源可用时，需要更少的时间来获得Dispatch Semaphore  
使用Dispatch Semaphore的过程如下：  
1. 使用`dispatch_semaphore_create`函数创建semaphore，制定正数值标识资源的可用数量  
2. 在每个任务中，调用`dispatch_semaphore_wait`来等待semaphore  
3. 当上面的调用返回时，获得资源并开始工作  
4. 使用完资源后，调用`dispatch_semaphore_signal`函数释放和signal这个semaphore  
```objective-c
// 创建信号，制定初始池大小
dispatch_semaphore_t sema = dispatch_semaphore_create(getdtablesize() / 2);

//等待一个可用的文件描述符
dispatch_semaphore_wait(sema, DISPATCH_TIME_FOREVER);
// code 这里写代码获取文件，并对其操作

// 完成后释放文件描述符
// code 这里写代码关闭文件
dispatch_semaphore_signal(sema);
```

##### Dispatch Group
用于监控一组Block对象完成(可以同步或异步的监控block)
Dispatch Group用来阻塞一个线程，知道一个或多个任务完成执行。  
基本的流程是设置一个组，dispatch任务到queue，然后等待结束。需要使用`dispatch_group_async`函数，会关联任务到相关的组和queue。使用`dispatch_group_wait`等待一组任务完成  
```objective-c
dispatch_group_t group = dispatch_group_create();
dispatch_queue_t queue = dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0);

dispatch_group_async(group, queue, ^{
    [NSThread sleepForTimeInterval:1];
    NSLog(@"q1");
});

dispatch_group_async(group, queue, ^{
    [NSThread sleepForTimeInterval:2];
    NSLog(@"q2");
});

NSLog(@"waiting....");
// 阻塞当前线程等待group完成
dispatch_group_wait(group, DISPATCH_TIME_FOREVER);
NSLog(@"finished");
```

##### Dispatch Barrier
解决资源争夺问题有三种方案：  
- 枷锁  
- 使用异步执行串行队列的方式  
- dispatch_barrier  
`dispatch_barrier-async`往队列里发送任务快，这个任务有栅栏(barrier)作用。  
barrier必须单独执行，只在并发队列中有意义。如果发现接下来要执行的block是一个barrier block，*那么久一直要等到当前所有并发的block都执行完，才会单独执行这个barrier block代码块* 然后继续其他并发block  
通常对于一个对象的并发读写问题，可以同时读，但是不能同时写，所以可以把写放入`dispatch_barrier_async`中  

###### Dispatch Source
在特定的类型的系统事件发生时，会产生通知，可以使用Dispatch source 来监控各种事件，如：进程通知，信号，描述符事件等，当事件发生时，dispatch source异步的提交你的任务到指定的dispatch queue来进行处理  

现在系统通常提供异步接口，允许应用向系统提交请求，然后在系统处理请求时，应用可以继续处理自己的事情，GCD正是基于这个基本行为而设计的，允许你提交请求，并通过block和dispatch queue报告结果  
dispatch source是基础数据类型，协调特定底层系统事件的处理  
GCD支持以下Dispatch Source:  
- Timer Dispatch Source ： 定期产生通知  
- Signal Dispatch Source ： UNIX信号到达时产生通知  
- Descriptor Dispatch Source ： 各种文件和socket操作的通知  
- 文件可读  
- 文件可写  
- 文件在文件系统中被删除、移动、重命名、文件元数据信息修改  
- Process Dispatch Source ： 进程相关的事件通知  
- 当进程退出时  
- 当进程发起fork或exec等调用  
- 信号被递送到进程  
- Mach port Dispatch Source ： mach相关事件的通知  
- Custom Dispatch Source ： 自定义并触发  

Dispatch Source替代了异步回调函数，来处理系统相关的事件，当你配置一个dispatch source时，你指定要检测的事件、dispatch queue、以及处理事件的代码(block或函数)。当事件发生时，dispatch source会提交你的block或函数到指定的queue去执行。和手工提交到queue不同，dispatch source为应用提供连续的事件源。除非显式取消，否则dispatch source会一直保留与dispatch queue的关联。只要相应的事件发生，就会提交关联的代码到dispatch queue执行  

为了防止事件积压到dispatch queue，Dispatch source实现了事件合并机制。如果新事件在上一个事件处理器出列并执行之前到达，dispatch source会将新旧事件的数据合并。根据事件类型的不同，合并操作可能会替换旧事件，或者更新旧事件的信息  

##### 创建Dispatch Source
需要同时创建事件源和dispatch source本身。事件源是处理事件所需要的native数据结构，例如基于描述符的Dispatch Source，需要打开描述符；基于进程的事件，需要获得目标程序的进程ID  

创建Dispatch Source:  
1. 使用`dispatch_source_create`创建Dispatch Source  
2. 配置Dispatch source  
  - 为dispatch source设置一个事件处理器  
  - 对于定时器源，使用`dispatch_source_set_timer`函数设置定时器信息  
3. 为dispatch source赋予一个取消处理器(可选)  
4. 调用`dispatch_resume`函数开始处理事件   

由于Dispatch Source必须进行额外的配置才能被使用，`dispatch_source_create`函数返回的dispatch source将处于挂起状态。此时dispatch source会接收事件，但是不会处理。这时候可以安装事件处理器，并执行额外配置  

##### 编写和安装一个事件处理器
需要定义一个事件处理器来处理事件，可以是函数或者block对象，并使用`dispatch_source_event_handler`或`dispatch_source_event_handler_f`安装事件处理器。事件到达时，Dispatch Source会提交你的事件处理器到指定的Dispatch queue，由queue执行事件处理器  

如果事件处理器已经开始执行，一个或多个新事件到达，Dispatch Source会保留这些事件，直到前面的事件处理器完成执行。然后以新事件再次提交处理器到queue  

函数事件处理器有一个context指针指向Dispatch Source对象，没有返回值，Block事件处理器没有参数，也没有返回值  
```objective-c
// 基于block的事件处理
void (^dispatch_block_t)(void)

// 基于函数的事件处理
void (* dispatch_function_t)(void * )
```
在事件处理器中，可以从dispatch source中获得事件的信息，函数处理器可以直接使用参数指针，block则必须自己捕获到dispatch source指针，一般block定义时会自动捕获外部定义的所有变量   
```objective-c
// 创建dispatch_queue_t
dispatch_queue_t myQueue = dispatch_queue_create("myQueue", DISPATCH_QUEUE_SERIAL);

// 创建dispatch_source_t
dispatch_source_t source = dispatch_source_create(DISPATCH_SOURCE_TYPE_READ, "myDescriptor", 0, myQueue);

dispatch_source_set_event_handler(source, ^{
    // 从上下文中获取信息
    size_t estimated = dispatch_source_get_data(source);
});

dispatch_resume(source);
```
block捕获外部变量允许更大的灵活性和动态性。  
下面是事件处理器能够获得的事件信息：  
- dispatch_source_get_handle   
  返回dispatch source管理的底层系统数据类型  
- dispatch_source_get_data  
  返回事件管理的所有未决数据  
- dispatch_source_get_mask  
  返回用来创建dispatch source的事件标志  





## GCD
#### 什么是GCD(Grand Central Dispatch)
为并发代码在多核硬件(iOS, OSX)上执行提供有力支持。  

#### GCD术语
- Serial & Concurrent           串行与并发  
- Synchronous & Asynchronous    同步与异步  
- Critical Section              临界区
  两个进程不能执行同一段代码  
- Race Condition                竞态条件  
  特定序列或时机的软件系统以不受控制的方式运行的行为，例如程序的并发任务执行的确切顺序   
- Deadlock                      死锁  
  一和二互相等待对方执行完成，结果就赵成了死锁  
- Thread Safe                   线程安全  
  线程安全的代码能够在多线程或者并发任务中被安全调用，而不是导致问题(数据损坏，崩溃等)。线程不安全的代码在某个时刻只能在一个上下文中运行。NSDictionary(安全)， NSMutableDictionary(不安全)  
- Context Switch                上下文切换  
  当你在单个线程里切换执行不同的线程时，存储与恢复执行状态的过程，这个过程在编写多任务应用时很普遍，但会带来一些额外的开销  
- Concurrency & Parallenlism    并发与并行  
  并发代码的不同部分可以“同步”执行，然而，如何发生取决于系统。  
  多核设备通过并行来同时执行多个线程  
  单核设备比较坑，必须先运行一个线程，执行一个上下文切换，然后运行另一个线程或者进程  
- Queues                        队列   
  GCD提供`dispatch queues`来处理代码块，这些队列管理你提供给GCD的任务，并用FIFO顺序执行这些任务  
  所有的调度队列，自身都是线程安全的，可以从多个线程并行的访问他们。当了解了调度队列如何为你自己代码的不同部分提供线程安全后，GCD的优点就显而易见了。这一点的关键是选择正确的-类型-的调度队列和正确的-调度函数-来提交工作  
  - Serial Queues 串型队列  ： 一次执行一个任务，而且一个Block结束后下一个Block开始之间的间隔不确定，主要用于对特定资源的同步访问    
  - Concurrency Queues  并发队列 ： 并发队列中，唯一能保证的是他们会按照被添加的顺序开始执行，任务可能以任意顺序完成，你不会知道何时开始下一个任务，或者任意时刻有多个Block在运行。  
  - 主队列 ： 全局可用的串行queue，在应用主线程中执行，这个queue与应用的RunLoop交叉执行，通常用于应用的关键同步点  
- Queue Types                    队列类型  
  系统提供了一个主队列(`main queue`)的特殊队列。和其他串型队列一样，这个队列中的任务一次只能执行一个。然而它能保证所有的任务都在主线程运行。而主线程是*唯一*可用于更新UI的线程。这个队列就是用于发生消息给`UIView`或发送通知  
  系统还提供了几个并发队列：他们叫做全局调度队列(`Global Dispatch Queues`)，这四个队列有着不同的优先级：`background`, `low`, `default` 以及`high`。而且Apple的API也会调用这些队列   
