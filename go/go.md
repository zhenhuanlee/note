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
  ```go
  s1 := int[]{1,2,3,4}
  s2 := int[]{4,5,6}
  copy(s1, s2)
  fmt.Println(s1) // => [4 5 6 4]
  copy(s2, s1)
  fmt.Println(s1) // => [1 2 3]
  ```

#### map
- key-value对  
- key必须是支持`==`比较运算的类型，不可以是函数，map或者slice  
- map查找比线性搜索快很多，但是比索引(数组/slice 下标)访问数据的类型慢100倍  
- map使用`make()`创建，支持`:=`简写方式  
  - `make([keyType]valueType, cap)`cap表容量，可省略  
- 超出容量会自动扩容，但尽量提供一个合适的初始值  
- 使用`len()`获取元素个数  
- 键值对不存在时自动添加，使用`delete()`删除  
- 使用`for range`对map和slice进行迭代操作  

```go
var m map[int]map[int]string
m = make(map[int]map[int]string)
a, ok = m[2][1]
fmt.Println(a, ok)
// 多返回值，第二个返回值是bool，判断键值对存不存在  
// 上面的情况就是不存在的，作为value的map没有初始化  
```

```go
sm := make([]map[int]string, 5)
for _, v := range sm {
  v := make([int]string, 1)
  v[1] = "OK"
  fmt.Println(v)
}
fmt.Println(sm)
// 因为v只是一个拷贝，所以并不是改变sm  
```

#### 函数function
- Go函数不支持嵌套，重载和默认参数  
- 支持：无需声明原型，不定长度变参，多返回值，命名返回值参数，匿名函数，闭包  
- 函数也可以作为一种类型使用  
- `defer`析构函数
  - 即使函数发生严重错误也会执行  
  - 支持匿名函数的调用  
  - 常用于资源清理，文件关闭，解锁以及记录时间等操作  
  - 通过与匿名函数的配合可在return之后修改函数的计算结果  
  - 如果函数体内某个变量作为`defer`时匿名函数的参数，则在定义defer时即已经获得了拷贝，否则则是引用某个变量的地址  
  - Go没有异常机制，但有`panic/recover`模式来处理错误  
  - Panic可以再任何地方引发，但recover只有在defer调用的函数中有效  

```go
fmt.Println("a")
defer fmt.Println("b")
defer fmt.Println("c")
// a c b
for i := 0; i < 3; i++ {
  defer func() {
    fmt.Println(i)
  }() // ()调用这个函数
}
// 3 3 3 
// 闭包的特性，i一直是个引用，执行的时候已经是3了
```

```go

func A() {
	fmt.Println("func a")
}

func B() {
	defer func() {
		if err := recover(); err != nil {
			fmt.Println("recover in b")
		}
	}()
	panic("panic in b")
}

func C() {
	fmt.Println("fun c")
}

A()
B()
C()
// func a
// recover in b
// fun c
```

#### 结构struct
- Go的struct与C的struct非常相似，且Go没有class  
- 使用type<name>struct()定义结构，名称遵循可见性规则  
- 支持指向自身的指针类型成员  
- 支持匿名结构，可用作成员或定义成员变量  
- 匿名结构也可以用于map的值  
- 可以使用字面值对结构进行初始化  
- 允许通过指针来读写结构成员  
- 相同类型的成员可进行直接拷贝赋值  
- 支持==与!=比较运算，但不支持>或<  
- 支持匿名字段，本质上是定义了以某个类型名为名称的字段  
- 嵌入结构作为匿名字段看起来像继承，但并不是  
- 可以使用匿名字段指针  

```go
type person struct {
	Name string
	Age  int
}

func main() {
	a := person{}
	a.Name = "a"
	a.Age = 14
	fmt.Println(a)
}

a := person{
  Name: "a",
  Age:  19,
}
```
结构体的指针操作时，可以直接操作不用取值`*`  
初始化时就应该使用指针，否则是值拷贝  

```go
// 匿名结构
a := &struct {
  Name string
  Age  int
}{
  Name: "adf",
  Age:  19,
}
fmt.Println(a)
```

```go
// 嵌套匿名结构
type person struct {
	Name    string
	Age     int
	Contact struct {
		Phone, City string
	}
}

func main() {
	a := person{Name: "aaa", Age: 12}
	a.Contact.Phone = "123123"
	a.Contact.City = "NJ"
	fmt.Println(a)
}
```

```go
//匿名字段
type person struct {
	string
	int
}

func main() {
	a := person{"aaa", 12}
	fmt.Println(a)
}
```

