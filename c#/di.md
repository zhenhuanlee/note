## 依赖注入 (ServiceCollection)
```cs
public static void Main()
{
    Car carxx = new(new BatteryB());

    // 做一个容器的生成类
    ServiceCollection containerBuilder = new();

    // 1.服务的类型
    // 2.服务的实现类型
    // 3.服务的生命周期：singleton, scoped, transient

    // 注册容器里的服务信息
    containerBuilder.AddSingleton(carxx); // A: 手动实例化
    containerBuilder.Add(ServiceDescriptor.Singleton(carxx)); // 和 A 等价
    containerBuilder.AddSingleton<Car>(); // 注册了一个类型为 Car 的单例服务
    containerBuilder.AddSingleton<Car, Car>(); // 委托给 container 实例化的时候，会默认
    ServiceDescriptor serviceDescriptor = new ServiceDescriptor(typeof(Car), carxx); // 等价于 工厂方法 ServiceDescriptor.Singleton(car)
    containerBuilder.Add(serviceDescriptor);


    containerBuilder.AddScoped<IBattery, BatteryA>(); // 注册了一个生命周期为作用域的以 BatteryA 的实例 实现接口 IBattery 的服务
    containerBuilder.AddTransient<IBattery, BatteryB>(); // 这一次会把前面的操作都覆盖, 类型 Car 不会被影响
    containerBuilder.TryAddSingleton<IBattery, BatteryA>(); // 尝试去注册一个服务，但是因为已经有了，所以不会生效,这里还是 BatteryB

    containerBuilder.AddSingleton(provider => new Car(new BatteryA())); // 通过 delegate 重载

    containerBuilder.MakeReadOnly(); // containerBuilder 不能再修改

    // 生成容器
    // 长生命周期的不应该依赖短生命周期的，所以可以加个检查
    // 但是这个很消耗性能，所以应该在生产关掉！！
    var options = new ServiceProviderOptions()
    {
      ValidateScopes = true,
      ValidateOnBuild = true,
    }
    IServiceProvider container = containerBuilder.BuildServiceProvider(options);

    // 通过容器进行服务的调用
    Car? car2 = container.GetService<Car>();
    /*
     * 这里有点特别，因为 Car 类型之前已经手动 add 过实例了，所以这里会使用 carxx 这个对象
     * 但是如果没有，container 会帮我们自动实例化，那 container 会帮我们自动实例化，而且回去找参数最长的构造函数（好像得是类型参数）
     * 而且 IBattery 是个 interface，不能实例化，得自己手动注册一个 Battery 对象，否则会报错
    */
    car2?.Start();
    //Car car3 = container.GetRequiredService<Car>(); // 不可为null，空就报错
    IBattery? ba = container.GetService<IBattery>();
}
```

### 生命周期的依赖校验和作用域的创建
```cs
public static void Main()
{
    // 生命周期，singleton， scoped， transient 由长倒短，长的不应该依赖短的，导致短的无法释放
    // 可以将检查打开
    ServiceCollection containerBuilder = new();
    containerBuilder.AddTransient<Car, Car>();
    containerBuilder.AddScoped<IBattery, BatteryA>();

    IServiceProvider container = containerBuilder.BuildServiceProvider(new ServiceProviderOptions()
    {
        ValidateScopes = true,
        ValidateOnBuild = true,
    });

    // 通过容器调用
    IServiceScopeFactory serviceScopeFactory = container.GetRequiredService<IServiceScopeFactory>();

    // 创建 scope1
    using (var scope1 = serviceScopeFactory.CreateScope())
    {
        var serviceProvider1 = scope1.ServiceProvider;
        Car car1 = serviceProvider1.GetRequiredService<Car>();
    }
    
    // 可以简化成
    using var scope2 = serviceScopeFactory.CreateScope();
    var serviceProvider2 = scope2.ServiceProvider;
    Car car2 = serviceProvider2.GetRequiredService<Car>();
}
```

### 带键的服务
```cs
public static void Main()
{
    // 前面说过了，默认会最后声明的那个
    // 所以可以使用带键的服务
    ServiceCollection containerBuilder = new();
    containerBuilder.AddTransient<Car>();
    containerBuilder.AddKeyedTransient<IBattery, BatteryA>("a");
    containerBuilder.AddKeyedTransient<IBattery, BatteryB>("b");

    var container = containerBuilder.BuildServiceProvider();
    var battery = container.GetKeyedService<IBattery>("a");
    battery?.GetCode();
}
```
