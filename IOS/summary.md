#### typedef
作用：给类型起别名，常用语简化复杂类型，变量类型意义化等  
```objective-c
typedef double NSTimeInterval;  // 给double取别名为NSTimeInterval（变量类型意义化）
typedef NSTimeInterval MyTime;  // 给NSTimeInterval取别名为MyTime
typedef char * MyString;        // 给char * 取别名为MyString

typedef struct Person
{
  char * name
}MyPerson;  // 给Person结构体取别名为MyPerson。使用：MyPerson p = {"Jack"};

typedef enum Gender
{
  Man,
  Woman
} MyGender;   // 给Gender枚举类型取别名为MyGender。使用：MyGender g = Man;

typedef void(^MyBlock) (int a, int b);  // 给block取别名为MyBlock
typedef int(* MyFunction) (int a, int b); // 给指向函数的指针取别名为MyFunction
```

#### define
作用文本替换（把出现的替换为定义的）  
```object-c
#define MyString @"hello world!" // MyString 替换后面的文本
#define MyString2 Mystring       // MyString2替换为MyString
```

#### typedef 和 define使用注意
- define 是文本替换，属于预编译指令，本身不参与编译，除非希望替换的文本中有`;`，否则不用加  
  typedef 是类型替换，语句的一种  
- define 写在方法/函数中则作用于从写的地方开始有小，直至使用`#undef`(不写此指令则后面一直有效)  
  typedef 写在方法/函数中，则作用域只在此方法/函数中  
- 若使用`typedef char * MyString;` 则`MyString s1, s2;`等价于`char *s1, *s2;`  
  若使用`#define MyString char *`  则`MyString s1, s2;`等价于`char *s1,s2`即`char *s1; char s2`  

#### OC遍历数组和字典  
1. for循环  
2. for-in 遍历  
3. 枚举器  
遍历数组
```objective-c
NSArray *array = @[@"2", @"3", @"3"];

// 获取数组枚举器(正向枚举)
NSEnumerator *enumerator = [array objectEnumerator];

id value = nil;

while (value = [enumerator nextObject]) {
  NSlog(@"%@", value);
}

// 反向枚举(逆序)
enumerator = [array reverseObjectEnumerator];

value = nil;
while (value = [enumerator nextObject]) {
  NSLog(@"%@", value);
}
```
遍历字典
```objective-c
NSDictionary *dic = [NSDictionary dictionaryWithObjectAndKeys:@"leeee", @"name",
@"23", @"age"];

NSEnumerator *enumerator2 = [dic objectEnumerator];

id value2 = nil;

while (value2 = [enumerator2 nextObject]) {
  NSLog(@"%@", value2);
}
```

#### file's owner 以及 outlet 与连线
xib是一个xml文件，app启动的时候会根据xml结构构造xib对应的界面及其控件  
file's owner 是对应的类，比如view对应的xib文件的file's owner对应的就是viewcontrol类  
outlet是针对xib文件中希望能够在外部引用的控件成员  
file's owner 和 label 之间的连线确定了代码中的outlet控件与xib中的控件的对应  

