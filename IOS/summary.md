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
  ```objective-c
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
