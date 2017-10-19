# Objective-C 基础教程

### 第3章 面向对象编程基础知识
#### 3.4 Objective-C中的OOP
##### 3.4.1 @interface 部分  
`@interface`指令把该类的一些信息(如：对象的数据成员和它提供的特性)传递给编译器  
```Objective-C
@interface Circle : NSObject
{
  ShapeColor fillColor; // 实例变量
}

- (void) setFillColor: (ShapeColor)fillColor;  // 方法声明

@end // Circle
// @interface Circle 告诉编译器：这是名为Circle的新类定义的接口  
```
> 在OC中看到`@`符号，都可以把它看成是C语言的扩展  

##### 3.4.2 @implementation 部分
`@interface`一般用于定义公共接口，真正使对象起作用的代码位于`@implementation`中  
`@implementation`是一个编译器指令，表明将为某个类提供代码  
> 在OC中调用方法时，一个名为self的秘密参数将会被传递给接受对象，而这个参数引用的就是该接收对象  
> 如在`[circle setFillColor:kRedColir]`中`circle`将作为其`self`参数进行传递  
> `self`是秘密和自动的，方法中引用实例变量的代码如下：`self->fillColor = c;`

##### 3.5 小结
1. 间接(如变量和文件的使用)  
2. 面向过程：函数第一，数据第二  
3. 面向对象：数据第一，函数第二  
4. 每个方法的调用都包括一个名为`self`的隐藏参数，他是对象自身  

### 第4章 继承
##### 4.3.1 方法的调度
继承方法的调度过程： 向`Circle`对象发送`setFillColor:`消息时，调度程序首先询问`Circle`类是否使用自身的代码响应`setFillColor:`方法。在本例中，回答是不能，调度程序发现`Circle`类没有为`setFillColor:`定义方法，因此，它将在超类`Shape`中查找对应的方法。  

##### 4.3.2 实例变量
创建一个新类时，其对象首先从自身的超类中继承实例变量，然后(可选)添加他们自己的实例变量。  
每个方法调用都获得一个`self`的隐藏参数，它是一个指向接收消息的对象的指针。方法使用`self`参数查找他们要使用的实例变量  
因为继承和超类之间建立了一种'is a'关系，所以`NSObject`的实例变量称为`isa`  
> `self` 为什么会指向`isa`?   
> 因为一个类从`NSObject`开始继承，因此`self`的第一个实例变量指向`isa`   

基地址和偏移量  
> 编译器使用‘基地址加偏移量’机制实现奇妙的功能。给定的对象基地址是指，第一个实例变量的首个字节在内存中的位置。通过该地址加上偏移地址，编译器可以查找其他实例变量的位置  
> 如：圆角矩形对象的基地址是 0x1000，则isa实例变量的地址是 0x1000 + 0，即位于 0x1000 位置。isa的值占四个字节，因此，下一个失恋变量的起始地址位于4个偏移位置之后，即位于 0x1000 + 4。每个实例变量与对象的基地址都有一个偏移位置  
> 如果要访问`fillColor`实例变量，编译器生成代码并得到存储`self`的位置值，然后加上偏移量(本例为4)，得到指向存储变量值的位置  

### 第7章 深入了解Xcode
Xcode自动提示，名称旁边的彩色方框标识这个符号的类型：E表示枚举、f表示函数、#表示#define指令、m表示方法、C表示类  

### Foundation Kit快速教程
#### 8.1 一些有用的数据类型  
为什么`NSPoint`，`NSSize`是struct而不是对象->因为性能。GUI程序会用到许多临时的点、大小和矩形来完成工作。所有的OC对象都是动态分配的，而动态分配是一个代价较高的操作，会消耗大量的时间。  

#### 8.2 字符串
Object-C运行时生成一个类的时候，会创建一个代表该类的**类对象**。类对象包含了：  
- 指向超类的指针  
- 类名  
- 指向方法列表的指针  
- 一个long型数据，为新创建的类实例对象指定大小  
> 一般类方法都是用来创建实例的，我们称这种用来创建新对象的类方法为工厂方法  

##### 8.2.5 不区分大小写的比较
`compare:options:`options的参数是一个**位掩码(mask)** ，可以使用位或运算符(|)来添加选项标记  

#### 8.4 集合家族
NSArray是一个cocoa类，有两个限制：  
1. 只能存储OC对象，不能存储C中的基本数据类型，如int、float、enum、struct等  
2. 不能存储nil  

#### 8.5 各种数值
NSArray 和 NSDictionary只能存储对象，而不能直接存储任何基本类型数据。但是可以通过对象来封装基本数据  

##### 8.5.1 NSNumber
用来包装(即以对象的方式实现)基本数据类型  
可以使用下列方法创建新的NSNumber对象（装箱 - boxing）  
```Objective-C
+ (NSNumber * ) numberWithChar: (char)value;
+ (NSNumber * ) numberWithInt: (int)value;
+ (NSNumber * ) numberWIthFloat: (float)value;
+ (NSNumber * ) numberWithBool: (BOOL)value;
```
以下的方法可以重新获得（取消装箱 - unboxing）  
```Objective-C
- (char) charValue;
- (int) intValue;
- (float) floatValue;
- (BOOL) boolValue;
- (NSString * ) stringValue;
```

##### 8.5.2 NSValue
NSNumber实际上是NSValue的子类， NSValue可以包装任意值  
```Objective-C
+ (NSValue * ) valueWithBytes: (const void * )value objCType: (const char * )type
// cocoa提供了将常用struct数据转换成nsvalue的方法
+ (NSValue * ) valueWithPoint: (NSPoint) point;
+ (NSValue * ) valueWithSize: (NSSize) size;
+ (NSValue * ) valueWithRect: (NSRect) rect;
- (NSPoint)pointValue;
- (NSSize)sizeValue;
- (NSRect)rectValue;
```

##### 8.5.3 NSNull
Cocoa里最简单的类  
```Objective-C
+ (NSNull * )null;

[array addObject:[NSNull null]];
```
