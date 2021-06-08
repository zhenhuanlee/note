### layout机制相关方法
- (CGSzie)sizeThatFits:(CGSize)size  
- (void)sizeToFit  

- (void)layoutSubviews  
- (void)layoutIfNeeded  
- (void)setNeedsLayout

- (void)setNeedsDisplay  
- (void)drawRect  

#### layoutSubviews 会在以下情况被调用  
1. `init`不会触发`layoutSubviews`，但是调用`initWithFrame`进行初始化时，当rect值不为CGRectZero时会触发  
2. `addSubview`会触发  
3. 设置view的frame时会触发  
4. 滚动一个`UIScrollView`会触发  
5. 旋转Screen会触发父UIView上的  
6. 改变一个`UIView`大小的时候也会触发父类`UIView`上的  

### 常见方法
- layoutSubviews  默认没有做任何事情，需要子类进行重写  
- setNeedsLayout  标记为重新布局，异步调用layoutIfNeeded刷新布局，不立即刷新，但`layoutSubviews`一定会被调用  
- layoutIfNeeded  如果，有需要刷新的标记，立即调用`layoutSubviews`进行布局，如果没有，不会调用`layoutSubviews`，一个元素第一次显示之前，一定被标记为`needLayout`  

### 重绘
- `drawRect:(CGRect)rect`  重写此方法，执行重绘任务  
- `setNeedsDisplay`        标记为重绘，异步调用`drawRect`方法  
- `setNeedsDisplayInRect:(CGRect)invalidRect`  标记为需要局部重绘  

### sizeToFit & sizeThatFits
- `sizeToFit`会自动调用`sizeThatFits`  
- `sizeToFit`不应该在子类中被重写，应该重写`sizeThatFits`  
- `sizeThatFits`传入的参数是`receiver`当前的size，返回一个适合的size  
- `sizeToFit`可以被手动直接调用  
- `sizeToFit`和`sizeThatFits`方法都没有递归，对`Subviews`也不负责，只负责自己   

### 其他
- `layoutSubViews`对`subviews`重新布局  
- `layoutSubViews`方法调用先于`drawRect`  
- `setNeedsLayout`在`receiver`标上一个需要被重新布局的标记，在系统`runloop`的下一个周期自动调用`layoutSubviews`  
- `layoutIfNeeded`，`UIKit`会判断该`receiver`是否需要layout，`layoutIfNeeded`遍历的不是`superview`链，应该是`subviews`链  
- `drawRect`是对`receiver`的重绘，能够获得`context`  
- `setNeedDisplay`在`receiver`标上一个需要被重新绘图的标记，下一个`draw`周期自动重绘，刷新频率是60hz，也就是1/60秒后重绘  
