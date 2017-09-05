# RunLoop
## RunLoop的概念
如何让一个线程执行完一个任务后不退出？ Event Loop!（如Node.js的事件处理）  
RunLoop实际上是一个对象，这个对象管理了其需要处理的事件和消息，并提供了一个入口函数来执行上面的Event Loop逻辑。  
线程执行完这个逻辑后，就会一直处于这个函数内部：“接收消息 -> 等待 -> 处理”的循环中，知道这个循环结束，函数返回。   
OSX/iOS系统中，提供了两个对象：`NSRunLoop` 和 `CGRunLoopRef`  
`CGRunLoopRef`基于`CoreFoundation`框架，提供纯C的函数API，线程安全  
`NSRunloop`是基于`CGRunloopRef`的封装，提供了面向对象的API，但是这些API不是线程安全  

#### RunLoop与线程的关系
iOS中能遇到的两个线程对象：`pthread_t`和`NSThread`  
这两个对象是一一对应的  
```objective-c
// 获取主线程
pthread_main_np();
// 对应于NSThread中的
pthread_self();
// 或
[NSThread currentThread];
```
> CGRunloop是基于pthread来管理的

RunLoop是不能创建的，只能通过`CFRunLoopGetMain()`和`CFRunLoopGetCurrent()`这两个函数自动获取  
> 线程刚创建的时候是没有RunLoop的，如果不主动获取，那么一直都不会有。RunLoop的创建是发生在第一次获取时，销毁是在线程结束时。  


## RunLoop对外的接口
在CoreFoundation中，关于RunLoop的5个类：  
- CFRunLoopRef  
- CFRunLoopModeRef  
- CFRunLoopSourceRef  
- CFRunLoopTimerRef  
- CFRunLoopObserverRef  

#### CFRunLoopRef  & CFRunLoopModeRef  
其中，`CFRunLoopModeRef`没有对外暴露，只是通过CFRunLoopRef的接口进行了封装，他们的关系如下：  
![RunLoop](http://cc.cocimg.com/api/uploads/20150528/1432798883604537.png)  
一个RunLoop包含若干个Mode，每个Mode又包含若干个Source/Timer/Observer。每次调用RunLoop的主函数时，只能指定其中一个Mode，这个Mode被称为`CurrentMode`。如果要切换Mode，只能退出Loop，在重新指定一个Mode进入。这样做主要是为了分隔开不同组的Source/Timer/Observer，让其互不影响  

#### CFRunLoopSourceRef
是事件产生的地方。Source有两个版本：Source0 和 Source1  
- Source0 只包含一个回调(函数指针)，它并不能主动触发事件。使用时需要先调用`CFRunLoopSourceSignal(source)`，将这个Source标记为待处理，然后手动调用`CFRunLoopWakeUp(runloop)`来唤醒RunLoop，让其处理这个事件  
- Source1 包含了一个`mach_prot`和一个回调(函数指针)，被用于通过内核和其他线程相互发送信息。这种Source能主动唤醒RunLoop线程  

#### CFRunLoopTimerRef  
是基于时间的触发器，它和NSTimer是toll-free bridged的，可以混用。其包含一个时间长度和一个回调(函数指针)。当其加入到RunLoop时，RunLoop会注册对应的时间点，当时间点到时，RunLoop会被唤醒以执行那个回调  

#### CFRunLoopObserverRef
是观察者，每个Observer都包含了一个回调(函数指针)，当RunLoop的状态发生变化时，观察者就能通过回调接收到这个变化，可以观测的时间点有以下几个：  
```objective-c
typedef CF_OPTIONS(CFOptionFlags, CFRunLoopActivity) {
  kCFRunLoopEntry         = (1UL << 0), // 即将进入Loop
  kCFRunLoopBeforeTimers  = (1UL << 1),  // 即将处理 Timer
  kCFRunLoopBeforeSources = (1UL << 2),  // 即将处理 Source
  kCFRunLoopBeforeWaiting = (1UL << 5),  // 即将进入休眠
  kCFRunLoopAfterWaiting  = (1UL << 6),  // 刚从休息中唤醒
  kCFRunLoopExit          = (1UL << 7),  // 即将退出 Loop
}
```
> 上面的 Source/Timer/Observer被统称为mode item，一个item可以被同时加入多个mode。但一个item被重复加入同一个mode时是不会有效果的。如果一个mode中一个item都没有，则RunLoop会直接退出，不进入循环  



## RunLoop的Mode

## RunLoop的内部逻辑

## RunLOOP的底层实现

## 苹果用RunLoop实现的功能
### AutoReleasePool

### 事件响应

### 手势识别

### 界面更新

### 定时器

### PerformSelector

### 关于GCD

### 关于网络请求

## RunLoop的实际应用举例
### AFNetworking

### AsyncDisplayKit
