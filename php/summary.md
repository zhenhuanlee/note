- 在所有函数外部定义的变量，拥有全局作用域。除了函数外，全局变量可以被脚本中的任何部分访问，要在一个函数中访问一个全局变量，需要使用 global 关键字  
将所有全局变量存储在一个名为 $GLOBALS[index] 的数组中
```php
 <?php
$x=5; // 全局变量

function myTest()
{
    $y=10; // 局部变量
    echo "<p>测试函数内变量:<p>";
    echo "变量 x 为: $x";
    echo "<br>";
    echo "变量 y 为: $y";

    global $x,$y;
    $y=$x+$y;
    // PHP 将所有全局变量存储在一个名为 $GLOBALS[index] 的数组中
    // 上面的 等价于
    $GLOBALS['y']=$GLOBALS['x']+$GLOBALS['y'];
} 

myTest();

echo "<p>测试函数外变量:<p>";
echo "变量 x 为: $x";
echo "<br>";
echo "变量 y 为: $y";
?> 
```

- 当一个函数完成时，它的所有变量通常都会被删除。然而，有时候您希望某个局部变量不要被删除。要做到这一点，请在您第一次声明变量时使用 static 关键字  
```php
<?php
function myTest()
{
    static $x=0;
    echo $x;
    $x++;
    echo PHP_EOL;    // 换行符
}

myTest();
myTest();
myTest();
?>
```
- 松散比较：使用两个等号 == 比较，只比较值，不比较类型  `44 ==  '44' -> true`  
- 严格比较：用三个等号 === 比较，除了比较值，也比较类型 `44 === '44' -> false`  

- PHP 常量  
常量是全局的  
```php
<?php
// 区分大小写的常量名
define("GREETING", "欢迎访问 Runoob.com");
echo GREETING;    // 输出 "欢迎访问 Runoob.com"
echo '<br>';
echo greeting;   // 输出 "greeting"
?>
```