```go
// 嵌入结构(相当于继承，但不是)
type student struct {
	human
	Name string
	Age  int
}

func main() {
	a := student{
		Name: "JOE",
		Age:  10,
    humane: human{Sex: 0}
	}
  a.human.Sex = 123
	a.Sex = 123
	fmt.Println(a)
}
```

#### 方法method
- Go中虽然没有class，但依旧有method  
- 通过显示说明receiver来实现与某个类型的组合  
- 只能为同一个包中的类型定义方法  
- Receiver可以是类型的值或者指针  
- 不存在方法重载  
- 可以使用值或指针来调用方法，编译器会自动完成转换  
- 从某种意义上来说，方法是函数的语法糖，因为Receiver其实就是方法所接收的第一个参数(method value VS. method expression)  
- 如果外部结构和嵌入结构存在同名方法，则优先调用外部结构的方法  
- 类型别名不会拥有底层类型所附带的方法  
- 方法可以调用结构中的非公开字段  

```go
type A struct {
  Name string
}

func (a A) Print() {
	fmt.Println("A")
}

func main() {
	a := A{}
  a.Print() // method value
	A.Print(a) // method expression
	(*A).Print(&a)
}
```

```go
// 访问私有变量
type A struct {
  name string // 私有
}

func main() {
  a := A{}
  a.Print()
  fmt.Println(a.name) // 访问了私有字段
  // 因为私有只是相对包而言
}

func (a *A) Print() {
  a.name = "123"
  fmt.Println(a.name) // 访问了私有字段
}
```

#### 接口interface
- 接口是一个或多个方法签名的集合  
- 只要某个类型拥有该接口的所有方法签名，即算实现该接口，无需显示声明实现了哪个接口，这成为Structural Typing  
- 接口只有方法声明，没有实现，没有数据字段  
- 接口可以匿名嵌入其他接口，或嵌入到结构中  
- 将对象赋值给接口时，会发生拷贝，而接口内部存储的是指向这个复制品的指针，既无法修改复制品的状态，也无法获取指针  
- 只有当接口存储的类型和对象都为nil时，接口才等于nil  
- 接口调用不会做receiver的自动转换  
- 接口同样支持匿名字段方法  
- 接口也可以实现类似OOP中的多态  
- 空接口可以作为任何类型数据的容器  

```go
type USB interface {
	Name() string //name方法，返回一个字符串
	Connect()     //没有返回值
}

type PhoneConnecter struct {
	name string
}

func (pc PhoneConnecter) Name() string {
	return pc.name
}

func (pc PhoneConnecter) Connect() {
	fmt.Println("Connect:", pc.name)
}

func Disconnect(usb USB) {
	fmt.Println("Disconnected")
}

func main() {
	a := PhoneConnecter{"PhoneConnecter"} // 实现了USB接口
	a.Connect()
	Disconnect(a)
}
```
##### 嵌入接口
```go
type Connecter interface {
	Connect()
}

type USB interface {
	Name() string //name方法，返回一个字符串
	Connecter     // 嵌入了
}
```
##### 类型断言 ok pattern
```go
func Disconnect(usb USB) {
  if pc, ok := usb.(PhoneConnecter); ok { // 类型断言 判断是不是PhoneConnecter类型
		fmt.Println("Disconnected:", pc.name) 
		return
	}
	fmt.Println("Unknown device.")
}
```

##### type switch 
```go
// 当类型比较多时
switch v := usb.(type) {
case PhoneConnecter:
  fmt.Println("xxx", v.name)
default:
  fmt.Println("yyy")
}
```

##### 接口转换
只能向下转换
```go
type USB interface {
	Name() string //name方法，返回一个字符串
	Connecter     // 嵌入了
}

type Connecter interface {
	Connect()
}

type PhoneConnecter struct {
	name string
}

func main() {
	pc := PhoneConnecter{"PhoneConnecter"}
	var a Connecter
	a = Connecter(pc) // 强制类型转换
	a.Connect()
}
```

#### 反射 reflection
```go
package main

import (
	"fmt"
	"reflect"
)

type User struct {
	Id   int
	Name string
	Age  int
}

func (user User) Hello() {
	fmt.Println("Hello")
}

func main() {
	u := User{1, "OK", 12}
	Info(u)
}

func Info(o interface{}) { // 参数类型是一个空接口
	t := reflect.TypeOf(o)
	fmt.Println("Type: ", t.Name())
	v := reflect.ValueOf(o)
	fmt.Println("Fields:")

	for i := 0; i < t.NumField(); i++ {
		f := t.Field(i)
		val := v.Field(i).Interface()
		fmt.Printf("%6s: %v = %v\n", f.Name, f.Type, val)
	}

  for i := 0; i < t.NumMethod(); i++ {
		m := t.Method(i)
		fmt.Printf("%6s: %v\n", m.Name, m.Type)
	}

}
```

