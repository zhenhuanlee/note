# 通知
#### 适用场景
1. 从后一个控制器向前一个控制器传递某个信息的时候(前到后用delegate, block)  
2. 视图和控制器之间传递信息  
3. 相隔多个界面传递信息  

#### 通知原理  
基本通知有三步：post发送通知  ->  NSNotificationCenter通知中心处理通知  -> addobserve接收通知  
不论发送还是通知都只有**通知中心**这一条路  
> 通知中心是一个单例类，管理系统的所有通知事件，不论是手动发出的还是系统的通知，在创建通知的时候有两种情况  
> 1. 只传递动作，不传递具体的信息内容详情  
>   `- (void) postNotificationName:(NSString *)aName object:(nullable id)anObject;`
> 2. 传递具体的信息内容，用字典传递  
>   `- (void) postNotificationName:(NSString *)aName object:(nullable id)anObject userInfo:(nullable NSDictionary)aUserInfo;`

一个完整的创建通知的demo  
```objective-c
// 发送一个通知，不带参数
NSNotificationCenter *center = [NSNotificationCenter defaultCenter];
[center postNotificationName:@"colorChange" object:nil];
// 上面两句等价于下面这个
[[NSNotificationCenter defaultCenter] postNotificationName:@"colorChange" object:nil];

// 发送一个通知，带参数
/*
 * 需要将待传的参数放到字典中
 * object这个参数的作用是再次进行细分通知
 * object:nil 接收方会接受所有名字为giveName的通知
 * object:@"ni" 接收方只接受名字为giveName且object正确的通知
 */
NSDictionary *dic = @{@"name": @"sun"};

[[NSNotificationCenter defaultCenter] postNotificationName:@"giveName" object:nil userinfo:dic];
```
这是创建一个不传值得通知"colorChange"，用于某个事件的执行  

> oc中的单例类有个特征，创建他们的不是`init`而是`standard...`或`default...`  

接收方只有一个方法  
```objective-c
- (void)addObserver:(id)observer selector:(SEL)aSelector name:(nullable NSString *)aName object:(nullable id)anObject;
```
demo:  
```objective-c
[[NSNotificationCenter defaultCenter] addObserver:self selector:@selector(changeColor) name:"colorChange" object:nil];

// 通知事件
- (void)changeColor {
  self.nextButton.backgroundColor = [UIColor yellowColor];
}

```
这是一个完整的接收通知的写法  
`addObserver: self`就是说实现这个通知的方法在本类中实现，也就是`- (void)changeColor`这个方法  
`selector:@selector(changeColor)`就是选择要实现这个通知的方法  
`name:@"colorChange"`这个名字必须和发送通知的名字一样  

传递值通知的demo  
```objective-c
// 接收通知
[[NSNotificationCenter defaultCenter] addObserver:self selector:@selector(giveName) name:@"giveName" object:nil]

// 通知事件
- (void)giveMyName:(NSNotification *)user
{
  NSDictionary *dic = user.userinfo;
  NSString *name = dic[@"name"];
  [self.nextButton setTitle:name forState:UIControlStateNormal];
}
```
> 取值的参数用的是`NSNotification`声明的，`NSNotification`这个类的作用就是为`NSNotificationCenter`服务的，NSNotification里面包含三个参数：  
> @property (readonly, copy)NSString *name;  
> @property(nullable, readonly, retain)id object;  
> @property(nullable, readonly, copy) NSDictionary *userinfo;  
> 没错，`NSNotificationCenter`发送接收通知方法里面的name,object,userinfo都是`NSNotification`的参数，可以通过`NSNotification`取得通知的所有信息  

注销通知：
```objective-c
// 释放内存有两个方法
- (void)removeObserver:(id)observer;

- (void)removeObserver:(id)observer name:(nullable NSString *)aName object:(nullabel id)anObject;
```
一般用第一个，把本控制器活着的视图的所有内存都在结束的时候释放  
```objective-c
// 释放通知
- (void)dealloc {
 [[NSNotificationCenter defaultCenter] removeObserver:self]; 
}
```

追加一个发送通知的方法  
```objective-c
- (void)postNotification:(NSNotification *)notification;
```
这个方法和上面两个大同小异，只是把notification拿出来单独定义  

demo:
```objective-c
// 要传递的参数
NSNotification *not = [[NSNotification alloc] initWithName:@"giveMessage" object:nil userinfo: nil];
// 发送通知
[[NSNotificationCenter defaultCenter] postNotification:not]
```

还有一个block相关的方法  
```objective-c
- (id)addObserverForName:(nullable NSString *)name object:(nullable id)obj queue:(nullable NSOperationQueue *)queue usingBlock:(void(^)(NSNotification *note))block NS_AVAILABLE(10_6,4_0);
```
后缀可以看出，是iOS4.0增加的方法。这是一个接收通知的方法，相对于另一个方法它最大的好处是通知时间的执行直接就在接收通知后面，不用再写一个方法去执行，优化代码。  
demo:  
```objective-c
// 接收通知
[[NSNotificationCenter defaultCenter]addObserverForName:@"giveMessage"
object:nil queue:nil usingBlock:^(NSNotification *not)] 
{
  [self.nextButton setTitle:@"block" forState:UIControlStateNormal];
}
```
这里，把queue线程设为了nil，也就是说，这个通知方法和发送通知只占一个线程  
`usingBlock:^(NSNotification *not)`后面这个`NSNotification`的声明必须要有

