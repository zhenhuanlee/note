<!-- # Go [反射三定律](https://segmentfault.com/a/1190000006190038)
#### 第一定律
反射可以将'借口类型变量'转换为'反射类型对象'  
> 这里反射类型指`reflection.Type`和`reflection.Value`  
从用法上来说，反射提供了一种机制，允许程序在运行时检查接口变量内部的(value, type)对。  
reflection的两种类型:`type`和`value`使访问接口内的数据成为可能。他们对应两个最简单的方法，分别是`reflection.TypeOf`和`reflection.ValueOf`，分别用来读取接口变量的`reflection.Type`和`reflection.Value`部分。当然，从`reflection.Value`也很容易获取到`reflection.Type`。  

- `reflection.TypeOf`  
```go
var x float64 = 3.4
fmt.Println(reflect.TypeOf(x)) // float64
// x被存储在一个空接口变量中传递过去，然后reflect.TypeOf对空接口变量进行了拆解，恢复其类型信息
```

- `reflect.ValueOf`  
```go
x := 3.4
fmt.Println(reflect.ValueOf(x)) // 3.4
```

他们还有很多的方法：  
```go
x := 3.4
v := reflect.ValueOf(x)
fmt.Println(v.Type()) // float64
fmt.Println(v.Kind() == reflect.Float64) // true
fmt.Println(v.Float()) // 3.4
```

反射库提供了很多有意义的属性，举例`Value`的`getter`和`setter`  
> 为了保证api的精简，这两个方法操作的是某一组类型范围最大的那个。如任何整形数，都是用int64。也就是说Value类型的Int方法返回值为int64类型  
```go
var x uint8 = 'x'
v := reflect.ValueOf(x)
fmt.Println(v.Type()) // uint8
fmt.Println(v.Kind() == reflect.Uint8) // true
fmt.Println(reflect.TypeOf(v.Uint)) // func() uint64
// 此处已经变成uint64了
```

**反射类型变量的`kind`方法会返回底层数据类型，而不是静态类型**  
```go
type MyInt int
var x MyInt = 7
v := reflect.ValueOf(x)
fmt.Println(v.Kind())          // int
fmt.Println(v.Type())          // main.MyInt
fmt.Println(reflect.TypeOf(x)) // main.MyInt
// 虽然变量V的 ‘静态类型’是MyInt，但是Kind方法仍然返回reflect.Int
// type方法不会像Type方法一样区分MyInt和Int
```

#### 第二定律
反射可以将"反射类型对象"转换为"接口类型变量"  
go语言中的反射可以创造自己反面类型的对象  

根据一个`reflect.Value`类型的变量，我们可以使用`Interface()`方法恢复其接口类型的值。事实上，这个方法会把`type`和`value`信息打包并填充到一个接口变量中，然后返回  
`func (v Value)Interface() interface{}`  

然后我们可以通过断言，恢复底层的具体值  
```go
var x float64 = 100
v := reflect.ValueOf(x)
y := v.Interface()
fmt.Println(y)           // 100  print会恢复数据类型
fmt.Println(y.(float64)) // 100
```
> 简单来说，就是`Interface()`和`ValueOf()`作用恰好相反，返回值的静态类型是`interface{}`  

#### 第三定律
如果要修改'反射类型对象'，其值必须是'可写的'(settable)  
```go
var x float64 = 3.4
v := reflect.ValueOf(x)
v.SetFloat(8.1)
// 抛出了一个异常
// panic: reflect.Value.SetFloat using unaddressable value
// 这个异常的原因是变量v是"不可写的"
// “可写性”是反射类型变量的一个属性，但不是所有的反射类型变量都拥有这个属性
```
可以用`CanSet()`方法检查一个`reflect.Value`类型变量的'可写性'.  
```go
var x float64 = 3.4
v := reflect.ValueOf(x)
fmt.Println(v.CanSet()) // false
```
"可写性"有些类似于寻址能力，但是更严格。他是反射类型变量的一种属性，赋予改变量修改底层存储数据的能力。“可写性”最终是由一个事实决定：反射对象是否存储了原始值  
基础数据类型(如`int`)传递给`reflect.ValueOf()`的是一个x的拷贝。所以即使更新成功了，也不会修改原始变量的值  
所以可以传递指针  
```go
var x = 3.4
p := reflect.ValueOf(&x)
fmt.Println(p.Type()) // *float64
fmt.Println(p.CanSet()) // false
v := p.Elem()
fmt.Println(v.CanSet()) // true
v.SetFloat(9.1)
fmt.Println(v) // 9.1
```
 -->

