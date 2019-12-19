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