##### 反射匿名或嵌入字段
```go
package main

import (
	"fmt"
	"reflect"
)

type User struct {
	Id   int
	Name string
	Age  int
}

func main() {
	u := User{1, "OK", 12}
	Set(&u)
	fmt.Println(u)
}

func Set(o interface{}) {
	v := reflect.ValueOf(o)

	if v.Kind() == reflect.Ptr && !v.Elem().CanSet() {
		fmt.Println("xxx")
	} else {
		v = v.Elem()
	}
	if f := v.FieldByName("Name"); f.Kind() == reflect.String {
		f.SetString("Bye")
	}
}
```

##### 如何通过反射调用方法
```go
package main

import (
	"fmt"
	"reflect"
)

type User struct {
	Id   int
	Name string
	Age  int
}

func main() {
	u := User{1, "OK", 12}
	t := reflect.ValueOf(u)
	mv := t.MethodByName("Hello")

	args := []reflect.Value{reflect.ValueOf("Joe")}
	mv.Call(args)
}

func (u User) Hello(name string) {
	fmt.Println("hello", name, "my name is", u.Name)
}
```


#### 并发concurrency
- 从源码解析来看，goroutine只是由官方实现的超级"线程池"而已，每个实例4-5KB的内存占用，大幅减少创建和销毁的开销，是制造Go号称高并发的根本原因，另外goroutine的简单易用，也在语言层面上给与了开发者巨大的便利  
- 并发主要由切换时间片来实现，并行则是直接利用多核实现多线程的运行，但Go可以设置使用核数，以发挥多核计算机的能力  
- Goroutine奉行通过通信来共享内存，而不是共享内存来通信  

##### channel
- channel是goroutine沟通的桥梁，大都是堵塞同步的  
- 通过`make`创建，`close`关闭  
- channel是引用类型  
- 可以使用for range来迭代不断操作channel  
- 可以设置单向或者双向通道  
- 可以设置缓存大小，在未被填满前不会发生堵塞  

##### select
- 可以处理一个或多个channel的发送与接收  
- 同时有多个可用的channel时按随机顺序处理  
- 可用空的select来阻塞main函数  
- 可以设置超时  

```go
package main

import (
	"fmt"
	// "time"
)

func main() {
	c := make(chan bool)
	go func() {
		fmt.Println("Go!")
		c <- true
	}()
	<-c // 运行到此堵塞，等待线程中的传入
}

```

```go
package main

import (
	"fmt"
	"runtime"
	"sync"
	// "time"
)

func main() {
	runtime.GOMAXPROCS(runtime.NumCPU())
	wg := sync.WaitGroup{}
	wg.Add(10)

	for i := 0; i < 10; i++ {
		go Go(&wg, i)
	}

	wg.Wait()
}

func Go(wg *sync.WaitGroup, index int) {
	a := 1
	for i := 0; i < 100000000; i++ {
		a += 1
	}
	fmt.Println(index, a)

	wg.Done()
}
```

```go
// select
package main

import (
	"fmt"
	// "runtime"
	// "sync"
	// "time"
)

func main() {
	c1, c2 := make(chan int), make(chan string)
	o := make(chan bool)
	go func() {
		for {
			select {
			case v, ok := <-c1:
				if !ok {
					o <- true
					break
				}
				fmt.Println("c1", v)
			case v, ok := <-c2:
				if !ok {
					o <- true
					break
				}
				fmt.Println("c2", v)
			}
		}
	}()

	c1 <- 1
	c2 <- "hi"
	c1 <- 3
	c2 <- "hel"

	close(c1)
	close(c2) // 为了不复杂化并发程序，应该在c1关闭时就退出select，所以这行应该删掉  

	<-o
}
```
```go
// select 2
package main

import (
	"fmt"
)

func main() {
	c := make(chan int)

	go func() {
		for v := range c {
			fmt.Println(v)
		}
	}()

	for i := 0; i < 10; i++ {
		select {
		case c <- 0:
		case c <- 1:
		}
	}
}

```

```go
// select 设置超时
package main

import (
	"fmt"
	"time"
)

func main() {
	c := make(chan bool)
	select {
	case v := <-c:
		fmt.Println(v)
	case <-time.After(3 * time.Second): // time.After()返回的是一个(chane time)
		fmt.Println("timeout")
	}
}

```
