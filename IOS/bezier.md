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
