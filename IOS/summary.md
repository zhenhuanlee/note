#### UIApplication
`UIApplication`对象是应用程序的象征，一个`UIApplication`对象就代表一个应用程序  
`UIApplication`对象可以进行一些应用级别的操作：
  1. 应用右上角显示红色的数字  
  2. 设置联网指示器的可见性  
  3. 管理状态栏  
#### UIApplication Delegate  
1. 简单说明：移动操作系统都会被打扰(如来电)，这时会产生一些系统事件，这时`UIApplication`会通知它的`delegate`对象，让`delegate`代理来处理这些系统事件  
iOS启动原理：  
![iOS启动原理](http://images.cnitblog.com/i/450136/201406/031950050525709.png)  
程序驱动完整过程：  
1. main函数  
2. UIApplicationMain  
  1. 创建`UIApplication`对象  
  2. 创建`UIApplication`的`delegate`对象  
3. 分两种情况  
  - 没有StoryBoard
    `delegate`对象开始处理(监听)系统事件  
    1. 程序启动完成会调用代理的`application:didFinishLaunchingWithOptions`方法  
    2. 在`application:didFinishLaunchingWithOptions`中创建`UIWindow`  
    3. 创建和设置`UIWindow`的`rootViewController`  
    4. 显示窗口  
  - 有StoryBoard   
    根据`Info.plist`获得最主要stroyBoard的文件名，加载最主要的stroyBoard  
    1. 创建`UIWindow`  
    2. 创建和设置`UIWindow`的`rootViewController`  
    3. 显示窗口  


#### UIWindow
`UIWindow`是一个特殊的`UIView`，通常一个app中至少有一个`UIWindow`  
iOS程序启动完毕后，创建的第一个视图就是`UIWindow`，接着创建控制器的view，最后将控制器的view加载`UIWindow`上，于是控制器的view就显示在屏幕上了  
没有`UIWindow`就看到不任何UI界面  
添加`UIView`到`UIWindow`的常用两种方式  
```
-(void) addSubView:(UIView *)view;
// 直接将view添加到UIWindow中，但不会理会view对应的UIViewController  

@property(nonatomic, retain) UIViewController *rootViewController;
// 自动将rootViewController的view添加到UIWindow中，负责管理rootViewController的生命周期  
```
常用方法：
- 让当前的`UIWindow`变成`keyWindow`(主窗口)  
  `-(void)makeKeyWindow;`  
- 让当前的`UIWindow`变成`KeyWindow`并显示出来  
  `-(void)makeKeyAndVisible;`  
- 在本应用中打开的`UIWindow`列表，这样就可以接触应用中的任何一个`UIView`对象  
  `[UIApplication sharedApplication].windows`  
- 用来接收键盘以及非接触类的消息事件的`UIWindow`，而且程序中每个时刻只能有一个`UIWindow`是`keyWindow`。如果某个`UIWindow`内部的文本框不能输入数字，可能是因为这个`UIWindow`不是`keyWindow`  
  `[UIApplication sharedApplication].keyWindow`  
- 获取某个`UIView`所在的`UIWindow`  
  `view.window`    

#### UIViewController  
控制器的创建  
- 了解`UIStoryBoard`对象，通过这个对象，就能加载`storyBoard`文件。注意：必须有`storyBoard`文件，创建`UIStoryBoard`对象才有意义，`alloc` `init`创建`UIStoryBoard`对象没有意义  
  - `instantiateInitialViewController` 默认加载箭头指向的控制器  
  - `instantiateViewControllerWithIdentifier` 根据标识在stroyBoard查找 控制器并且创建   
- 通过xib创建控制器的view(空项目)  
  - xib里面必须有一个view描述控制器的view，因为控制器的view属性必须有值  
  - xib需要制定描述哪一个控制器，描述UIView不需要
  - xib文件里可能有很多view，需要拖线致命哪个是控制器的view  

#### UIView





### App的启动过程
1. `Main`函数  
2. `UIApplicationMain`函数  
3. 初始化`UIApplication`（创建）
4. 设置`UIApplication`代理和相应的代理属性  
5. 开启事件循环，监听系统事件  
6. 检测`info.plist`文件，看看是否有`Main.StoryBoard`文件存在  
  如果有：  
    1. 加载`Main.StoryBoard`  
    2. 在`StoryBoard`上创建一个`UIWindow`  
    3. 设置`Window`的根控制器  
    4. 遍历控制器上面的所有子控件，没有则创建对应的控件  
  如果没有：
    1. 通过一个强引用创建`UIWindow`  `self.window = [[UIWindow alloc] init]`  
    2. 设置`Window`的`frame`为屏幕的`bounds`  `self.window.frame = [UIScreen mainScreen].bounds`  
    3. 设置`window`的根控制器  `self.window.rootViewController = [[UIViewController alloc] init]`  
    4. 将`window`作为主窗口并且显示在界面上   `self.window makeKeyAndVisible`  


### Cocoa
#### Cocoa是什么
- Cocoa程序是有一些对象组成，而这些对象都继承于他们的根类：NSObject，他们都是基于Objective-C运行环境的  
1. Cocoa框架  
  iOS众多框架中，最重要最基本的两个是：Foundation和UIKit  
  和界面无关的类基本是Foundation框架的，界面相关的是UIKit框架  
  ![Framework](https://pic1.zhimg.com/v2-b8ba69b58e878152185caba43a2da7b4_b.jpg)  
2. Foundation框架  
  下图中，灰色的是iOS不支持的  
  ![foundation](https://pic4.zhimg.com/v2-133427ac6f96373070d7ebfbf5d28cfb_b.jpg)  
  Objective-C Foundation Continued  
  ![foundation](https://pic4.zhimg.com/v2-919c52a662aaf9f2fd37d0e3b0399b47_b.jpg)  
  Objective-C Foundation Continued
  ![foundation](https://pic2.zhimg.com/v2-69e31bb71179f9a2de6114cd3e69e72d_b.jpg)  
  > 1. 值对象  
  > 2. 集合  
  > 3. 错做系统服务，包括下面三个：文件系统和URL， 进程间通讯。。。。
  > 4. 通知  
  > 5. 归档和序列化  
  > 6. 表达式和条件判断  
  > 7. Objective-C 语言服务   

3. UIKit框架
框架类组织架构图  
![uikit](https://pic4.zhimg.com/v2-6de81687ec60f6d46d085828ff784377_b.jpg)  
可以看出，responder是图中最大分支的根系，UIResponder为处理响应时间和响应链，定义用户界面和默认行为  
当用户用手指滚动列表或者在虚拟键盘上输入时，UIKit就生成事件传送给UIResponder响应链，直到链中有对象处理这个事件  
相应的核心对象：UIApplication, UIWindow,  UIView都直接或间接的从UIResponder继承  

#### Cocoa对象
根类`NSObject`是大部分Object-C的根类，其他类继承`NSObject`，访问Objective-C运行时系统的基本接口，这样其他类的实例可以获得运行时的能力  
1. 根类和根协议  
  NSObject不但是个类名，也是个协议的名字，NSObject协议制定了根协议必须实现的接口  
2. 根类的主要方法：  
  分配、初始化、复制  
  - `alloc`和`allocWithZone`方法用于从内存中分配一个内存对象，并使对象指向其运行时的类定义  
  - `init`方法是对象初始化  
  - `new`是一个将简单的内存分配和初始化结合起来的方法  
  - `copy`和`copyWithZone`

  对象的保持和清理：
    - `retain` ：方法增加对象的保持次数  
    - `release`： 方法减少对象的保持次数
    - `autorelease`: 方法也是减少对象的保持次数，但是以推迟的方法  
    - `retainCount`: 方法返回对当前的保持次数  
    - `dealloc`: 方法由需要释放对象的实例变量以及释放动态分配的内存的实现  
3. Cocoa对象的生命周期  
  ![life](https://pic1.zhimg.com/v2-16ca46aa305437bdb41f385bfff2ee20_b.png)



### 如何制作UI界面
1. 代码手写UI  
  学院派的极客或者依赖多人合作的大项目大规模使用  
  具有最好的代码重用性  
  但是慢，维护痛苦

2. Xibs  
  使用IB和xib文件组织UI，可以省下大量代码和时间  
  一般来说，单个的xib文件对应一个ViewController，而对于一些自定义的view，旺旺也会使用单个xib并从main bundle进行加载的方式来载入  
  IB帮助完成view的创建，布局与file owner的关系映射灯一些列工作，请牢记xib的文件都是view的内容  
  xib并不是完美的，最大的问题在于xib中的设置旺旺并非最终设置，在代码中可以覆盖xib文件中的UI设计，在不同的地方堆同一个属性进行设置，是维护的噩梦
  而且IB没有逻辑判断，也很难再运行时进行配置，所以可以辅以代码来补充和完成功能  

3. StoryBoard  
  简单的来说，可以把StoryBoard看做是一组viewController对应的xib，以及它们之间的转换方式的集合。  
  在StoryBoard中不仅可以看到每个ViewController的布局样式，也可以明确的知道各个ViewController之间的转换关系，  
  相对于xib，其代码量更少，也由于集合了各个xib，使得对于界面的理解和修改的速度也得到了更大的提升  
  StoryBoard面临的最大问题是多人协作，所以可以将项目的不同部分分解成若干个StoryBoard，并各自负责  

### IOS事件传递
无论IOS还是Android都是事件驱动的操作系统  

### 基本原理
#### 事件的分类
- IOS将事件分为三类
  - Multitouch Events  
    多点触摸事件  
  - Motion Events  
    移动事件，用户摇晃，移动，倾斜手机时产生的事件  
  - Remote Control Events  
    远程控制事件，用户在操作多媒体的时候产生的事件，如播放音乐，视频等  

> `Multitouch Events`有明确的触摸视图，`UIKit`框架的`View`对象可以明确获取当前点击的视图对象以及坐标。 而`Motion Events`和`Remote Control Events`没有明确的交互界面的概念，iOS系统为此提出了`Responder`概念  

##### Responder
1. 什么是`Responder`
  `Responder`是`UIKit`框架封装的一个对象类型，它可以响应并处理事件，所有`Responder`对象的基类都是`UIResponder`    
  ![responder](http://upload-images.jianshu.io/upload_images/703764-0932ae01563d3ca5.png?imageMogr2/auto-orient/strip%7CimageView2/2/w/1240)  
  上图可以看出：`UIApplication`, `UIViewController`, `UIView`都是`UIResponder`对象，都具有对事件进行响应，处理的能力  

  `UIResponder`的类里面的一些方法和属性  
  ```Objective-C
  - (UIResponder )nextResponder;
  - (BOOL)canBecomeFirstResponder;    // default is NO
  - (BOOL)becomeFirstResponder;
  // Touch Event
  - (void)touchesBegan:(NSSet<UITouch > )touches withEvent:(UIEvent )event;
  - (void)touchesMoved:(NSSet<UITouch *> )touches withEvent:(UIEvent )event;
  - (void)touchesEnded:(NSSet<UITouch *> )touches withEvent:(UIEvent )event;
  - (void)touchesCancelled:(NSSet<UITouch *> )touches withEvent:(UIEvent )event;
  // Motion Event
  - (void)motionBegan:(UIEventSubtype)motion withEvent:(UIEvent )event NS_AVAILABLE_IOS(3_0);
  - (void)motionEnded:(UIEventSubtype)motion withEvent:(UIEvent )event NS_AVAILABLE_IOS(3_0);
  - (void)motionCancelled:(UIEventSubtype)motion withEvent:(UIEvent )event NS_AVAILABLE_IOS(3_0);
  // Remote Control Event
  (void)remoteControlReceivedWithEvent:(UIEvent)event NS_AVAILABLE_IOS(4_0);
  ```

  由此可见，`UIResponder`对象可以处理`TouchEvent`,`MontionEvent`,`Remote Control Event`和`nextResponder`
  其中`nextResponder`可以获取到下一个关联的`Responder`，`Responder`对象正是关联`nextResponder`引用组成了一个Responder链，我们称之为The Responder Chain，系统事件会沿着这个`Responder Chain`传播到`nextResponder`，直到最后一个`Responder`,如果依然没有处理该事件，事件就会被舍弃。当然系统必须找到第一个`Responder`，即`First Responder`   
