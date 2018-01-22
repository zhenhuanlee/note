# Go编程基础  
### 什么是Go
并发支持，垃圾回收的编译型系统编程语言，旨在创造一门具有静态编译语言的高性能的动态语言的高效开发之间拥有良好平衡的一门编程语言  

#### 特点
- 类型安全和内存安全  
- 以非常直观的方式和极低代价的方案实现高并发  
- 高效的垃圾回收  
- 快速编译(同时解决了C语言中头文件太多的问题)
    编译时会判断引入的包有没有被使用  
- 为多核计算机提供性能提升的方案
    可以指定核心数    
- UTF-8编码支持  

#### Go存在的价值
服务端程序开发的语言  

#### Go的内置关键字  
break  default  func   interface   select   case  defer  go  map  struct   chan  else  goto  package  switch  const  fallthrough  if  range  type  continue  for   import  return  var  

#### Go程序的一般结构  
- Go程序是通过package来组织的  
- 只有package名称为main的包可以包含main函数  
- 一个可执行程序有且只有一个main包  
- 通过import关键字来导入其他非main包  
- 通过const关键字来定义常量  
- 通过在函数体外使用var关键字来进行全局变量的声明与赋值  
- 通过type关键字来进行结构(struct)或接口(interface)的声明  
- 通过func关键字来进行函数的声明  

```go
// 当前程序的包名
package main

// 导入其他的包
import "fmt"
// 帮引入的包别名  
import std "fmt"
// 省略调用，这样就可以不用写包名了
import . "fmt"

// 常量的定义
const PI = 3.14

// 全局变量的声明与赋值
var name = "gopher"

// 一般类型声明
type newType int

// 结构的声明
type gopher struct{}

// 接口的声明
type goloang interface{}

// 由main函数作为程序入口点启动
func main() {
  ...
}
```

#### 可见性规则
- Go语言中，使用大小写来决定该 常量，变量，类型，接口，结构，函数是否可以被外部包所调用：  
    - 小写即为 **private**  
    - 大写即为 **public**  

#### Go基本类型  
- bool
  - 长度： 1字节  
  - 取值范围： true, false  
  - 注意事项： 不可以用数字代表true false  
- 整形: int/ uint(无符号整形)    
  - 根据运行平台可能是32或64位  
- 8位整形: int8/ uint8  
  - 长度： 1字节  
  - 取值范围： -128~127 / 0~255
- 字节型： byte(uint8 别名)  
- 浮点型： float32 / float64  
  - 长度： 4/8字节  
  - 小数位： 精确到7/15位小数  
- 复数： complex64 / complex128  
- 足够保存指针的32位或64位证书: uintptr
- 其他值类型  
  - array  struct  string  
- 接口类型： interface  
- 引用类型  
  - slice  map  chan  
- 函数类型: func  

#### 类型零值  
通常并不等于空值，而是当变量被声明为某种类型后的默认值  
  - 值类型的默认值 0
  - bool的默认值 false  
  - string的默认值  空字符串  

#### 变量声明  
```swift
var (
  aaa = "hello"
  sss, bbb = 1, 2
)
//  局部变量不能用var()方式简写  
var a, b, c, d int
a, b, c, d = 1, 2, 3, 4
var e, f, g, h int = 5, 6, 7, 8
var i, j, k, l = 1, 2, 3, 4
i, m, q := 1, 2, 3, 4
```
#### 变量的类型转换  
- Go中不存在隐式转换，所有类型必须显示声明  
- 转换只能发生在两种相互兼容的类型之间  
- 类型转换的格式  
```swift
//  <TypeOfValueA>(<ValueB>)  

var a float32 = 1.1
b := int(a)
```

#### 常量与运算符  
- 常量使用关键词`const`  
- 如果在定义常量组时，如果不提供初始值，则表示将使用上行的表达式  
- iota是常量的计数器，从0开始，组中每定义一个常量，自动递增1  
- 通过初始化规则与iota可以达到枚举的效果  
- 每遇到一个const关键字，iota就会重置为0  
```swift
const (
	a = 'A'
	b
	c = iota
	d
)
a = 65
b = 65
c = 2
d = 3
```

#### 指针
Go虽然保留了指针，蛋不支持指针运算以及`->`运算，而直接采用`.`选择符来操作指针目标对象的成员  
- 操作符`&`取变量地址，使用`*`通过指针间接访问目标对象  
- 默认值为nil而非NULL  

#### if
```swift
if a := 1; a > 0 {
  ...
}
```

#### for
```swift
for {
  ...
}

for a < 1 {
  ...
}

for i := 1; i < 10; i++ {
  ...
}
```

#### switch
```swift
// 不用写break
// 有fallthrough语句  
switch a {
case 0:
  ...
case 1:
  ...
default:
  ...
}

switch {
case a == 0:
  ...
  fallthrough
case a > 0:
  ...
default:
  ...
}

switch a:=1; {
  ...
}
```

#### 跳转语句：goto  break  continue
- 三个语法都可以配合标签使用  
- `Break` & `Continue`配合标签可用于多层循环的跳出  
- `Goto`是调整执行位置，与其它两个语句配合标签的结果并不相同  
```go
func main() {
LABEL1:
	for {
		for i := 0; i < 10; i++ {
			if i > 3 {
				break LABEL1
			}
		}
	}
}
```

#### 数组 
- 定义   
```go
var a [2]int
a := [3]int{2:1} => [0 0 1]
a := [...]int{1, 2, 3}
a := [...]int{19: 1}
```
- 数组的指针
```go
a := [...]int{99: 1}
var p *[100]int = &a
fmt.Println(p) => &[0 0 0 0  ...]

// 返回一个指向数组的指针
p := new([10]int) => &[0 0 0 ...]
```
不管是数组本身还是指向数组的指针，都可以使用`[]`   

- 指针数组
```go
x, y := 1, 2
a := [...]*int{&x, &y}
fmt.Println(a)
```

- 数组是值类型，是值传递  
- 多维数组  
```go
a := [2][3]int{
  {1，1，1}
  {2，2，2}
}
```
#### 切片
- 其并不是数组，它指向底层的数组  
- 作为变长数组的替代方案，可以关联底层数组的局部或全部  
- 为引用类型  
- 可以直接创建或从底层数组获取生成  
- 使用`len()`获取元素个数，`cap()`获取容量  
- 一般使用`make()`创建  
- 如果多个slice指向相同底层数组，其中一个的值改变会影响全部  
- `make([]T, len, cap)`  
- 其中`cap`可以沈略，则和`len`值相同  
- `len`表示存数元素个数，`cap`表示容量  
> slice是指向底层数组的某一个位置，也就是所，如果一个slice是一个数组的一部分，那么即使数组下标越界，也能取到值  

```swift
//没有指定长度
var s1 []int

// 从数组中获取slice
a := [10]int{}
s1 := a[5:10] // a[5 6 7 8 9] except 10
s2 := a[5:]   // equal above
s3 := a[:5]

// 先分配10个连续的内存，如果超过10个就重新分配，每次增加一倍，也就是20, 40...
s1 := make([]int, 3, 10)
```
- Reskice()
  - `Reslice`时索引以被slice的切片为准  
  - 索引不可以超过被slice的切片的容量`cap()`的值  
  - 索引越界不会导致底层数组的重新分配而是引发错误  

- Append
  - 可以再slice尾部追加元素  
  - 可以将一个slice追加到另一个slice尾部  
  - 如果最终长度未超过追加到slice的容量，则返回原始slice  
  - 如果超过追加到的slice的容量，则将重新分配数组并拷贝原始数据  

- copy  
  - 
