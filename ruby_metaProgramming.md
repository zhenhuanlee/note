# 对象模型

## 打开类

当多次定义一个类时，只是重新打开这个类，并为之定义y方法

``` ruby
    Class D
      def x; 'x'; end;
    end

    class D
      def y; 'y'; end;
    end
```

鲁莽的修改一个类：猴子补丁
