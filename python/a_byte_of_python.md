**老张，多看看**
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
