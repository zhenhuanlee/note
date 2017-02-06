# 简明Python教程
For Python 3
## 基础
- 整型只有 int，没有long
- 双引号和单引号完全相同
- 用三引号来指定多行字符串 '''  or  """
```python
""" 这是一段多行字符串，这是他的第一行
This is the second line.
"What's your name?," I asked.
He said "Bond, James Bond."
"""
```
- 字符串是不可变的，且没有`char`类型
- format
数字只是个选项，可以选填
```python
age = 20
name = 'Swaroop'

print('{0} was {1} years old when he wrote this book'.format(name, age))
print('Why is {0} playing with tath python?'.format(name))
```
```python
# 对于浮点数 '0.333' 保留小数点后三位
print('{0:.3f}'.format(1.0/3))
# 使用下划线填充文本，并保持文字处于中间位置
# 使用(^)定义'___hello___'字符串长度为11
print('{0:_^11}'.format('hello'))
# 基于关键词输出 'Swaroop wrote A Byte of Python'
print('{name} wrote {book}'.format(name='Swaroop', book='A Byte of
Python))
```
```python
# 将结尾的换行符替换为 '11'
print('a', end='11')
```
- 转义序列
```python
"This is the first sentence. \
This is the second sentence."
# 相当于
"This is the first sentence. This is the second sentence."
```
- 原始字符串
用`r`或`R`来指定一个原始(Raw)字符串
```python
r"Newlines are indicated by \n"
```
在处理正则表达式时应全程使用原始字符串，否则，将会有大量 Backwhacking
需要处理。如反向引用可以通过 `'\\1'` 或 `r'\1'` 来实现。
- 逻辑行和物理行
逻辑行：编写程序时所看到的内容。Python 假定每一物理行个对应一个逻辑行
但是如果希望在一行物理行中指定多行逻辑行
```python
i = 5; print(i)
# 等同于
i = 5
print(i)

i = \
5
# 等同于
i = 5
```
- 缩进
错误的缩进可能会导致错误
```python
i = 5
# 下面将发生错误，注意行首有个空格
 print('Value is', i)
print('I repeat, the value is', i)
```

## 运算符与表达式 
#### 计算命令
优先级
- lambda
- if - else
- or
- and
- not x
- in, not in, is, is not, <, <=, >, >=, !=, ==
- |
- ^
- &
- <<, >>
- +, -
- *, /, //, %
- +x, -x
正、负、按位取反
- **
- x[index], x[index: index], x(arguments...), x.attributes
下标，切片，调用，属性引用
- (expressions..), [expressions...], {key: value...}, {expressions...}
显示绑定或数组、显示列表、显示字典、显示设置

## 控制流
- `if` 语句
```python
number = 23
guess = int(input('Enter an integer: '))

if guess == number:
  print('Congratulations, you guessed it.')
  print('(but you do not win any prizes!)')
elif guess < number:
  print('no, it is a little higher than that')
else:
  print('No, it is a little lower than taht')

print('Done')
```
> 没有`switch`，在某些情况下，使用字典能更快的完成

- for
```python
for i in range(1, 5):
  print i
else:
  print('The for loop is over')
```

## 函数
```python
def say_hello():
  print('hello world')

sya_hello()  # 调用函数
```

### 函数参数
术语：
- 在定义函数时给定的名称称作"形参"（parameters）
- 在调用函数时你所提供给函数的值称作"实参"（Arguments）

### 局部变量
所有变量的作用域是它们被定义的块，从定义他们的名字的定义点开始 

### `global`语句
可以使用函数之外的变量的值，但是应该被避免
```python
x = 50
def func():
  global x

  print('x is', x)
  x = 2
  print('Changed global x to', x)

func()
print('Value of x is', x)
```
### 默认参数值
```python
def say(message, times=1):
  print(message * times)

say('Hello')
say('world', 5)
```

### 关键字参数
只对其中的一些进行指定
```python
def func(a, b=5, c=10):
  print('a is', a, 'and b is', b, 'and c is', c)

func(3, 7)
func(25, c=24)
func(c=50, a=100)
```
### 可变参数
不限制函数的参数个数,通过`*`来实现
```python
def tatal(a=5, *numbers, **phonebook):
  print('a', a)

  # 遍历元组中所有数据
  for single_item in numbers:
    print('single item', single_item)
  
  # 遍历字典中所有项目
  for first_part, second_part in phonebook.items():
    print(first_part, second_part)

print(total(10, 1, 2, 3, Jack=123, Jhon=2231, Inge=123))
```
### return
```python
def maxium:
  if x > y:
    return x
  elif x = y:
    return 'The numbers are equal'
  else:
    return y

print(maxium(2, 3))
```
如果只是`return`相当于`return None`
每一个函数都在其末尾隐藏了一个`return None`
`pass`语句用于指示一个没有内容的语句块

### DocStrings
文档字符串(Document Strings)
用于更好的记录程序，并让其更加容易理解
可以通过函数来获取文档
```python
def print_max(x, y):
  '''Prints the maxium of two numbers.
  
  The two numbers must be integer.'''
  # 如果可能将其转换为整数类型
  x = int(x)
  y = int(y)
  if x > y:
    print(x, 'is maxium')
  else:
    print(y, 'is maxium')

print_max(3, 5)
print(print_max.__doc__)
```
约定：第一行以某一个大写字母开始，以句号结尾。第二行为空行。第三行是任何详细的解释说明。
文档还可以`help(print_max)`

## 模块 
模块是一种可重用的程序，包是用以组织模块的另一种层次结构
- 最简单的方法是创建一个包含函数、变量以`.py`为后缀的文件
- 还可以使用撰写Python解释器本身的本地语言来编写模块,如可以使用 C语言
  来编写 Python
模块，并且在编译后,可以通过标准的Python解释器在你本地的Python代码中使用它
案例：
```python
import sys

print 'The command line arguments are:'
for i in sys.argv
  print(i)

print('\n\nThe PYTHONPATH is', sys.path, '\n')
```
输出
```python
$ python module_using_sys.py we are arugments
The command line arguments are:
module_using_sys.py
we
are
arguments

The PYTHONPATH is ['/tmp/py',
#many entries here, not shown here]
```
#### 它是如何工作的
- 通过`import`引入`sys`模块。
`sys`模块包含了与Python解释器及其环境相关的功能，也就是所谓的系统功能(System)
- 如果它不是一个已编译好的模块，即用Python 编写的模块，那么 Python
  解释器将从它的`sys.path`变量所提供的目录中进行搜索。如果找到了对应的模块，则该模块中的语句将开始运行
- `sys`模块中的`argv`变量通过使用点号予以指明`sys.argv`，清晰的表明这一名称是`sys`模块的一部分
- `python module_using_sys.py we are arguments`，Python
  将命令行参数存储在`sys.argv`变量中供我们使用，`sys.argv[0] =
'module_using_sys.py'`、sys.argv[1] = 'we'、

### 按字节码编译的.pyc文件
导入一个文件是一件代价高昂的事情，创建按字节码编译的(Byte-Compiled)文件(以`.pyc`为扩展名)，是将Python转换成中间形式的文件，在下一次从其他不同的程序导入模块时会更加快速，因为导入模块时所需要的一部分处理工作已经完成了。同时，这些按字节码编译的文件是独立于运行平台的。
### 模块的`__name__`
每一个Python模块都定义了它的`__name__`属性。如果它与`__main__`属性相同，则代表这一模块是用户独立运行的
```python
if __name__ == '__main__':
  print('This program is being run by itseld')
else:
  print('I am being imported from another module')
```
### 编写自己的模块
每一个Python程序同时也是一个模块
```python
## mymodule.py

def say_hi():
  print('Hi, this is mymodule speaking.')

__version__ = '0.1'
```
```python
import mymodule
# 等价于
from mymodule import say_hi, __version

from mymodule import *
# 会引入 say_hi 等所有公共名称，但是不会引入 __version__，因为后者以双下划线开头
# 尽量避免使用from..import 这种格式
```
### `dir`函数
`dir()`能够返回由对象所定义的名称列表，如果这一对象是一个模块，则该列表会包含函数内所定义的函数、类与变量
该函数接受参数，如果参数是模块名称，函数将返回这一指定模块的名称列表。
如果没有提供参数，函数将返回当前模块的名称列表。
```python
import sys

# 输出 sys 模块中的属性名称
dir(sys)
['__displayhook__', '__doc__'...]

# 给出当前模块的属性名称
dir()
['__builtins__', '__doc__'...]

# 创建一个新的变量 'a'
a = 5
dir()
['__builtins__', '__doc__'..., 'a']

# 删除或移除一个名称
del a
dir()
['__builtins__', '__doc__'...]
```
`del`用于删除一个变量或名称
### 包
变量通常位于函数内部，函数与全局变量通常位于模块内部，
包(Packages)可以组织起这些模块
包：一个包含模块与一个特殊的`__init__.py`文件的文件夹，后者向Python表明这一文件夹是特别的，因为其包含Python模块
下面是构建的文件夹的结构：
```python
- <some folder present in the sys.path>/
  - world/
    - __init__.py
    - asia/
      - __init__.py
      - india/
        - __init__.py
        - foo.py
    - africa
      - __init__.py
      - madagascar/
        - __init__.py
        - bar.py
```
## 数据结构
用来存储一系列相关数据的集合
Python 有四中内置的数据结构--列表(List)、元组(Tuple)、字典(Dictionary)、集合(Set)
### 列表(List)
有序项目的集合
```python
# this is my shop list
shoplist = ['apple', 'mongo', 'carrot', 'banana']

print('I have', len(shoplist), 'items to purchase.')

print('These items are:', end=' ')
from item in shoplist:
  print(item, end=' ')

shoplist.append('rice')
shoplist.sort()

olditem = shoplist[0]
del shoplist[0]

```
### 元组(Tuple)
- 元组同时也是一个序列
- 用于将多个对象保存到一起，可以近似的看作低配列表
- 元组的一大特征类似于字符串，他们是不可变的
- 元组通常用于保证某一语句或某一用户定义的函数可以安全的采用一组数值，意即元组内的数值不会改变
```python
zoo = ('python', 'elephant', 'penguin')

# 括号是可选的，但是推荐用括号
new_zoo = 'monkey', 'camel', zoo

new_zoo[2][0]

# 如果一个元组只有一个项目
singleton = (2, )
```
### 字典(Dictionary)
- 字典中的成对的键值-值对不会以任何方式进行排序，如果要，只能在使用它们之前进行排序
```python
ab = {
  'Swaroop': 'swaroop@swaroopch.com',
  'Larry': 'larry@wall.org',
  'Matsumoto': 'matz@ruby-lang.org',
  'Spammer': 'spammer@hotmail.com'
}

print("Swaroop's address is", ab['Swaroop'])

# 删除
del ab['Spammer']

for name, address in ab.items():
  print('Contact {} at {}'.format(name, address))

# 添加一对
ab['Guido'] = 'guido@python.org'

# 判断
if 'Guido' in ab:
  print("\nGuido's address is", ab['Guido'])
```
### 序列
列表、元组、字符串都可以看作序列(Sequence)的某种表现形式
- 序列的主要功能是资格测试(Membership Test)
  - `in`与`not in`表达式
  - 索引操作
列表、元组、字符串都拥有一种切片(Slicing)运算符，它能够允许我们序列中的某段切片
```python
shoplist = ['apple', 'mongo', 'carrot']
name = 'swroop'

# slice on a list
# 包含起始位置，但不包含结束位置
>>> shoplist[0:2]
['apple', 'mongo']
>>> oplist[:2]
['apple', 'mongo']
>>> shoplist[0:]
['apple', 'mongo', 'carrot']

# slice on a string
# 同上
>>> name[0:2]
'sw'

# 可以提供第三个参数表示步长，默认1
>>> shoplist = ['apple', 'mango', 'carrot', 'banana']
>>> shoplist[::1]
['apple', 'mango', 'carrot', 'banana']
>>> shoplist[::2]
['apple', 'carrot']
>>> shoplist[::-1]
['banana', 'carrot', 'mongo', 'apple']
```
### 集合(Set)
是简单对象的无序集合(Collection)。
通过集合可以测试某些对象的资格或情况，检查它们是否是其他集合的子集，找两个集合的交集等
```python
>>> bri = set(['brazil', 'russia', 'india'])
>>> bri_new = {'brazil', 'russia', 'india'}
>>> bri == bri_new
True
>>> 'india' in bri
True
>>> bric = bri.copy()
>>> bric.add('china')
>>> bric.issuperset(bri)
True
>>> bri.remove('russia')
>>> bri & bric
{'brazil', 'india'}
```
### 引用
当你创建了一个对象，并将其分配给某个变量时，变量只会查阅(Refer)某个对象，并且它也不会代表对象本身
```python
shoplist = ['apple', 'mango', 'carrot', 'banana']
mylist = shoplist
del shoplist[0]
mylist == shoplist    # True

mylist = shoplist[:]
del mylist[0]
mylist == shoplist   # False
```
如果想创建一份侏儒序列等复杂对象的副本(而非整数这种简单的对象(object))，必须使用切片操作来创作副本
简单的赋值，都将"查阅"

## 面向对象编程
### self
类方法与普通函数只有一种特定的区别--前者必须有一个额外的名字，这个名字必须添加到参数列表的开头，但是你不用在调用这个功能时为这个参数赋值，Python会为它提供。这种特定的变量引用的是对象本身，按照惯例，它被赋予`self`这一名称
> Python 中的`self`相当于C++中的指针以及Java与C#中的`this`指针
如：有个`MyClass`类，这个类有个`myobject`实例，当调用这个对象的方法
`myboject.method(arg1, arg2)`时，Python会自动将其转换成`MyClass.method(myobject,
arg1, arg2)`--这就是`self`的全部特殊之处所在

这同时意味着，如果有一个没有参数的功能，你依旧必须拥有一个参数--`self`

### 类
```python
class Person:
  pass  # 一个空的代代码块

p = Person()
print(p)
```

### 方法
```python
class Person:
  def say_hi(self):
    print('Hello, how are you?')

p = Person()
p.say_hi()
```

### `__init__`方法
会在类的对象被实例化(Instantiated)的时候立即运行
```python
class Person:
  def __init__(self, name):
    self.name = name

  def say_hi(self):
    print('Hello', self.name)

p = Person('Swaroop')
p.say_hi()
# Hello Swaroop
```

### 类变量与对象变量
- 类变量(Class
  Variable)是共享的--他们可以被属于该类的所有实例访问。只有一个副本，改变是全局的
- 对象变量(Object Variable)由类的每一个独立的对象或实例拥有
```python
class Robot:
  population = 0
  
  def __init__(self, name):
    self.name = name
    print("(Initializing {})".format(self.name))
    
    robot.population += 1

  def die(self):
    """I dead"""
    print("{} is being destroyed!".format(self.name))

    Robot.population -= 1

    if Robot.population == 0:
      print("{} was the last one.".format(self.name))
    else:
      print("There are still {:d} robots working.".format(Robot.population))

  def say_hi(self):
    """Greeting from robot

    you can do it. """
    print("Greetings, my masters call me {}".format(self.name))

  @classmethod
  def how_many(cls):
    """打印出当前的人口数量"""
    print("We have {:d} robots".format(cls.population))


droid1 = Robot("R2-D2")
# (Initializing R2-D2)
droid1.say_hi()
# Greetings, my masters call me R2-D2.
Robot.how_many()
# We have 1 robots.

droid2 = Robot("C-3P0")
# (Initializing C-3PO)
droid2.say_hi()
# Greetings, my masters call me C-3PO.
Robot.how_many()
# We have 2 robots.

print("\nRobot can do some work here. \n")

print("Robots have finished their work. So let's destroy them.")
droid1.die()
# R2-D2 is being destroyed!
droid2.die()
# C-3PO is being destroyed!
# C-3PO was the last one.

Robot.how_many()
# We have 0 robots.
```
`@classmethod`装饰器(Decorator)将`how_many`方法标记为类方法
- 除开以双下划线开头的`__privatervar`(私有变量)，所有的类成员都是公开的，可以被其它任何类或对象所使用
- 任何类或对象之中使用的对象其命名应以下划线开头，但这只是一个约定，不强制

### 继承
```python
class SchoolMember:
  '''代表任何学校里的成员'''
  def __init__(self, name, age):
    self.name = name
    self.age = age
    print('(Initialized SchoolMember: {})'.format(self.name))
  
  def tell(self):
    '''告诉我有关我的细节'''
    print('Name: "{}" Age: "{}"'.format(self.name, self.age), end=" ")

class Teacher(SchoolMember):
  '''代表一位老师'''
  def __init__(slef, name, age, salary):
    SchoolMember.__init__(self, name, age)
    self.salary = salary
    print('(Initialized Teacher: {})'.format(self.name))

  def tell(self):
    SchoolMember.tell(self)
    print('Salary: "{:d}"'.format(self.salary))

class Student(SchoolMember):
  '''代表一位学生'''
  def __init__(self, name, age, marks):
    SchoolMember.__init__(slef, name, age)
    self.marks = marks
    print('(Initialized Student: {})'.format(self.name))

  def tell(self):
    SchoolMember.tell(self)
    print('Marks: "{:d}"'.format(self.marks))

t = Teacher('Mrs. Shrividya', 40, 30000)
s = Student('Swaroop', 25, 75)

members = [t, s]
for member in members:
  # 对全体师生工作
  member.tell()

# Name:"Mrs. Shrividya" Age:"40" Salary: "30000"
# Name:"Swaroop" Age:"25" Marks: "75"
```

## 输入与输出
- 希望用户输入内容：`input()`
- 打印内容：`print`

### 文件
创建一个术语`file`类的对象，并使用它的`read`、`readline`、`write`方法来打开或使用文件，记得调用`close`方法
```python
poem = '''\
Programming is fun
When the work is done
if you wanna make your work alse fun:
  use Python!
