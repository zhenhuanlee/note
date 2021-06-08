# 入门
## 类
### 类的定义
```objective-c
  @interface SimpleClass : NSObject
  @end
```

### 类的属性声明
```objective-c
  @property NSString *firstName;
  @property NSString *lastName;
```

### 基本数据类型
- int `int a;`  32位  
- float `float f;`  32位  
- double `double num;`   64位  
- char `char c = 'A'`  1个字节  
- string  
- NSString  `@"hello world"`  
### 限定词  
- long  `long a;` 兼容以前的18位等等低级计算机,long表示32位
- long long 比long更大    
- short 短整型  
- unsigned  无符号  
- signed  有符号  

### goto
```objective-c
int i = 0
a: {
    i++;
    NSLog(@"i's value is %d", i);
}

if(i < 5) goto a;
```
