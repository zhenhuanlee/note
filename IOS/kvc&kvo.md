# KVC 和 KVO
Key-value coding(KVC)和Key-value observing(KVO)是两种驾驭OC动态特性并简化代码的机制  

### Key-Value Observing
The worst API in all of Cocoa. Its terrible API belies one of the most compelling features of the framework.

#### 在OC和Cocoa中，有许多的方法去传数据  
- NSNotification & NSNotificationCenter  
- Key-Value Observing  
- Delegates  
- Callbacks  

##### Subscribing
注册observer的方法是:  
```objective-c
- (void)addObserver:(NSObject * )observer
         forKeyPath:(NSString * )keyPath
            options:(NSKeyValueObservingOptions)options
            context:(void * )context
```
> `observer`: KVO注册的对象，这个对象必须实现`observeValueForKeyPath:ofObject:change:context:`  
> `keyPath`: 这个key path是相对于receiver而言的，是所观察的属性，不可为nil  
> `options`: `NSKeyValueObservingOptions`值的组合  
> `context`: 传给`observeValueForKeyPath:ofObject:change:context`，可以是任意值  

##### Responding  
所有的改变都汇聚到`observeValueForKeyPath:ofObject:change:context:`
