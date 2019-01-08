# Glibc uClibc EGLIBC Musl-libc都是些啥
- Glibc (GNU C Library) 是GNU项目所实现的C语言标准库，目前常用的桌面和服务器中的GNU/Linux类的系统中，都是用的这套C语言标准库。其实现了常用的C库函数，支持很多种系统平台，功能很全，但是也相对臃肿和庞大  
- uClibc 一个小型的C语言标准库，主要用于嵌入式，μ指micro  
- EGLIBC glibv的一种变种，目的是将glibc用于嵌入式系统  
- Musl-libc Musl是一个轻量级的C标准库，设计作为glibc, uClibc或Android Bionic的替代作用于嵌入式操作系统和移动设备  