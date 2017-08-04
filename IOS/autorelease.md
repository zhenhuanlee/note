## Autorelease
Autorelease是iOS中管理对象内存的机制  
MRC中调用`[obj autorelease]`来延迟内存的释放是一件简单的事；  
ARC中，甚至可以完全不知道Autorelease就可以管理好内存  

#### Autorelease对象什么时候释放
在没有手加`Autorelease Pool`的情况下，Autorelease对象是在当前的`runloop`迭代结束时释放，而它能释放的原因是系统在每个`runloop`迭代中都加入了自动释放池的`push`和`pop`  
