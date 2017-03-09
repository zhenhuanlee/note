原文：[Understanding Scope in
Ruby](https://www.sitepoint.com/understanding-scope-in-ruby/)
# 作用域
### Ruby Variable Scope
+ Class variable(@@): 当前类和子类有效，外部无效
+ Instance variable(@):
  只在特定的对象中有效，贯穿所有的类的实例的方法，定义的类中无效
+ Global variable($): 作用域为整个项目
+ Local variable: 取决于所在位置

![scope](../assets/variable-scope-ruby.jpg)

### When is a local variable in scope(什么时候一个局部变量是在作用域中的？)
ruby
的解释器会将局部变量放到作用域中，无论何时看到它被赋值,也不管赋值语句有没有执行
```ruby
if false
  a = 'hello'
end
a     #=> nil
```

### 命名冲突
因为方法的调用者可以不指定，有点像局部变量，所以有可能会冲突
```ruby
def sth
  'hello'
end
p sth    #=> 'hello'

sth = 'Ruby'
p sth   #=> 'Ruby'
```
当冲突发生时，变量名优先，但是可以通过`sth()`，或者`self.sth`来调用

### 局部 vs 实例变量
- 实例变量和具体的对象相绑定 
- 局部变量和具体的域相关联
- 实例变量随着对象的不同而不同
- 局部变量随着域的不同而被改变或替换

### 作用域门：一个必要的概念去理解作用域
每当定义

1. class
2. module
3. method

中的任何一个,你就进入了一个新的作用域

### 打破作用域门
不用上面的几个关键词就可以了  
class    =>  Class.new {}  
module   =>  Module.new {}  
def xx   =>  define_method :xx {}

### block 是作用域门么
```ruby
sample_list = [1, 2, 3]
hi = '123'

sample_list.each do {
  puts hi           # 正确打印 123
  hello = 'hello'
}

p hello             # undefined local variable for method "hello"
```
如果你不想代码快去修改变量，block-local(块局部)变量可以帮忙，在块的参数后面加一个分号
```ruby
hi = 'hi'
hello = 'hello'

3.times do |i; hi, hello|
  p i
  p hi            # nil
  p hello         # nil
  hi = 'hi again'
  hello = 'hello again'
end

p hi      # "hi"
p hello   # "hello"
```
当使用`do end` 代码块时，就引入了一个新的作用域
不管方法是`each, select, map, detect ...`

### 块和域的一些奇怪的地方
```ruby
2.times do
  i |= 1
  print "#{i} "
  i += 1
  print "#{i} "
end
```
结果竟然是`1 2 1 2`
这是因为每一次迭代都是定义一个新的代码块，会重置本地变量
```ruby
def foo
  x = 1
  lambda { x }
end

x = 2

p foo.call
```
答案是1，原因是块和块的对象(procs,
lambdas)看的是定义时候的作用域，而不是调用时的作用域
他们这样做是因为他们在 ruby 中被视为是闭包，闭包是包含以下特征的简单代码
- 像对象一样被传递(先定义，后调用)
- 能记住闭包(上例中是lambda)被定义时作用域内的变量
这可以在很多地方派上用场，如定义一个无限数字生成器
```ruby
def increase_by(i)
  start = 0
  lambda { start += i }
end

increase = increase_by(3)
start = 453534534
p increase.call   # 3
p increase.call   # 6
```
可以这样使用的原因是，lambda作为一个闭包，记住了定义它的作用域内的变量的值，也就是start的值
也可以使用lambda延时变量的定义
```ruby
i = 0
a_lambda = lambda do
  i = 3
end

p i     # 0
a_lambda.call
p i     # 3
```

```ruby
a = 1
ld = lambda { a }
a = 2
p ld.call     # 2
```
不是1是因为，a = 2也是定义lambda
的作用域，在lambda第一次被调用的时候才能确定a的值，定义非调用

### 两个方法如何分享一个变量
```ruby
def let_us_define_methods
  shared_variable = 0

  Kernel.send(:define_method, :increase_var) do
    shared_variable += 1
  end

  Kernel.send(:define_method, :increase_again) do
    shared_variable += 1
  end
end

let_us_define_methods # methods defined now
p increase_var    # 2
p increase_again  # 3
```
`Kernel.send(:define_method, :increase_var)`
equal to
`define_method :increase_var`

### 顶层作用域
如果在顶层作用域，说明你还没有调用任何方法，或者所有的方法都已经返回了
ruby中一切皆对象，即使你身处顶层作用域，你在一个叫main（属于Object类）的对象中
```ruby
p self         # main
p self.class   # Object
```

### 我在哪
经常的，在debugging 的时候，如果你知道self的值，那么很多头疼的问题都会被解决
self当前的值影响了当前的实例变量和没有显式接受者的方法，如果你接收到一个`undefined
method/instance variable`,但是你确定你已经定义了，那么你可能追踪self出了问题

### Exercise
```ruby
class SomeClass
  b = 'hello'
  @@m = 'hi'
  def initialize
    @some_var = 1
    c = 'hi'
  end

  def some_method
    sleep 1000
    a = 'hello'
  end
end

some_object = SomeClass.new
some_object.some_method 
```
