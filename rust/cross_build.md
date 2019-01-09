# 交叉编译
#### 交叉编译到一个树莓派系统示例  
```shell
# 1. Install the C cross toolchain
sudo apt-get intall -qq gcc-arm-linux=gnueabihf
# 2. Install the cross compiled standard crates
rustup target add armv7-unknown-linux-gnueabihf
# 3. Config cargo for cross compilation
mkdir -p ~/.cargo
cat >> ~/.cargo/config <<EOF
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF
# compile
cargo build --target=armv7-unknown-linux-gnueabihf
```

## 术语
- triple: 编译产生的二进制程序不仅可以运行在目标主机上，还可以运行在其他的有相同的架构(e.g. ARM)和操作系统(e.g. Linux)的系统上，triple的格式一般是`{arch}-{vendor}-{sys}-{abi}`，e.g. `arm-unknown-linux-gnueabihf`  
    - architecture: `arm`  
    - vendor: `unknown`. 没有指定`vendor`，说明这个不重要  
    - system: `linux`  
    - ABI: `gnueabihf`. 指明系统使用的是glibc作为C标准库(libc)，具有硬件加速浮点算法(如FPU)  
> 有的triples省略了vendor或者abi，如`x86_64-apple-darwin`  

## 需求
#### 编译一个rust程序需要4个东西
- 确定目标系统的`triple`  
- 一个`gcc`交叉编译器，因为rustc使用gcc去链接`stuff`  
- C依赖，通常是`libc`，为目标系统交叉编译  
- rust依赖，通常是`std`库，为目标系统交叉编译  

##### target triple
如何找出目标系统的triple  
- Architecture: `uname -m`   
- Vendor: Linux-`unknown`, windows-`pc`, OSX/IOS-`apple`  
- System: `uname -s`  
- ABI: 
    - linux: 指的是libc的实现，可以`ldd --version`  
    - Mac & *BSD: 只有一种，所以可以省略  
    - Windows: `gnu` or `MSVC`  

##### C 交叉工具链
`gcc`交叉编译仅针对一个`triple`，这个`triple`会作为所有的工具链命令的前缀，`ar`，`gcc`等。这有助于区分本地编译所使用的工具，如`arm-none-eabi-gcc`。
这里让人困惑的地方是`triples`可能会非常的随意，所以你的C交叉编译器可能会有一个和你的`triple`不一样的前缀，比如：  
    - linux中，arm的交叉编译包是`arm-linux-gnueabihf-gcc`  
    - Exherbo中的t`riple`则是`arm-unknown-linux-gnueabihf`，和其他的都不匹配，但是他们指向的是同一组系统  
确认你的工具链是否正确的最好的方法是交叉编译一个C程序，然后在目标系统上跑一下  

如何获得C交叉编译工具链，这取决于系统。一些linux发行版提供了打包好的交叉编译器，至于其他的话，可能就得自己编译交叉编译器了，[crosstool-ng](https://github.com/crosstool-ng/crosstool-ng)这个工具可以帮上点忙。linux到OSX的话看看这个[osxcross](https://github.com/tpoechtrager/osxcross)  

下面是一下打包好的交叉编译器的例子：  
- `arm-unknown-linux-gnueabi`，ubuntu和Debian提供`gcc-*-arm-linux-gnueabi`包，其中`*`是gcc版本，如`gcc-4.9-arm-linux-gnueabi`  
- `arm-unknown-linux-gnueabihf`，和上面的差不多，只是把`gnueabi`替换成了`gnueabihf`  
- OpenWRT设备，`mips-unknown-linux-uclibc`(15.05 and older)和`mips-unknown-linux-musle`(post 15.05)使用[OpenWRT SDK](https://wiki.openwrt.org/doc/howto/obtain.firmware.sdk)  
- Raspberry Pi，使用[Raspberry tools](https://github.com/raspberrypi/tools/tree/master/arm-bcm2708)  

> C交叉工具链会为目标附带一个交叉编译的libc，需要确认：  
> - 这个工具链libc和目标的libc相匹配。举例：如果目标使用的是musl libc，那么工具链也必须使用musl libc  
> - 工具链的libc的ABI和目标的libc是兼容的。这个通常意味着工具链的libc必须目标的libc老，当然版本一致是最好的。

##### 交叉编译了的Rust crates
大多数的程序都link了`std`库，所以至少需要一个交叉编译了的`std`库去交叉编译目标程序。最简单的方法就是从[official builds](http://static.rust-lang.org/dist/)获取  

`rustup`可以使用`rustuo target add xxx`。

> 如果是nightly，每次更新Rust都需要重新安装cross compiled standard crates  

##### 通过rustc编译

##### 通过cargo编译  
要使用cargo来交叉编译，必须要先设置合适的linker和archiver，一旦设置，只需要传递`--target`flag，(configuration system)[http://doc.crates.io/config.html]  
```toml
# .cargo/config
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
# cargo build --target=arm-unknown-linux-gnueabihf
```

##### 高点


###### 跨系统编译  
找一个跨系统的C交叉编译工具链是比较难的，一个更加简单的方法是试试[Travis CI](https://travis-ci.org/)和[AppVeyor](https://www.appveyor.com/)，来看看作者的[rust-everywhere](https://github.com/japaric-archived/rust-everywhere)  

###### 如果编译纯静态链接
简单的答案：`cargo build --target x86_64-unknown-linux-musl`    
对于target是`*-*-linux-gnu*`的来说，rustc总是会编译出一个动态链接到`glibc`和其他libraries的二进制文件  
rust提供了两个targets`x86_64-unknown-linux-musl`和`i686-unknown-linux-musl`来编译静态链接的二进制，编译出的二进制会静态链接到`MUSL`这个C库  