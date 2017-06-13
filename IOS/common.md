#### Cocoa Touch
- Multi-Tpuch Alerts
- Core Motion Web View
- View Hierarchy Map Kit
- Localization Image Picker
- Controls Canera


#### let & var
- let 常量，一旦定义，不会改变


#### !
! 用来对optional解包


#### 闭包
performOperation({ $0, $1})


#### 划重点


#### String, Double, Dictionary ,Tuple... 是结构体，他们和类的区别是
- struct cannot inherited  
- 结构体传的是值，而类传递的是引用  

#### Optional
简单的enum，是泛型
```swift
enum Optional<T> {
  case None
  case some(T)
}

let x: String? = nil
...is...
let x = Optional<String>.None

let x: String? = "hello"
...is...
let x = Optional<String>.some("hello")

var y = x!
...is...
switch x {
  case Some(let value): y = value
  case None: // raise an exception
}
```

#### Range
```swift
struct Range<T> {
  var startIndex: T
  var endIndex: T
}
```
```swift
let array = ["a", "b", "c", "d"]
let subArray1 = array[2..3]    // ["c", "d"]
let subArray2 = array[2..<3]   // ["c"]
for i in [27..104] {}             
```

#### NSObject
Base class for all Object-C classes

#### NSNumber
Generic number-holding classes
let n = NSNumber(35.5)
let intversion = n.intValue // alse doubleValue, boolValue, etc.

#### NSDate
Used to find out the date and time right now or store past or future dates  
See alse NSCalender,NSDateFormatter,NSDAteComponents  
If you are displaying a date in your UI, there are localization ramifications

#### NSData
无类型数据，可以看做一个指向内存的指针，这是IOS可以四处传递无类型数据、原始数据的原型

#### 三个基础的结构
Classes  Structures  Enumerations
##### Similarities
- Declareation syntax  
```swift
class CalulatorBrain {

}
struct Vertex {

}
enum Op {

}
```
- Properties and Functions  
```swift
func doit(argument: Type) -> ReturnValue {

}

var storeProperty = <initial value> (not enum)

var computedProperty: Type {
  get {}
  set {}
}
```
- Initializers(again, not enum)
```swift
init(argument1: Type, argument2: Type, ...) {

}
```
##### Differences
- Inheritance(class only)  
- Introspection and casting(class only) // 内省和类型转换  
- Value type(struct, enum) vs. Reference type(class)   
  类属于引用类型，传递的是这些对象的指针  
  值类型就是一份拷贝，如:`let x = Array<>`  `var y = x`

#### Property Observers
change any property with `willSet` and `disSet`
```swift
var simeStoredProperty: Int = 42 {
  willSet { newValue is the new value }
  disSet { oldValuee is the old value }
}

override var inheritedProperty {
  willSet { newValue is the new value }
  disSet { oldValue is the old value }
}
```
懒加载
```swift
lazy var brain = CalculatorBrain()  // nice if CalculatorBrain used lots of resources
// only var
```
