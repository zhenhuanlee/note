# TypeScript

## 数据类型
- boolean  
- number  
- string  
- array  
```typescript
  // var a = [1,2,3]   // es5  
  var arr:number[] = [1, 2, 3];
  var arr:Array<number> = [2, 3, 4];
  var arr:any[] = [1, "2"];
```
- tuple  属于数组的一种  
```typescript
  var arr:[number, string] = [123, "456"];
```
- enum  
```typescript
  enum Gender {man, woman};
  enum Gender {man=1, woman=2};

  let s:Flag = Flag.man
```
- any  
```ts
var num:any = 123;
num = "456";
```
- numm & undefined never数据类型的子类型  
```ts
var num:number; // num => undefined

var num:number | undefined;

var num:null;

// 一个元素可能是 null, undefined, number 
var num:number | null | undefined;
```

- void 方法没有返回值  
```ts
function run():void {
  console.log("no value");
}
```

- never 代表从不会出现的值，never 变量只能呗 never 类型所赋值  
```ts
var a:never;
a = (() => {
  throw new Error("error");
})()
```

## 函数  
```ts
// 普通函数
function f1():string {
  return "xf";
}

// 匿名函数
var f2 = function(namge:string, age:number):string {
  return `${name} -- ${age}`;
}

// 可选参数，可传可不传 关键词 ·?·
function f3(name:string, age?:number):void {}

// 默认参数
function f4(name:string="xx"):void {}

// 剩余参数
function f5(...a:number[]):void {}

// 函数重载，和 java 不同，es5不支持重写
function getInfo(name:string):string;
function getInfo(name:number, age:number):number;
function getInfo(str:any, age?:any):any{
    if (typeof str === "string") {
        return "x";
    } else {
        return 20;
    }
};
getInfo("xx");
getInfo(20);
getInfo(true); // 错误

// 箭头函数, 箭头函数中的 this 指向上下文
setTimeout(() => {
  console.log(123)
}, 1000);
```

## 类，静态方法和继承 (这里有es5的**继承**demo)
- es5中实现一个类的构造函数，属性，实例方法，静态方法(类方法)    
```js
function Person() {
  // 在构造函数中声明属性和实例方法
  this.name = "san";
  this.run = function() {  // 实例方法
    //xxx
  };
}

// 在原型链中声明属性和实例方法
Person.prototype.sex = "nan";
Person.prototype.say = function() {}; // 实例方法

var p = new Person();
p.sex;

Person.getInfo = function() {};  // 静态方法
```

#### es5中的 "继承" 实现     
```js
/* 继承 1
   "对象 冒充"的继承模式
*/
function Web() {
  /* 对象冒充继承模式
   - 对象冒充实现继承，可以继承构造函数里面的属性和方法
   - 但是不能继承原型链的  
  */
  Person.call(this); 
}
var w = new Web();
w.run();  // 会找 Person 的 run 方法

/* ------------------------------- */

/* 继承 2
  "原型链" 实现继承
*/
function Web() {};
/* 原型链继承
  - 既可以继承构造函数里面的属性和方法
  - 但是，prototype指向一个实例，这个感觉怪怪的，比如构造函数要传参啥的，不太好  
*/
Web.prototype = new Person();


/* 继承 3
  原型链 + 构造函数 的组合继承模式 
*/ 
function Person(name, age) {
  this.name = name;
  this.age = age;
}

function Web(name, age) {
  Person.call(this, name, age) // 对象冒充实例化子类可以给父类传参
}
Web.prototype = new Person();

var w = bew Web("xx", 20);

// 原型链 + 冒充 的另一种形式，我觉得这个才是最好的
function Web(name, age) {
  Person.call(this, name, age);
}
Web.prototype = Person.prototype;
```

#### typescript 中的类
```ts
class Person {
  name:string; // 属性，前面省略了 public 关键词

  constructor(name:string) { // 构造函数
    this.name = name;
  }

  run():void {
    console.log(this.name)
  }
}

var p = new Person('三');
p.run()
```

#### typescript 中实现继承
```ts
class Person {
    name:string;

    constructor(name:string) {
        this.name = name;
    }

    run():void {
        console.log(this.name)
    }
}

class Man extends Person {
    age:number;

    constructor(name:string, age:number) {
        super(name);
        this.age = age;
    }

    run():void {
        super.run();
        console.log(this.age)
    }
}

var m = new Man("li", 20);
m.run();
```

