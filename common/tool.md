# 常用的调试工具
- wireshark
- tcpdump
- strace/dtruss: 系统调用跟踪工具
- lsof: 列出文件打开情况
- valgrind: 查内存泄露
- ltrace: 查询库调用
- [NCurses Disk Usage](https://dev.yorhel.nl/ncdu): 查看文件占用
- ngrok: 反射代理，外网访问局域网
### 网络
- nethogs: 按进程查看流量占用  
- iptraf: 按连接/端口查看流量
- ifstat: 按设备查看流量  
- ethtool: 诊断工具  
- tcpdump: 抓包工具  
- ss: 连接查看工具  
- 其他: dstat, slurm, nload, bmon  

# web在线调试
- [jsFiddle](https://jsfiddle.net/)
- [CSSDesk](http://cssdesk.com/)
- [Dabblet](http://dabblet.com/)
- [JS Bin](http://jsbin.com)
- [Tinkerbin](http://tinkerbin.com/)
- [Rendur](http://rendur.com/)

# daemons 
- pm2 -n 'geth' -f start geth -x -- --fast --rinkeby --cache=512 --datadir=/ethereum --rpc --rpcaddr=127.0.0.1 --rpcport=101 --rpcapi "db,eth,net,web3,personal"

