# 面向函数式编程
> 简单例子  
>
```js
    function greet(name) {
        return "Hi, I'm " + name
    }

    greet("lee")
```

#### 如何函数式
- 纯函数就是给定输入，给出输出，输入确定，输出就确定，如上面的例子  
- 避免副作用  
函数的输出不完全由输入决定，比如有全局变量的存在  
- 不要使用`for`, `while`  
可以使用`map`, `reduce`, `filter`等高阶函数，将想要的操作当作参数传进去  
- 避免数据变异  
```js
var rooms = ["h1", "h2", "h3"];
var newRooms = rooms.map(function(rm) {
    if (rm == "h3") { return "h4"; }
    else { return rm; }
});
```
- 上面的方法会发生内存拷贝，比较好的办法是用持久化的数据结构，复用老数据  
试着了解`mori.js`库，实际上使用的是闭包  
同样的还有个`mutable.js`  


#### 高阶函数(闭包？)  
```js
function makeAdjectifier(adjective) {
    return function(string) {
        return adjective + " " + string;
    };
}

var coolifier = makeAdjectifier("cool");
coolifier("conference")
```