#### ts 范围修饰符
- public(default): 本类，子类，外部  
- protected: 本类，子类  
- private: 只有本类  


#### ts 静态方法  
```ts
class Person {
    public static gender:string = 'man';  // 静态变量
    public static print() { // 静态方法
        console.log(`this person is ${this.gender}`);
    }
}

Person.print();
```


#### 抽象方法和抽象类
> 关键词 abstract，不包含具体实现，必须在派生类中实现  
```ts
abstract class Animal{
    public abstract eat():void;

    public say(){
        console.log('say sth');
    }; // 可以有普通方法
}

class Cat extends Animal {
    public eat() {
        console.log("eat fish");
    }
}

var c = new Cat;
c.eat();
c.say();
```

## TS 中的接口 
- 属性类接口(有点像struct)  
```ts
interface Person {
    firstName:string; // must
    secondName:string; // must
    age?:number // optional
}

// 必须传入 firstName secondName
function printName(p:Person) {
    console.log(p.firstName, p.secondName)
}

printName({firstName: "z", secondName: "q"});
// OR  

var obj = {
    age: 20,
    firstName: "d",
    secondName: 'Q',
};

printName(obj);
```
- 函数类型接口  
```ts
interface encrypt{
    (key:string, value:string):string; // 必须传入两个string，返回一个string
}

var md5:encrypt = function (key:string, value:string):string {
    return key+value;
}

console.log(md5("ha", "xi"));
```
- 可索引接口（数组，对象的约束）  
```ts
// 数组的约束
interface User{
    [indx:number]:string; // 当 indx 是 number 的时候就是数组
}
var arr:User = ["123", "23"]
```
```ts
// 对象的约束
interface User{
    [index:string]:string;
}
var u:User = {a: "123"}
```
- 类类型接口  
```ts
// 对类的约束，和抽象类有点类似
// 就是把属性和方法接口整合了一下😂
interface Animal {
    name:string;
    eat(str:string):void;
}
class Dog implements Animal {
    name:string;
    constructor(name:string) {
        this.name = name;
    }
    public eat():void {
        console.log("eat fish");
    }
}
```
- 接口扩展，顺便继承  
```ts
interface Animal{
    name:string;
}
interface Cat extends Animal {
    eat():void;
}

class Skin {
    public color:string;

    constructor(color:string) {
        this.color = color;
    }
}

class White extends Skin implements Cat {
    public name:string;

    constructor(name:string, color:string) {
        super(color);
        this.name = name;
    }

    public eat():void {

    }
}
```

## 泛型 
- 泛型变量  
```ts
function getData<T>(value:T):T {
    return value;
}
```

- 泛型类  
```ts
// 泛型类
class MinClass<T> {
    public list:T[] = [];

    public add(num:T) {
        this.list.push(num);
    }

    public min():T {
        return this.list[0];        
    }
}

var m = new MinClass<number>();
m.add(123);
```

- 泛型接口  
第一种  
```ts
interface Conf {
    <T>(val: T): T;
}
var con: Conf = function <T>(val: T): T {
    return val;
}
con<string>("df");
```

第二种  
```ts
interface Config<T> {
    (val1: T): T;
}
var c: Config<string> = function (val1: string): string {
    return val1;
}
```

## 模块
- 概念  
TypeScript 1.5 里术语已经发生了变化  
"内部模块"现在叫“命名空间”；“外部模块”现在简称为“模块”；  
模块在其自身作用域里执行，而不是在全局作用域里；  
这意味着定义在一个模块里的变量，函数，类等模块外部是不可见的，除非你明确的使用 `export` 形式之一导出他们，相反，
如果想使用其他模块导出的变量，函数，类，接口等的时候，就必须先导入他们，使用 `import` 的形式之一  

- 实现1  
文件 `./modules/db.ts`  
```ts
export var url = "xxx";
export function getData():any[] {  return []; }
export function save():boolean { return true; }
```

文件 `index.ts`  
```ts
import {getData, save, url} from './modules/db';

console.log(url);
getData();
save();
```

- 实现2  
文件 `./modules/db.ts`  
```ts
var url = "xxx";
function getData():any[] {  return []; }
function save():boolean { return true; }

export {url, getData, save};
```

