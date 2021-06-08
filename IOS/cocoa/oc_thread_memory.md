# Objective-C高级编程 iOS与OX X多线程和内存管理
## 自动引用计数
### 1.1什么是自动引用计数
自动引用计数(ARC, Automatic Reference Counting)是指内存管理中堆引用采取自动计数的技术  

### 1.2 内存管理/引用计数
#### 1.2.2 内存管理的思考方式
很容易将注意力放到计数上，但是，正确的思考方式是：  
- 自己生成的对象，自己所持有  
- 非自己生成的对象，自己也能持有  
- 不再需要自己持有的对象时释放  
- 非自己持有的对象无法释放  

对象操作      | OC方法  |
------------ | ------ |
生成并持有对象 | alloc/new/copy/mutableCopy等方法 |
------------ | -------|
持有对象      |  retain |
------------ | -------|
释放对象      | release |
------------ | -------|
废弃对象      | dealloc |

> Cocoa框架中Foundation框架类库的NSObject类担负内存管理的职责，Objective-C 内存管理中的alloc/retain/release/dealloc方法分别指代NSObject类的alloc类方法, retain实例方法, dealloc实例方法  

##### 自己生成的对象，自己所持有
使用以下名称开头的方法名，以为着自己生成的对象只有自己持有  
- alloc  
- new  
- copy  
- mutableCopy  
> ”自己“可以理解为“对象的使用环境”，但是理解为编程人员“自身”也是可以的  

> `copy`方法利用基于`NSCopying`方法约定，由各类实现的`copyWithZone`方法生成并持有对象的副本。与`copy`方法类似，`mutableCopy`方法利用基于`NSMutabelCopying`方法约定，由各类实现的`mutableCopyWithZone`方法生成并持有对象的副本，却别是一个可变一个不可变  

##### 非自己生成的对象，自己也能持有  
```Objective-C
// 取得非自己生成并持有的对象
id obj = [NSMutableArray array];
// 取得的对象存在，但自己不持有对象  
```
> 代码中，`NSMutableArray`类对象被赋给变量obj，但是obj自己并不持有该对象，使用`retain`方法可以持有对象  

```Objective-C
// 取得非自己生成并持有的对象
id obj = [NSMutableArray array];
// 取得的对象存在，但自己不持有对象
[obj retain];
// 自己持有对象
```
> 通过`retain`方法，非自己生成的对象跟用alloc/new/copy/mutableCopy方法生成并持有的对象一样，成为了自己所持有的  

##### 不再需要自己持有的对象时释放
自己持有的对象，一旦不再需要，持有者有义务释放该对象。释放使用`release`方法  
```Objective-C
[obj release];
// 指向对象的指针仍然被保存在变量obj中，貌似能够访问，但对象一经释放，绝对不可访问  
```

##### 无法释放非自己持有的对象
对于用alloc/new/copy/mutableCopy方法生成并持有的对象，或者用retain方法持有的对象，由于持有者是自己，所以不需要该对象时需要将其释放。而由此以外所得到的对象绝对不能释放，倘若在应用程序中释放了非自己所持有的对象，就会造成崩溃。  

##### autorelease
看上去像ARC，但是实际上更像C语言中的自动变量(局部变量)的特性  
> C中的自动变量：程序执行时，若某自动变量超出其作用域，该自动变量将被自动废弃  
> `{ int a; } // 外面就超出了作用域`  

与C中自动变量不同的是，编程人员可以设定变量的作用域：  
1. 生成并持有NSAutoreleasePool对象  
2. 调用已分配对象的autorelease实例方法  
3. 废弃NSAutoreleasePool对象  
```Objective-C
NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];
id obj = [[NSObject alloc] init];
[obj release];
[pool drain];  // 等同于[pool release];
```
在Cocoa框架中，相当于程序主循环的`NSRunLoop`或者在其他程序可运行的地方，对`NSAutoreleasePool`对象进行生成，持有和废弃处理。因此，应用程序开发者不一定非得使用NSAutoreleasePool对象来进行开发工作
