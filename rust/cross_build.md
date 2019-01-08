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

