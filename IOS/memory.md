# iOS 内存相关  
## iOS 内存管理机制  
Object-C 在iOS中不支持GC机制，而是采用引用计数的方式管理内存  
> 引用计数(Reference Count):  
> 在引用计数中，每一个对象负责维护对象所有引用的计数值。当一个新的引用指向对象时，引用计数就递增，当去掉一个引用时，引用计数就递减。当引用计数到0时，改对象就将释放占有的资源  

### MRC(Manual Reference Counting)中引起引用计数变化的方法  

Object-C对象方法              | 说明
---------------------------- | ----
alloc/new/copy/multableCopy  | 创建对象，引用数 +1
retain                       | 引用计数 +1
release                      | 引用计数 -1
dealloc                      | 当引用计数位0时调用  
[NSArray array]              | 引用数不增加，由自动释放池管理
[NSDictionary dictionary]    | 引用数不增加，由自动释放池管理

### ARC(Automatic Reference Counting)中的内存管理  

Objective-C 对象所有权修饰符  | 说明   
`__strong`                  | 对象默认修饰符，对象强引用，在对象超出作用域时失效，其实就相当于retain操作，超出作用域时执行`release`操作  
`__weak`                    | 弱引用，不持有对象，对象释放时会将对象置nil
`__unsafe_unretained`       | 弱引用，不持有对象，对象释放时不会将对象置nil  
`__autoreleasing`           | 自动释放，自动释放池管理对象  





