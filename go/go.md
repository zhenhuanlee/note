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
