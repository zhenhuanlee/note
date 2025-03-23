### Delegate 函数指针？
```cs
public class Program
{
  private delegate void MyDelegate(); // 最普通的用法，不优雅，很麻烦

  private static MyDelegate? myDelegate1;
  private static MyDelegate? myDelegate2;

  public Action _myAction1; // 这个会被在方法外调用
  public event Action _myAction2; // event 关键词确保了只会在当前勒内部调用 - 也就是说，其他地方可以访问这个变量，并添加修改，但是不能 invoke，所以不能设置为 private

  private static Action? myAction; // 进阶用法，Action 没有返回值，可以有参数
  private static Func<int, String>? myFunc; // Func 有返回值，第一个 int 是参数，String 是返回值类型

  public static void Main(string[] args)
  {
    myDelegate1 = () => Console.WriteLine("hello");

    myDelegate1?.Invoke(); // 判断空
    Say(myDelegate1);

    myDelegate2 += SayHello;
    myDelegate2 += SayWorld;
    myDelegate2 -= SayWorld; // 多方法调用
    Say(myDelegate2);

    myAction = () => Console.WriteLine("hello");
    myAction?.Invoke();

    myFunc = num => $"hello {num}";
    Console.WriteLine(myFunc.Invoke(3));

    Action _action1 = delegate() // 内函数，非最优，看看下面的 lambda
    { ... };

    Func<int, int, int>? _action = (a, b) => a + b; // 简洁写法
  }

  private static void Say(MyDelegate? myDelegate)
  {
    myDelegate?.Invoke();
  }

  public static void SayHello()
  {
    Console.WriteLine("Hello");
  }

  public static void SayWorld()
  {
    Console.WriteLine("World");
  }
```