# [内存相关](http://blog.devtang.com/2016/07/30/ios-memory-management/)
## 什么是引用计数
引用计数(Reference Count)是一个简单而有效的管理对象生命周期的方式。  
- 创建一个新的对象时，它的引用计数为1  
- 当有一个新的指针指向这个对象时，引用计数 +1  
- 当这个对象不再指向这个对象时，引用计数 -1  
- 当对象的引用计数为0时，说明这个对象不再被任何指针指向，可以销毁回收了  
![picture](http://blog.devtang.com/images/memory-ref-count.png)
```Object-C
NSObject *object = [[NSObject alloc] init];
NSLog(@"Reference Count = %lu", [object retainCount]);

NSObject *another = [object retain];
NSLog(@"Reference Count = %lu", [object retainCount]);
[another release];

NSLog(@"Reference Count = %lu", [object retainCount]);

[object release];
NSLog(@"Reference Count = %lu", [object retainCount]);

/*
 * 输出为：
 * MyBlock[8403:228149] Reference Count = 1
 * MyBlock[8403:228149] Reference Count = 2
 * MyBlock[8403:228149] Reference Count = 1
 * MyBlock[8403:228149] Reference Count = 1
 */
```

### 为什么需要引用计数
真实场景下，函数内使用一个临时的对象是不需要修改它的引用计数的，因为对象的生命周期只是在一个函数内，只需要在函数返回前将该对象销毁即可  
引用计数真正的使用场景是在面向对象的程序设计架构中，用于对象之间传递和共享数据。  
举个栗子：  
加入对象A生成了一个对象M，需要调用对象B的某一个方法，将对象M作为参数传递过去。  
在没有引用计数的情况下，一般内存管理原则是“谁申请，谁释放”，那么对象A就需要在对象B不再需要对象M的时候，将对象M销毁。但对象B可能只是临时用一下对象M，也可能觉得对象M很重要，将它设置成自己的一个成员变量，那这种情况下，什么时候销毁对象M就成了一个难题  
- 一个暴力的做法是，对象A在调用完对象B之后，马上就销毁对象M，然后对象B需要将参数另外复制一份，生成一个对象M2，然后自己管理M2的生命周期。这种方法带来的问题是，带来了更多的内存申请、复制、释放的工作。本来一个可以复用的对象，因为不方便管理声明周期，就简单的把它销毁，又重新构造一份一样的，影响性能  
  ![copy](http://blog.devtang.com/images/memory-talk-2.png)  
- 另一种做法是，对象A在构造完对象M之后，始终不销毁对象M，由对象B来完成对象M的销毁工作。这种做法强烈依赖于AB两个对象的配合，而且声明和释放分散在不同对象中，比较复杂，特别是再来个C对象啥的。  
  ![77](http://blog.devtang.com/images/memory-talk-3.png)  
- 所以，引用计数很好的解决了这个问题  

### 针对上面代码最后一个输出为什么是1而不是0
该对象的内存已经被回收，向一个已经被回收的对象发一个retainCount消息，它的输出结果应该是不确定的，如果该对象所占的内存被复用了，很可能造成程序异常崩溃   
那么为什么是1而不是其他的呢，因为当最后一个执行`release`时，系统知道就要回收内存了，就没有必要再将`retainCount`-1了，对象被回收后，它的所有的内存区域，包括`retainCount`都没有意义了，不将1变成0，可以减少一次内存操作  

### ARC下的内存管理问题
ARC能解决90%的内存管理问题，剩下的10%需要自己动手。这主要就是与底层Core Foundation对象交互的那部分，底层的Core Foundation对象由于不在ARC的管理下，所以需要自己维护这些对象的引用计数  

#### 循环引用(Reference Cycle)问题
引用计数有个问题就是不能解决循环引用问题。  
对象A和对象B，相互引用了对方作为自己的成员变量，只有当自己销毁时，才会将陈冠变量的引用计数-1，因为对象A的销毁依赖于对象B销毁，而对象B的销毁依赖于对象A的销毁，这样就造成了循环引用  

##### 主动断开循环引用
这种方式常见于各种与block相关的代码逻辑中。  
```Object-C
// https://github.com/yuantiku/YTKNetwork/blob/master/YTKNetwork/YTKBaseRequest.m
// 第 147 行：
- (void)clearCompletionBlock {
  // 主动释放掉对于 block 的引用
  self.successCompletionBlock = nil;
  self.failureCompletionBlock = nil;
}
```
不过不常用，更多的是下面的弱引用  

##### 弱引用
弱引用虽然持有对象，但是不增加引用计数，这样就避免了循环引用的产生。弱引用通常在delegate模式中使用。  
如两个ViewController A和B，A需要弹出B，让用户输入一些内容，当用户完成后，B将内容返回给A，这时候，ViewController的delegate成员变量通常是一个弱引用，避免两个ViewController相互引用对方造成循环引用问题  
![weak delegate](http://blog.devtang.com/images/memory-cycle-4.png)  

###### 弱引用的实现原理
系统对于每一个有弱引用的对象，都维护了一个表来记录它所有的弱引用的指针地址。这样当一个对象的引用计数为0时，系统就通过这张表，找到所有的弱引用指针，继而把他们都置为nil   
所以，弱引用是有额外开销的

#### 使用Xcode检测循环引用
Xcode的Instruments工具集可以很方便的检测循环引用

#### Core Foundation对象的内存管理
底层的Core Foundation对象，在创建时大多以xxxCreateWithxxx的方式  
```Object-C
// 创建一个CFStringRef对象
CFStringRef str = CFStringCreateWithCString(kCFAllocationDefault, "hello world", kCFStringEncodingUTF8);

// 创建一个 CTFontRef对象
CTFontRef fontRef = CTFontCreateWithName((CFStringRef)@"ArialMT", fontSize, NULL);
```
对于这些对象的引用计数修改，要使用`CFRetain`和`CFRelease`  
```Object-C
// 创建一个 CTFontRef对象
CTFontRef fontRef = CTFontCreateWithName((CFStringRef)@"ArialMT", fontSize, NULL);

// 引用计数 +1
CFRetain(fontRef);
// 引用计数 -1
CFRelease(fontRef);
```
`CFRetain`和`CFRelease`可以类比`retain`和`release`  
在ARC下，有时需要将一个Core Foundation对象转换成一个Objective-C对象，这时候就需要告诉编译器，转换过程中，引用计数需要做如何的调整。这就引入了`bridge`关键字，一下是这些关键字的说明：  
- `__bridge`：只做类型转换，不修改相关对象的引用计数，原来的Core Foundation对象在不用时，需要调用`CFRelease`方法  
- `__bridge_retained`：类型转换后，将相关对象的引用+1，原来的Core Foundation对象在不用时，需要调用`CFRelease`方法  
- `__bridge_transfer`：类型转换后，将该对象的引用计数交给ARC管理，Core Foundation对象在不用时，不再需要调用CFRelease方法  