'''

# 打开文件以编辑('w'riting)
f = open('poem.txt', 'w')
# 向文件中编写文本
f.write(poem)
# 关闭文件
f.close()

# 如果没有特别指定
# 将假定启用默认的阅读('r'ead)模式
f = open('poem.txt')
while True:
  line = f.readline()
  # 零长度指示 EOF
  if len(line) == 0:
    break
  # 每行（`line`）的末尾
  # 都已经有了换行符
  # 因为它是从一个文件中进行读取的
  print(line, end='')

# 关闭文件
f.close()
```
> 打开模式：   
> 1. 阅读模式(`r`)   
> 2. 写入模式(`w`)   
> 3. 追加模式(`a`)  
> 4. 还可以选择是通过文本模式(`t`)，还是二进制模式(`b`)来读取、写入或追加文本  
> 5. ...

### Pickle
一个标准模块，通过它可以将任何纯Python对象存储到一个文件中，并在稍后将其取回。这叫做*持久地(Persistently)*存储对象。   
案例(保存为[`io_pickle.py`](https://github.com/pumpkin2011/mypython/blob/master/io_pickle.py))
> 将一个对象存储到一个文件中：   
1. 二进制模式`open`文件  
2. 调用`pickle`模块的`dump`函数，这一过程被称作封装(Pickling)   
`pickle`模块的`load`函数接受返回的对象，这个被称为*拆封(Unpickling)*

## 异常
### 错误
错误处理器(Error Handler)会抛出(Raise)语法错误
### 异常
通过使用`try..except`处理异常   
案例(保存为[`exceptions_handler.py`](https://github.com/pumpkin2011/mypython/blob/master/exceptions_handler.py))
> 如果没有为`except`声明异常或错误的名称，它将处理*所有*错误与异常
> `else`子句将在没有发生异常的时候执行

### 抛出异常
可以通过`raise`语句来引发一次异常   
所引发的异常必须是直接或间接从属于`Exception`类   
案例(保存为[`exceptions_raise.py`](https://github.com/pumpkin2011/mypython/blob/master/exceptions_raise.py))

### Try...Finally
始终会被执行的语句块
案例：[`exceptions_finally.py`](https://github.com/pumpkin2011/mypython/blob/master/exceptions_finally.py)

### `with`语句
在`try`块中获取资源，然后在`finally`中释放是一种常见模式  
`with`能使这一过程以一种干净的姿态得以完成
```python
with open("poem.txt") as f:
  for line in f:
    print(line, end="")
```
> 将关闭文件的操作交由`with open`来自动完成  
`with`会获取由`open`语句所返回的对象  
它总会在代码块之前调用`thefile.__enter__`函数，并且在代码块执行完毕后调用`thefile.__exit__`

## 标准库
包含大量有用的模块,可以在[Library
Reference](https://docs.python.org/3/library/)中查找所有模块
### `sys`模块
包括了一些针对特定系统的功能，如`sys.argv`列表包含了命令行参数  
查看Python软件的版本：
```python
>>> import sys
>>> sys.version_info
sys.version_info(major=3, minor=6, micro=0, releaselevel='final', serial=0)
>>> sys.version_info.major == 3
True
```
### 日志模块
可以将一些Debugging信息或一些重要的信息存储  
案例：[`stdlib_logging.py`](/pumpkin2011/mypython/blob/master/stdlib_logging.py)

## 更多
### 传递元组
可以从一个函数中返回两个不同的值
```python
>>> def get_error_details():
      return (2, 'details')

>>> errnum, errstr = get_error_details()
>>> errnum
2
>>> errstr
'details'
```
### 特殊方法
诸如`__init__`和`__del__`等一些方法对于类来说有特殊意义  
特殊方法用来模拟内置类型的某些行为，例子：  
如果希望为自定义的类使用`x[key]`索引操作，那么需要做的只是实现`__getitem__()`方法
```python
class A:
  def __getitem__(self, index)
    print('act like a list', index)

>>> a = A()
>>> a[1]
act like a list 1
```
- `__init__(self, ...)`
在新创建的对象被返回准备使用时被调用
- `__del__(self)`
在对象被删除之前调用（使用机制不可预测，避免使用）
- `__str__(self)`
当时用`print`或`str()`时被调用
- `__lt__(self, other)`
`<`运算符
- `__getitem__(self, key)`
使用`x[key]`索引操作时被调用
- `__len__(self)`
当针对序列对象使用内置`len()`函数时被调用
### 单语句快
如果语句块只有一句，可以在同一行指定
```python
>>> flag = True
>>> if flag: print('Yes')
...
Yes
```
### Lambda表格
`lambda`可以创建一个新的函数对象。`lambda`需要一个参数，后面跟一个表达式作为函数体，这个表达式执行的值将作为这个新函数的返回值  
```python
ipoints = [{'x': 2, 'y': 3},
           {'x': 4, 'y': 1}]
points.sort(key=lambda i: i['y'])
print(points)
```
> `list`的`sort`方法可以获得一个`key`参数，用以决定列表的排序方式  
### 列表推导(List Comprehension)
从一个现有的列表中得到一个新列表  
案例：
```python
listone = [2, 3, 4]
listtwo = [2*i for i in listone if i > 2]
print(listtwo)
```
### 在函数中接受元组和字典
使用`*`和`**`作为元组和字典的前缀
### `assert`语句
`assert`用于断言某事是真的  
如非常去定某个列表中至少包含一个元素，并想确认这一点，如果不是真的，就抛出一个错误`AssertionError`
```python
>>> mylist = ['item']
>>> assert len(mylist) >= 1
>>> mylist.pop()
'item'
>>> assert len(mylist) >= 1
AssertionError
```
### 装饰器(Decorators)
应用包装函数的快捷方式，这有助于将某一功能与一些代码一遍又一遍的"包装"  
案例：创建一个`retry`装饰器[`decorator_retry.py`]()
