# 导航栏
### 1. UINavigationBar VS UINavigationItem
#### 官方解释
UINavigationBar 提供一种对导航层级内容的控制，它是一个bar，最典型的是显示在屏幕的顶端，包含屏幕层级的导航按钮，主要属性是left(back) button，center title，一个可选的right button，navigation bar可以单独使用，或者和导航控制器一起使用  

UINavigationItem 一个UINavigationItem对象管理着UINavigationBar对象中显示的按钮和视图。每一个push到navigation栈的controller必须有一个UINavigationItem对象，这个对象包含了想要显示的按钮和视图。UINavigationController对象使用最顶层的两个view controller去构建navigation bar的内容  

> UINavigationItem是一个独特的实例，当控制器视图被推倒导航控制器中时，它来代表这个视图控制器。
