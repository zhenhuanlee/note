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
mystring = world""

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
```swift
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