假设有两个独立的视图CnView和EnView，CnBiew和EnView上分别有个Button和Label，对于CnView，点击Button，Label显示“你好”，
EnView则显示"hello"  
1. XIB文件  
通过IB设计的CnView和EnView分别对应一个XIB文件，在CnView的XIB文件中，主要采用XML格式描述了Button和Label控件的属性，有个Button和Label，以及Button和Label的位置、大小等，EnView类似  
2. ViewController  
每个View对应有个视图控制器(多个View可以采用同一个视图控制器)，可以看做MVC中的C（其实不是），比如：CnView界面点击按钮之后的响应动作是由ViewController来实现的  
3. XIB和ViewController之间的关系  
XIB是表现，ViewController一方面响应XIB上的操作（点击，滑动等），同时也控制XIB的显示  
4. File's Owner（重点）
View和ViewController之间的对应关系，连接的桥梁(即，对于一个视图，它如何知道自己的界面的操作由谁来响应）  
5. First Responder  
View中每次都只会有一个对象与用户进行交互，当前交互的对象即为First Responder。比如点击文本框，那么此时文本框就是First Responder  
6. 输出口  
就是需要在代码里进行操作的控件。如果nib中的控件需要在代码中被修改，那么就要定义一个输出口指向这个控件，然后通过输出口来修改这个控件，输出口也就是一个指针指向了nib中的控件对象，使用关键字IBOutlet声明  
7. 操作  
触发一个控件后这个控件执行的方法。IBAction    
IBAction告诉Interface Builder这是一个操作方法，可以被某个控件触发。返回值IBAction类似于void，不返回值，操作方法接收一个参数(id)sender，控件触发了操作后就把自己传给sender





#### static/extern/内存分区说明
![内存分区示意图](http://img.blog.csdn.net/20170322161843508)  
##### 对象指针存放在栈区，


#### extern, static, const
1. extern  
  `extern`只能用来修饰全局变量，跨文件引用，如果在`.h`文件中声明了全局变量  
  ```
  #import <Foundation/Foundation.h>
  @interface ExternModel : NSObject
    // 这个是不合法的，因为OC的类会将这个全局变量当做成员变量来处理，而成员变量属性是要加{}的
    NSString *lhString; //声明全局变量的时候默认带有extern，这里必须显示声明
    // 正确的写法
    extern NSString *lhString; //这里由于带有extern会被认为是全局变量
  @end
  ```

#### MOVE
- M  Model  
- O  Operation Tree  
- V  View Tree  
- E  Events Changes


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
  - (void)touchesMoved:(NSSet<UITouch * > )touches withEvent:(UIEvent )event;
  - (void)touchesEnded:(NSSet<UITouch * > )touches withEvent:(UIEvent )event;
  - (void)touchesCancelled:(NSSet<UITouch * > )touches withEvent:(UIEvent )event;
  // Motion Event
  - (void)motionBegan:(UIEventSubtype)motion withEvent:(UIEvent )event NS_AVAILABLE_IOS(3_0);
  - (void)motionEnded:(UIEventSubtype)motion withEvent:(UIEvent )event NS_AVAILABLE_IOS(3_0);
  - (void)motionCancelled:(UIEventSubtype)motion withEvent:(UIEvent )event NS_AVAILABLE_IOS(3_0);
  // Remote Control Event
  (void)remoteControlReceivedWithEvent:(UIEvent)event NS_AVAILABLE_IOS(4_0);
  ```

  由此可见，`UIResponder`对象可以处理`TouchEvent`,`MontionEvent`,`Remote Control Event`和`nextResponder`
  其中`nextResponder`可以获取到下一个关联的`Responder`，`Responder`对象正是关联`nextResponder`引用组成了一个Responder链，我们称之为The Responder Chain，系统事件会沿着这个`Responder Chain`传播到`nextResponder`，直到最后一个`Responder`,如果依然没有处理该事件，事件就会被舍弃。当然系统必须找到第一个`Responder`，即`First Responder`   

#### Strong, Weak, Retain, Assign
- `assign` 用于非指针变量   
  用于基础数据类型（如NSInteger）和C数据类型（int,float,double,char等），还有id，如：  
  ```objective-c
  @property(nonatomic, assign)int number
  @property(nonatomic, assign)id className; // id必须用assign
  ```
  > 前面不加`*`就用assign  

- `retain` 用于指针变量  
  用于指针变量。定义了一个变量，然后这个变量在程序的运行过程中会被更改，并且影响到其他方法。一般是用于字符串(NSString, NSMutableString)，数组(NSArray, NSMutableArray)，字典对象，视图对象(UIView)，控制器对象(UIViewController)等  
  如：  
  ```objective-c
  @property (nonatomic, retain) NSString *myString;
  @property (nonatomic, Retain) UIView *myView;
  @property (nonatomic, retain) UIViewController *myViewController;
  ```
  > 有ARC，可以使用retain，系统会自动释放内存，在XCode4.3之上，`retain`和`strong`是一样的  

- `strong` & `weak`   
  ```objective-c
  @property(nonatomic, strong) MyClass *myObject;
  @property(nonatomic, retain) MyClass *myObject;
  // 上面两个是等价的  
  @property(nonatomic, weak) id<RNNewsFeedCellDelegate> delegate;
  @property(nonatomic, assign) id<RNNewsFeedCellDelegate> delegate;
  // 上面两个等价
  ```
  现在系统自动生成的属性都是用`weak`来修饰的  
  > 现在有ARC了，建议放弃`retain`改用`weak`  

- `copy`  
  据说效果和`retain`差不多，唯一区别就是，`copy`用于`NSString`而不是`NSMutableString`  
  竟然又说，一个类继承了`NSObject`，那么这个类里面的属性需要使用`copy`  

- 什么情况使用weak关键字，相比assign有什么不同  
  什么情况用weak？  
    1. 在ARC中，在有可能出现循环引用的时候，往往通过在其中一端使用weak来解决，比如:delegate代理
    2. 自身已经对它进行过一次强引用，没有必要再强引用一次，此时也会使用weak，自定义IBOutlet控件属性一般也使用weak；当然，也可以使用strong
  不同点：  
    1. `weak`此特质表明该属性定义了一种"非拥有关系"(nonowning relationship)。为这种属性设置新值时，设置方法既不保留新值，也不释放旧值。此特质同assign类似，然而在属性所指的对象遭到摧毁时，属性值也会清空(nil out)。而`assign`的“设置方法”只会执行针对“纯量类型”(scalar type，例如CGFloat或者NSInteger等)的简单赋值操作。  
    2. `assign`可以用非OC对象，而`weak`必须用于OC对象  
