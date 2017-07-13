# 贝塞尔曲线
#### 贝塞尔曲线是用一些列的点来控制曲线状态的，可以将这些点分为两类  
- 数据点： 确定曲线的启始和结束位置  
- 控制点： 确定曲线的弯曲程度  


#### 一阶曲线原理  
没有控制点，仅有两个数据点(P0, P1)，最终效果是一个线段  
![一阶](https://upload.wikimedia.org/wikipedia/commons/0/00/B%C3%A9zier_1_big.gif)  

#### 二阶曲线原理  
由两个数据点(A, C)，一个控制点(B)来描述曲线状态  
![二阶](http://ww2.sinaimg.cn/large/005Xtdi2jw1f361oje6h1j308c0dwdg0.jpg)  
连接AB BC并在AB上取点D，BC上取点E，使其满足条件AD/AB = BE/BC  
连接DE，取点F，使得 AD/AB = BE/BC = DF/DE   
![动态图](https://upload.wikimedia.org/wikipedia/commons/3/3d/B%C3%A9zier_2_big.gif)  

#### 三阶曲线原理
三阶曲线由两个数据点(A, D)，两个控制点(B, C)来描述曲线状态，如下：  
![三阶](http://ww2.sinaimg.cn/large/005Xtdi2gw1f36myeqcu5j308c0dwdg2.jpg)  
计算方式与二阶类似  
![三阶](https://upload.wikimedia.org/wikipedia/commons/d/db/B%C3%A9zier_3_big.gif)  


## UIBezierPath
此类是Core Graphics框架关于路径的封装，使用此类可以定义简单的形状  
UIBezierPath 是 CGPathRef数据类型的封装，如果是基于矢量形状的路径，都用直线和曲线去创建。

#### 画图步骤
1. 创建一个UIBezierPath对象  
2. 调用 `moveToPoint`  
3. 设置初始线段的起点  
4. 添加线或者曲线去定义一个或多个子路径  
5. 改变UIBezierPath对象和绘图相关的属性，如画笔属性，填充样式等  

#### UIBezierPath 的工厂类方法  
```objective-c
(instancetype)bezierPath
```
使用较多，这个方法创建的对象，可以根据我们的需要任意定制样式  

```objective-c
(instancetype)bezierPathWithRect:(CGRect)rect;
```
创建一个矩形画的贝塞尔曲线  

```objective-c
(instancetype)bezierPathWithOvalInRect:(CGRect)rect;
```
创建一个矩形画内切曲线，通常用来画圆或者椭圆  

```objective-c
(instancetype)bezierPathWithRoundedRect:(CGRect)rect cornerRadius:(CGFloat)cornerRadius;
```
可以画一个圆角矩形  

```objective-c
(instancetype)bezierPathWIthRoundedRect:(CGRect)rect byRoundingCorners:(UIRectCorner)corners cornerRadii:(CGSize)cornerRadii;
```
可以指定某个角为圆角的矩形  

```objective-c
(instancetype)bezierPathWithArcCenter:(CGPoint)center radius:(CGFloat)radius startAngle:(CGFloat)startAngle endAngle:(CGFloat)endAngle clockwise:(BOOL)clockwise;
```
用于画圆弧，  
center: 圆弧中心点坐标  
radius: 圆弧所在圆的半径  
startAngle: 狐仙开始的角度值  
endAngle: 狐仙结束的角度值  
clockwise: 是否顺时针画弧线  

```objective-c

```

```objective-c

```

```objective-c

```

```objective-c

```
