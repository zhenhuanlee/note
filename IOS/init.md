#### 为什么 `self = [super init]`
- alloc 会返回一个有效的未初始化的对象实例，`self`是`alloc`返回的指针  
- super 是一个"编译器指示符"，告诉编译器在父类中搜索方法  
- 优先调用`[super init]` 是为了使超类完成他们自己的初始化工作  
- `if(self = [super init])` 是为了防止父类初始化失败  

##### self & super
- `self` 是当前这个类的对象  
- `super` 是一个编译器指示符，和`self`指向同一个消息接受者  
