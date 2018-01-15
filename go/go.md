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