# 反射
#### API
```go
type dog struct {
	Name   string
	Age    int
	Parent *dog
}

func (dog) Say(sth string) string {
	// fmt.Println("wang")
	return sth
}

a := dog{Name: "haha", Age: 100}
	t := reflect.TypeOf(a)
	v := reflect.ValueOf(a)
	v2 := reflect.ValueOf(&a)

	fmt.Println(t.Name())                    // dog
	fmt.Println(t.Kind())                    // struct
	fmt.Println(t.NumField())                // 3
	fmt.Println(t.NumMethod())               // 1
	fmt.Println(t.Field(0))                  // {Name main string  0 [0] false}
	fmt.Println(t.Method(0))                 // {Say  func(main.dog) <func(main.dog) Value> 0}
	fmt.Println(t.FieldByIndex([]int{2, 1})) // {Age main int  16 [1] false}，嵌套结构，第2个的第一个属性
	fmt.Println(t.MethodByName("Say"))       // {Say  func(main.dog) <func(main.dog) Value> 0} true
  fmt.Println(t.Field(0).Tag.Get("json"))  // tagName
	m, _ := t.MethodByName("Say")
  fmt.Println(m.Func.Call([]reflect.Value{reflect.ValueOf(dog{}), reflect.ValueOf("laifu")})) // [laifu]

	fmt.Print("\nxxxxxxxxxxxxxnxxxxxxxxxxxxxnxxxxxxxxxxxxx\n\n")

	fmt.Println(t == v.Type())                                                           // true
	fmt.Println(v.Type())                                                                // main.dog
	fmt.Println(v.Type().Name())                                                         // dog
	fmt.Println(v.Field(0))                                                              // haha
	fmt.Println(v.Field(0).Interface())                                                  // haha, 转换成正射
	fmt.Println(v.Method(0))                                                             // 0x107c2b0
	fmt.Println(v.MethodByName("Say"))                                                   // 0x107c2b0
	fmt.Println(v.MethodByName("Say").Type())                                            // func(string)
	fmt.Println(v.MethodByName("Say").Kind())                                            // func
	fmt.Println(v.MethodByName("Say").Type().NumIn())                                    // 需要的参数个数，这里需要1个参数
	fmt.Println(v.MethodByName("Say").Type().Out(0))                                     // string 第一个返回值的类型
	fmt.Println(v.Interface())                                                           // {haha 100 <nil>}
	fmt.Println(v.Kind() == t.Kind())                                                    // true
	fmt.Println(v.MethodByName("Say").Call([]reflect.Value{reflect.ValueOf("wangcai")})) // [wangcai]

	fmt.Print("\nxxxxxxxxxxxxxnxxxxxxxxxxxxxnxxxxxxxxxxxxx\n\n")

	fmt.Println(v2.Elem())                           // {haha 100 <nil>}, 获取value中的值value
	fmt.Println(v2.Elem().CanSet())                  // true, 检查当前value中的值是否可以改变
	v2.Elem().FieldByName("Name").SetString("nihao") //
	fmt.Println(v2.Elem().FieldByName("Name"))       // nihao
	fmt.Println(v2.IsValid())                        // true, 用得少

```

#### 助记口诀
- 有了对象就有了对象的Type  
- 有了对象就有了对象的Value  
- 对象的Value和指针的Value可以互转  
- 正反(射)值可以互转  