文件 `index.ts`  
```ts
// as 关键词
import { getData as get } from './modules/index.ts'
get();
```
- `as` 关键词，见上  
- `default` 关键词  
> 一个模块里面只能用一次  
`db.ts` 中  
```ts
function get() {}
export default get;
```
`index.ts`中  
```ts
import g from './modules/db';
g();
```

## 命名空间 
> 需要 `export` 关键词暴露  
```ts
namespace A {
    export class Cat {
        static say():void { console.log("miao"); }
    }
}
A.Cat.say();
```

## 装饰器
> 通俗的讲，装饰器就是一个方法，可以注入到类，属性，方法，参数上  
> 常见的 类装饰器，属性装饰器，方法装饰器，参数装饰器  
> 装饰器的写法：普通装饰器（无法传参），装饰器工厂（可传参）  

1. 类装饰器  
> 装饰器的执行顺序：属性 > 方法 > 方法参数 > 类  
> 同样类型的装饰器的执行顺序是先 下 后 上 

- 普通装饰器  
```ts
// 普通 类装饰器
// kls 就是 Client 这个class
function logClass(kls:any) {
    console.log(kls.whoami);

    kls.prototype.from = "from logClass";

    kls.prototype.run = function() {
        console.log("method run");
    }
}

@logClass
class Client {
    public static whoami:string = 'i am Client';
    constructor() {}
}

var c:any = new Client();
console.log(c.from)
c.run()
```

- 装饰器工厂；可以传参数  
```ts
// 类装饰器工厂
function logClass(params:string) {
    return function(target:any) {
        console.log(target); // Client 这个类
        console.log(params); // hello  这个字符串
    }
}

@logClass('hello')
class Client {  }
```

- 方法重写  
> 感觉是声明了一个子类，并将 Client 类完全覆盖了  
```ts
// 类装饰器工厂
function logClass(kls:any) {
    kls.prototype.name = "xxxx";

    return class extends kls {
        name:string;
        constructor() {
            console.log("run before");
            super()
            this.name = "xxxx";
        }

        get():void {
            console.log("before ...");
            console.log("new " + this.name);
        }
    }
}

@logClass
class Client { 
    name:string | undefined;

    constructor() {
        this.name = "from constructor";
        console.log("will run after");
    }

    get():void {
        console.log("after ...");
        console.log(this.name);
    }
}

var c = new Client();
c.get();

/*  输出
  run before
  will run after
  before ...
  new xxxx
*/
```

2. 属性装饰器  
```ts
function log(params:any) {
    return function(target:any, attr:any) {
        console.log(target); // 类的原型对象
        console.log(attr);   // 值是 属性的名字 这里是 url
        console.log(params); // 所传参数

        target[attr] = params;
    }
}

class Client {
    @log("hahah")
    public url:any;

    get():void {
        console.log('`from get` ' +  this.url);
    }
}

(new Client).get();
```

3. 方法装饰器  
```ts
function log(params:any) {
    return function(kls:any, methodName:any, desc:any) {
        console.log(kls);   // Client 这个类
        console.log(methodName); // 方法名，这里是 `get`
        console.log(desc); // 一些属性，比如value， 是否可写啥的; value 就是方法自身

        kls.apiURL = 'i am api url';

        var om = desc.value;
        desc.value = function(...args:any[]) {
            console.log("i have changed");
            om.apply(this, args); // args 不是三点参数，会报错
        };
    }
}

class Client {
    @log("xxx")
    get(...args:any[]) {
        console.log(args + " i am original");
        return "x";
    }

    post() {}
}

var c:any = new Client();
console.log(c.apiURL);
c.get('nihao', 'a')

/* 输出
Client { get: [Function], post: [Function] }
get
{
  value: [Function],
  writable: true,
  enumerable: true,
  configurable: true
}
i am api url
i have changed
nihao,a i am original
*/
```

4. 参数装饰器（暂时不知道用途）  
```ts
function log(params:any) {
    return function(kls:any, name:any, index:any) {
        console.log(kls);    // Client { get: [Function] }
        console.log(name);   // mget  方法名
        console.log(index);  // 0     索引
        console.log(params); // en


    }
}

class Client {
    mget(@log('en') uuid:any, p:any) {
        console.log(uuid);
    }
}

var c = new Client();
c.mget("xxx", "yyy");

/* 输出
    Client { mget: [Function] }
    mget
    0
    uuid
    xxx
*/
```
