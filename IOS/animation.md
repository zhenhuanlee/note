# Core Animation
iOS动画主要指Core Animation   

### Core Animation类图以及常用字段
#### 继承关系  
![继承关系](http://img.my.csdn.net/uploads/201507/23/1437617562_3190.png)  

#### 常用字段
- duration  动画的持续时间  
- beginTime  动画的开始时间  
- repeatCount  动画的重复次数  
- autoReverse  执行的动画按照原动画返回执行  
- timingFunction  控制动画的显示节奏，下面提供五种选择：  
  - kCAMediaTimingFunctionLiner  匀速线性动画  
  - kCAMediaTimingFunctionEaseIn  先慢后快  
  - kCAMediaTimingFunctionEaseOut  先快后慢  
  - kCAMediaTimingFunctionEaseInEaseOut  先慢后快再慢  
  - kCAMediaTimingFunctionDefault  中间比较快  
- delegate  动画代理，能检测动画的执行和结束  
```objective-c
@interface NSObject (CAAnimationDelegate)
- (void)animationDidStart:(CAAnimation * )anim;
- (void)animationDidStop:(CAAnimation * )anim finished:(BOOL)flag;
@end
```
- path  关键帧动画中的执行路径  
- type  过渡动画的动画类型，系统提供了四种：
  - kCATransitionFade  渐变效果  
  - kCATransitionMoveIn  进入覆盖效果  
  - kCATransitionPush  推出效果  
  - kCATransitionReveal  揭露离开效果  
- subtype  过渡动画的动画方向  
  - kCATransitionFromRight  从右侧进入  
  - kCATransitionFromLeft   从左侧进入  
  - kCATransitionFromTop    从顶部进入  
  - kCATransitionFromBottom  从底部进入  

### iOS动画的调用方式  
#### UIView代码块调用
```objective-c
[UIView animateWithDuration:1 animations:^{
    view1.frame = CGRectMake(100, 100, 100, 100);
} completion:^(BOOL finished){
    view1.frame = CGRectMake(0, 100, 50, 50);
}];
// 会停留在动画结束的状态
```

#### UIView [begin commit]模式
```objective-c
[UIView beginAnimations:nil context:nil];
[UIView setAnimationDuration:1];
view1.frame = CGRectMake(50, 50, 100, 100);
[UIView commitAnimations];
// 会停留在动画结束的状态
```

#### 使用Core Animation中的类
```objective-c
CABasicAnimation *anim = [CABasicAnimation animationWithKeyPath:@"position"];
anim.fromValue = [NSValue valueWithCGPoint:CGPointMake(100, 100)];
anim.toValue = [NSValue valueWithCGPoint:CGPointMake(200, 200)];
anim.timingFunction = [CAMediaTimingFunction functionWithName:kCAMediaTimingFunctionEaseInEaseOut];
anim.duration = 1;
[view1.layer addAnimation:anim forKey:@"lll"];
// 会回到动画的初始状态
```

### iOS动画的使用
#### 基础动画(CABasicAnimation)  
基础动画提供了堆CALayer对象中可变属性进行简单动画的操作。  
比如：位移、透明度、缩放、旋转、背景色...   
重要属性:   
  - fromValue   keyPath对应的初始值  
  - toValue     keyPath对应的结束值  
> 如果`fillMode = kCAFillModeForwards` & `removedOnComletion=NO`，那么在动画执行完毕后，图层会保持显示动画执行后的状态。但在实际上，图层的属性值还是动画执行前的初始值，并没有真正被改变  

#### 关键帧动画(CAKeyframeAnimation)
`CAKeyframeAnimation`和`CABasicAnimation`都是`CAPropertyAnimation`的子类。`CABasicAnimation`只能从一个数值(fromValue)到另一个数值(toValue)，而`CAKeyframeAnimation`则会使用一个`NSArray`保存一组关键帧  
重要属性：  
  - values    就是上述的`NSArray`对象，里面的元素成为关键帧(keyFrame)。动画对象会在指定的事件(duration)内一次显示values数组中的每一个关键帧  
  - path      可以设置一个`CGPathRef` or `CGMutalbePathRef`，让层跟着路径移动。path只对`CALayer`的`anchorPoint`和`position`起作用。如果设置了path，那么values将被忽略  
  - keyTimes  可以为对应的关键帧指定对应的时间点，其取值范围为0~1.0，keyTimes中每一个时间值都对应values中的每一帧。当keyTimes没有设置的时候，各个关键帧的时间是平均的  
  ```objective-c
  // 圆周运动
  CAKeyframeAnimation *keyframe = [CAKeyframeAnimation animationWithKeyPath:@"position"];
  UIBezierPath *path = [UIBezierPath bezierPathWithOvalInRect:CGRectMake(100, 100, 200, 200)];
  keyframe.path = path.CGPath;
  keyframe.duration = 1;
  [view1.layer addAnimation:keyframe forKey:@"dd"];
  ```
  > tips: 可以将 CABasicAnimation 看做只有两个关键帧的 CAKeyframeAnimation    

#### 组动画(CAAnimationGroup)
`CAAnimation`的子类，可以保存一组动画对象，将`CAAnimationGroup`对象加入层后，组中所有动画对象可以同时并发运行  
重要属性：  
  - animations   用来保存一组动画对象的NAarray  
  ```objective-c
  CAAnimationGroup *group = [CAAnimationGroup animation];
  // 旋转
  CABasicAnimation *basic = [CABasicAnimation animationWithKeyPath:@"transform.rotation"];
  // 变大
  CABasicAnimation *basic2 = [CABasicAnimation animationWithKeyPath:@"transform.scale"];
  // 位置
  CAKeyframeAnimation *keyframe = [CAKeyframeAnimation animationWithKeyPath:@"position"];
  // 各个帧发生的时间点
  NSValue *p0 = [self packPointWithX:100 Y:200];
  NSValue *p1 = [self packPointWithX:100 Y:300];
  NSValue *p2 = [self packPointWithX:200 Y:300];
  NSValue *p3 = [self packPointWithX:200 Y:200];
  NSValue *p4 = [self packPointWithX:400 Y:200];
  // 旋转的度
  basic.toValue = @(M_PI);
  basic2.fromValue = @(0.8);
  basic2.toValue = @(1.2);

  keyframe.values = @[p0, p1, p2, p3, p4];
  keyframe.keyTimes = [NSMutableArray arrayWithObjects: @0, @0.1, @0.2, @0.3, @0.4, nil];

  group.animations = @[basic, keyframe, basic2];
  group.duration = 4;

  [view1.layer addAnimation:group forKey:@"groupAnimation"];
  ```

#### 过渡动画(CATransition)
CAAnimation的子类，用于做过渡动画或者转场动画，能够为层提供移出屏幕和移入屏幕的动画效果  
重要属性：  
  - type   过渡动画类型  
  官方只提供了4种效果  
    - kCATransitionFade    渐变效果
    - kCATransitionMoveIn  进入覆盖效果  
    - kCATransitionPush    推出效果  
    - kCATransitionReveal  揭露离开效果  

  - subType  动画过渡方向  
    - kCATransitionFromRight    从右侧进入
    - kCATransitionFromLeft     从左侧进入
    - kCATransitionFromTop      从顶部进入
    - kCATransitionFromBottom   从底部进入

  - startProcess    动画起点(在整体动画的百分比)  
  - endProcess      动画终点(在整体动画的百分比)
```objective-c
containView = [[UIView alloc] initWithFrame:CGRectMake(0, 0, 200, 350)];
[self.view addSubview:containView];

view1 = [[UIView alloc] initWithFrame:containView.bounds];
[view1 setBackgroundColor:[UIColor yellowColor]];
[containView addSubview:view1];
view1.tag = 1;

view2 = [[UIView alloc] initWithFrame:containView.frame];
[view2 setBackgroundColor:[UIColor purpleColor]];
[containView addSubview:view2];
view2.tag = 2;

[UIView transitionFromView:[containView viewWithTag:2] toView:[containView viewWithTag:1] duration:1 options:UIViewAnimationOptionTransitionFlipFromLeft completion:^(BOOL f){

    [containView insertSubview:[containView viewWithTag:1] == view1 ? view2 : view1 belowSubview:[containView viewWithTag:1]];

    view1.tag = view1.tag == 1 ? 2 : 1;
    view2.tag = 2 / view1.tag;

}];
```
