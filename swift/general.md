# Swfit

#### 可选类型
- 声明可选类型  
  ```swift
  var optionalInteger: Int?
  // or
  var optionalInteger: Optional<Int>
  var optionalArray: (Int[])?
  ```

- 强制解析 forced unwrapping  
  ```swift
  var myString: String?
  myString = "hello"
  if myString != nil {
    print(myString!)
  } else {
    print("is nil")
  }
  ```

- 自动解析  
可以在声明可选变量时使用感叹号!替换问好?，他会自动解析  
```swift
var mystring: String!
mystring = "world"

if mystring != nil {
  print(mystring)
} else {
  print("nil")
}
```

- 可选绑定(optional binding)  
来判断可选类型是否包含值，如果包含就把值赋给一个临时常量或变量。可选绑定可以用在if和while语句中来对可选类型的值进行判断并把值赋给一个常量或者变量  
```swift
var mystring: String?

mystring = "haha"

if let yourstring = mystring {
  print(yourstring)
} else {
  print('iiiii')
}
```
  
#### 常量
使用`let`关键词  

#### 区间  
1...5  =>  1,2,3,4,5  
1..<5  =>  1,2,3,4

#### 循环  
- for..in..  
- while..  
- repeat..while..  
- 循环控制语句  
  - continue  
  - break  
  - fallthrough: 如果在一个case执行结束之后，继续执行下面的case，需要使用fallthrough（贯穿）关键词  

#### 字符串  
- 使用字符串属性`isEmpty`来判断  
- 字符串的长度`string.characters.count`  

#### 函数类型  
```swift
func sum(a: Int, b: Int) {
  return a + b
}
var additiona: (Int, Int) -> Int = sum
print(additional(1, 2))
```

#### 闭包
全局函数和嵌套函数其实就是特殊的闭包  
- 全局函数  
  有名字，但是不能捕获任何值  
- 嵌套函数  
  有名字也能封闭函数内的值  
- 闭包表达式  
  无名闭包，使用轻量级语法，可以根据上下文环境捕获值  

swift中有很多优化的地方  
- 根据上下文推断参数和返回值类型  
- 从单行表达式闭包中隐式返回（也就是闭包只有一行代码，可以省略return）  
- 可以使用简化参数名，如$0, $1  
- 提供尾随闭包语法(Trailing closure syntax)  

```swift
{(Int, Int) -> Bool in
  Statement1,
  Statement2,
  ...
}
```
#### 结构体
在代码中，可以使用结构体来自定义数据类型  
结构体实例总是值传递  
符合结构体的条件：  
- 结构体的主要目的是用来封装少量相关简单数据值  
- 有理由预计一个结构体实例在复制或者传递时，封装的数据会被拷贝而不是被引用  
- 任何在结构体中存储的值类型属性，也会被拷贝，而不是被引用  
- 结构体不需要去继承另一个已存在类型的属性或行为  

举个例子🌰  
- 几何形状的大小，封装一个width属性和height属性，两者均为Double类型  
- 一定范围内的路径，封装一个start属性和length属性，两者均为Int类型  
- 三维坐标系内一点，封装x,y和z属性，三者均为Double类型  

```swift
struct markStruct {
  var mark1: Int
  var mark2: Int
  var mark3: Int

  init(mark1: Int, mark2: Int, mark3: Int) {
    self.mark1 = mark1
    self.mark2 = mark2
    self.mark3 = mark3  
  }
}

var marks = markStruct(mark1: 98, mark2: 96, mark3: 100)
```

#### 类
Swift类是构建代码所用的一种通用且灵活的构造体  

##### 类 vs 结构体  
共同点：  
  1. 定义属性用于存储值  
  2. 定义方法用于提供功能  
  3. 定义附属脚本用于访问值  
  4. 定义构造器用于生成初始化值  
  5. 通过扩展以增加默认实现的功能  
  6. 符合协议以对某类提供标准功能    

类特有：  
  1. 继承允许一个类继承另一个类的特征  
  2. 类型转换允许在运行时检查和解释一个类实例的类型  
  3. 解构器允许一个类实例释放任何其所被分配的资源  
  4. 引用计数允许对一个类的多次引用  
```swift
class student {
  var studname: String
  var mark: Int
  var mark2: Int
}
// instantiation 
let studrecord = student()
// visit class attributes
studrecord.studname
```

##### 恒等运算符  
因为类是引用类型，有可能有多个常量和变量在后台同时引用同一个类实例  
可以用恒等运算符(`===`)判断是否引用同一个类实例  

#### 属性  
swift属性将值和特定的类、结构或者枚举关联    
属性分存储属性和计算属性：  
- 存储属性： 存储常量或作为实例的一部分，用于类和机构体  
- 计算属性： 计算(而不是存储)一个值，用于类、结构体和枚举  

##### 存储属性
一个存储属性就是存储在特定类或结构体的实例里的一个常量或变量  
存储属性可以是变量存储属性(var定义)，也可以是常量存储属性(let定义)  
```swift
struct Number {
  var digits: Int
  let po = 3.1415
}

var n = Number(digits: 12345)
n.digits = 67
```

##### 延迟存储属性  
延迟属性是指当第一次被调用时才会计算其初始值的属性  
在属性声明前使用`lazy`来标识一个延迟存储属性  
```swift
class sample {
  lazy var no = number()
}

class number {
  var name = "Runoob Swift"
}

var firststamp = sample()
```

##### 计算属性 
```swift
class sample {
  var no1 = 0.0, no2 = 0.0
  var length = 300.0, breath = 150.0
  
  var middle: (Double, Double) {
    get {
      return (length / 2, breadth / 2)
    }
    set(axis) {
      no1 = axis.0 - (length / 2)
      no2 = axis.1 - (breadth / 2)
    }
  }
}

var result = sample()
print(result.middle)
result.middle = (0.0, 10.0)

print(result.no1)
print(result.no2)
``` 

##### 只读属性
只有`getter`没有`setter`的属性就是制度属性  

##### 属性观察期
属性观察器监控和响应属性值的变化，每次属性被设置值时都会调用属性观察器，即使值没有变化  
可以为除了延迟属性之外的其他存储属性添加属性观察器，也可以通过重载属性的方式为继承的属性(包括存储属性和计算属性)添加属性观察器  
> 不需要为无法重载的计算属性添加属性观察器，因为可以通过setter直接监控和响应值的变化  


